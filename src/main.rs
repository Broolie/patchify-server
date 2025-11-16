mod cli;
mod config;

use std::{net::SocketAddr, process::exit, str::FromStr, sync::Arc, thread::available_parallelism};

use anyhow::{Result, anyhow};
use clap::{Parser};
use mimalloc::MiMalloc;
use quinn::{crypto::rustls::QuicServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use tracing::{error, info};

use crate::{cli::Cli, config::Config};

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

    match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(available_parallelism().unwrap().get())
        .build()
        .unwrap()
        .block_on(start_server(config))
    {
        Ok(_) => (),
        Err(err) => error!("Unable to start server: {err}"),
    }
}

async fn start_server(config: Config) -> Result<()> {
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

    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(
        QuicServerConfig::try_from(crypto).expect("Unexpected error while creating server config"),
    ));

    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    let endpoint = quinn::Endpoint::server(
        server_config,
        SocketAddr::from_str(&config.bind_address)?)?;

    eprintln!("listening on {}", endpoint.local_addr().unwrap());

    while let Some(conn) = endpoint.accept().await {
        if !conn.remote_address_validated() {
            info!("requiring connection to validate its address");
            conn.retry().unwrap();
        } else {
            info!("accepting connection");
            let fut = handle_connection(conn);
            tokio::spawn(async move {
                if let Err(e) = fut.await {
                    error!("connection failed: {reason}", reason = e.to_string())
                }
            });
        }
    }

    Ok(())
}

async fn handle_connection(conn: quinn::Incoming) -> Result<()> {
    let connection = conn.await?;

    loop {
        let stream = connection.accept_bi().await;
        let stream = match stream {
            Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                info!("connection closed");
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
            Ok(s) => s,
        };
        let fut = handle_request(stream);
        tokio::spawn(async move {
            if let Err(e) = fut.await {
                error!("failed: {reason}", reason = e.to_string());
            }
        });
    }
}

async fn handle_request((mut send, mut recv): (quinn::SendStream, quinn::RecvStream)) -> Result<()> {
    let req = recv
        .read_to_end(64 * 1024)
        .await
        .map_err(|e| anyhow!("failed reading request: {}", e))?;
    
    todo!("Handle request");

    // send.write_all(&resp)
    //     .await
    //     .map_err(|e| anyhow!("failed to send response: {}", e))?;
    // // Gracefully terminate the stream
    // send.finish().unwrap();
    // info!("complete");
    // Ok(())
}
