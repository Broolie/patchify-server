use std::sync::Arc;

use anyhow::Result;
use flatbuffers::FlatBufferBuilder;
use quinn::SendStream;

use crate::{
    Session,
    config::Config,
    errors::Errors,
    request_handler::api::make_base_response,
    schemas::{
        AuthType, Status,
        v1::{
            AuthRequest, FetchAllRequest, FetchRequest, NeedAuthRequest, NeedAuthResponse,
            NeedAuthResponseArgs, PullRequest, Request, Response, RootRequest, RootResponse,
            RootResponseArgs, StatusRequest, VersionRequest, finish_root_response_buffer,
        },
    },
};

pub const API_VERSION: u8 = 1;

pub async fn process_request(
    config: Arc<Config>,
    session: Arc<Session>,
    request: &[u8],
    send_stream: &mut quinn::SendStream,
) -> Result<Vec<u8>> {
    let request = flatbuffers::root::<RootRequest>(request)?;
    let response = match request.request_type() {
        Request::AuthRequest => {
            handle_auth(
                config,
                session,
                request
                    .request_as_auth_request()
                    .expect("AuthRequest unwrap"),
            )
            .await?
        }
        Request::FetchRequest => {
            handle_fetch(
                config,
                session,
                request
                    .request_as_fetch_request()
                    .expect("FetchRequest unwrap"),
            )
            .await?
        }
        Request::FetchAllRequest => {
            handle_fetch_all(
                config,
                session,
                request
                    .request_as_fetch_all_request()
                    .expect("FetchAllRequest unwrap"),
            )
            .await?
        }
        Request::NeedAuthRequest => {
            handle_need_auth(
                config,
                session,
                request
                    .request_as_need_auth_request()
                    .expect("NeedAuthRequest unwrap"),
            )
            .await?
        }
        Request::PullRequest => {
            handle_pull(
                config,
                session,
                request
                    .request_as_pull_request()
                    .expect("PullRequest unwrap"),
                send_stream,
            )
            .await?
        }
        Request::StatusRequest => {
            handle_status(
                config,
                session,
                request
                    .request_as_status_request()
                    .expect("StatusRequest unwrap"),
            )
            .await?
        }
        Request::VersionRequest => {
            handle_version(
                config,
                session,
                request
                    .request_as_version_request()
                    .expect("VersionRequest unwrap"),
            )
            .await?
        }
        _ => return Err(Errors::InvalidRequest.into()),
    };

    Ok(response)
}

async fn handle_auth<'a>(
    config: Arc<Config>,
    session: Arc<Session>,
    request: AuthRequest<'a>,
) -> Result<Vec<u8>> {
    todo!()
}

async fn handle_fetch<'a>(
    config: Arc<Config>,
    session: Arc<Session>,
    request: FetchRequest<'a>,
) -> Result<Vec<u8>> {
    todo!()
}

async fn handle_fetch_all<'a>(
    config: Arc<Config>,
    session: Arc<Session>,
    request: FetchAllRequest<'a>,
) -> Result<Vec<u8>> {
    todo!()
}

async fn handle_need_auth<'a>(
    config: Arc<Config>,
    session: Arc<Session>,
    request: NeedAuthRequest<'a>,
) -> Result<Vec<u8>> {
    let mut builder = FlatBufferBuilder::new();
    let mut condense = AuthType::empty();
    for flag in config.available_auth_types.iter() {
        condense |= *flag; //TODO: make it sound
    }

    let need_auth_args = NeedAuthResponseArgs {
        auth_type: condense,
    };

    let need_auth_wip = NeedAuthResponse::create(&mut builder, &need_auth_args);

    let args = RootResponseArgs {
        base: Some(make_base_response(&mut builder, API_VERSION, Status::Ok)),
        response_type: Response::NeedAuthResponse,
        response: Some(need_auth_wip.as_union_value()),
    };

    let response_wip = RootResponse::create(&mut builder, &args);
    finish_root_response_buffer(&mut builder, response_wip);
    let response = builder.finished_data();

    Ok(response.to_vec())
}

async fn handle_pull<'a>(
    config: Arc<Config>,
    session: Arc<Session>,
    request: PullRequest<'a>,
    send: &mut SendStream,
) -> Result<Vec<u8>> {
    todo!()
}

async fn handle_status<'a>(
    config: Arc<Config>,
    session: Arc<Session>,
    request: StatusRequest<'a>,
) -> Result<Vec<u8>> {
    todo!()
}

async fn handle_version<'a>(
    config: Arc<Config>,
    session: Arc<Session>,
    request: VersionRequest<'a>,
) -> Result<Vec<u8>> {
    todo!()
}
