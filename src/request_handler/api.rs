pub mod v1;

use std::sync::Arc;

use anyhow::Result;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::{
    config::Config,
    errors::Errors,
    schemas::{Common, ResponseBase, ResponseBaseArgs, Status},
    session_manager::Session,
};

//TODO: think of better way, than just passing send_steam (some wrapper around deferred function?)
pub async fn map_request(
    config: Arc<Config>,
    session: Arc<Session>,
    request: &[u8],
    send_stream: &mut quinn::SendStream, // for requests, that send additional info, such as Pull
) -> Result<Vec<u8>> {
    let common = flatbuffers::root::<Common>(request)?;

    match common.api_version() {
        1 => Ok(v1::process_request(config, session, request, send_stream).await?),
        _ => Err(Errors::Unsupported.into()),
    }
}

#[inline]
pub fn make_base_response<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    api_version: u8,
    status: Status,
) -> WIPOffset<ResponseBase<'a>> {
    let args = ResponseBaseArgs {
        common: Some(&Common::new(api_version)),
        status,
    };

    ResponseBase::create(builder, &args)
}

#[inline]
pub fn too_long_response<'a>(builder: &mut FlatBufferBuilder<'a>) -> Vec<u8> {
    let response = make_base_response(builder, 0, Status::TooLong);
    builder.finish(response, None);

    builder.finished_data().to_vec()
}
