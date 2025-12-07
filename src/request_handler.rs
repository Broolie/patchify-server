mod api;

use std::sync::Arc;

use anyhow::{Result, anyhow};

use crate::{Session, config::Config, request_handler::api::too_long_response};

pub async fn handle_incoming_request(
    config: Arc<Config>,
    session: Arc<Session>,
    (mut send, mut recv): (quinn::SendStream, quinn::RecvStream),
) -> Result<()> {
    let result = recv.read_to_end(u16::MAX.into()).await;

    let request = match result {
        Ok(data) => data,
        Err(quinn::ReadToEndError::TooLong) => {
            send.write(too_long_response(&mut flatbuffers::FlatBufferBuilder::new()).as_slice())
                .await?;
            return Ok(());
        }
        Err(e) => return Err(anyhow!("failed to receive request: {e}")),
    };

    let response = api::map_request(config, session, &request, &mut send).await?;

    if !response.is_empty() {
        send.write_all(&response)
            .await
            .map_err(|e| anyhow!("failed to send response: {e}"))?;
    }

    Ok(())
}
