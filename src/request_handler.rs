use std::sync::Arc;

use anyhow::{Result, anyhow};
use flatbuffers::FlatBufferBuilder;
use quinn::SendStream;

use crate::{
    Session,
    common_generated::{AuthType, Common},
    config::Config,
    constants::API_VERSION,
    errors::Errors,
    schemas::patchify::{
        AuthRequest, CommonRequest, CommonResponse, CommonResponseArgs,
        FetchAllRequest, FetchRequest, NeedAuthRequest,
        NeedAuthResponse, NeedAuthResponseArgs, PullRequest, Request, Response, Status, StatusRequest, VersionRequest, finish_common_response_buffer,
    },
};

pub async fn handle_request(
    config: Arc<Config>,
    session: Arc<Session>,
    (mut send, mut recv): (quinn::SendStream, quinn::RecvStream),
) -> Result<()> {
    let request = recv
        .read_to_end(64 * 1024)
        .await
        .map_err(|e| anyhow!("failed reading request: {e}"))?;

    if request.len() > u16::MAX as usize {
        return Err(Errors::RequestTooLong.into());
    }

    let request = flatbuffers::root::<CommonRequest>(&request)?;

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
                &mut send,
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

    if !response.is_empty() {
        send.write_all(&response)
            .await
            .map_err(|e| anyhow!("failed to send response: {e}"))?;
    }

    Ok(())
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

    let args = CommonResponseArgs {
        common: Some(&Common::new(API_VERSION)),
        status: Status::Ok,
        response_type: Response::NeedAuthResponse,
        response: Some(need_auth_wip.as_union_value()),
    };

    let response_wip = CommonResponse::create(&mut builder, &args);
    finish_common_response_buffer(&mut builder, response_wip);
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
