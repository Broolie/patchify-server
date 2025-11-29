#[allow(unused)]
#[allow(clippy::all)]
#[allow(mismatched_lifetime_syntaxes)]
#[path = "schemas/patchify/common_generated.rs"]
mod common_generated;

mod constants;
mod errors;
mod schemas;
mod session_manager;

mod cli;
mod config;
mod request_handler;

use std::{net::SocketAddr, process::exit, str::FromStr, sync::Arc, thread::available_parallelism};

use anyhow::Result;
use clap::Parser;
use mimalloc::MiMalloc;
use quinn::crypto::rustls::QuicServerConfig;
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::{
    cli::Cli,
    config::Config,
    request_handler::handle_request,
    session_manager::{Session, SessionManager},
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::builder().finish())
        .unwrap();

    //TODO: integrate Cli into Config so that Config after construction will represent unmodifiable final app configuration
    let cli = Cli::parse();

    let config = match cli.config.as_deref() {
        Some(config_path) => Config::new(config_path)
            .inspect_err(|err| {
                error!("Unable to parse config: {err}");
                exit(1);
            })
            .unwrap(),
        None => Config::default(),
    };
    let config = Arc::new(config);
    let manager = Arc::new(Mutex::new(SessionManager::new()));

    match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(available_parallelism().unwrap().get())
        .build()
        .unwrap()
        .block_on(start_server(config.clone(), manager))
    {
        Ok(_) => (),
        Err(err) => error!("Unable to start server: {err}"),
    }
}

async fn start_server(config: Arc<Config>, manager: Arc<Mutex<SessionManager>>) -> Result<()> {
    let (key, cert) = {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        (
            PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der()).into(),
            vec![CertificateDer::from(cert.cert)],
        )
    };

    let mut crypto = match rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, key)
    {
        Ok(crypto) => crypto,
        Err(err) => {
            return Err(err.into());
        }
    };

    crypto.alpn_protocols = vec![b"hq-29".to_vec()];

    let mut server_config =
        quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(crypto)?));

    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    let endpoint =
        quinn::Endpoint::server(server_config, SocketAddr::from_str(&config.bind_address)?)?;

    println!("listening on {}", endpoint.local_addr().unwrap());

    while let Some(conn) = endpoint.accept().await {
        if !conn.remote_address_validated() {
            info!("requiring connection to validate its address");
            conn.retry().unwrap();
        } else {
            info!("accepting connection");
            let fut = handle_connection(config.clone(), manager.clone(), conn);
            tokio::spawn(async move {
                if let Err(e) = fut.await {
                    error!("connection failed: {reason}", reason = e.to_string())
                }
            });
        }
    }

    Ok(())
}

async fn handle_connection(
    config: Arc<Config>,
    manager_lock: Arc<Mutex<SessionManager>>,
    conn: quinn::Incoming,
) -> Result<()> {
    let connection = conn.await?;

    let mut manager = manager_lock.lock().await;
    let session = manager.create_session(connection.stable_id())?;

    loop {
        let stream = match connection.accept_bi().await {
            Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                info!("connection closed");
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
            Ok(s) => s,
        };

        let fut = handle_request(config.clone(), session.clone(), stream);
        tokio::spawn(async move {
            if let Err(e) = fut.await {
                error!("failed: {}", e.to_string());
            }
        });
    }
}
