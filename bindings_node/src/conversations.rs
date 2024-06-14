use std::ops::Deref;
use std::sync::Arc;
use std::vec;

use napi::bindgen_prelude::{Error, Result, Uint8Array};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::JsFunction;
use napi_derive::napi;
use xmtp_mls::groups::GroupMetadataOptions;

use crate::messages::NapiMessage;
use crate::{
  groups::{GroupPermissions, NapiGroup},
  mls_client::RustXmtpClient,
  streams::NapiStreamCloser,
};

#[napi(object)]
pub struct NapiListConversationsOptions {
  pub created_after_ns: Option<i64>,
  pub created_before_ns: Option<i64>,
  pub limit: Option<i64>,
}

#[napi(object)]
pub struct NapiCreateGroupOptions {
  pub permissions: Option<GroupPermissions>,
  pub group_name: Option<String>,
  pub group_image_url_square: Option<String>,
}

impl NapiCreateGroupOptions {
  pub fn into_group_metadata_options(self) -> GroupMetadataOptions {
    GroupMetadataOptions {
      name: self.group_name,
      image_url_square: self.group_image_url_square,
    }
  }
}

#[napi]
pub struct NapiConversations {
  inner_client: Arc<RustXmtpClient>,
}

#[napi]
impl NapiConversations {
  pub fn new(inner_client: Arc<RustXmtpClient>) -> Self {
    Self { inner_client }
  }

  #[napi]
  pub async fn create_group(
    &self,
    account_addresses: Vec<String>,
    options: Option<NapiCreateGroupOptions>,
  ) -> Result<NapiGroup> {
    let options = match options {
      Some(options) => options,
      None => NapiCreateGroupOptions {
        permissions: None,
        group_name: None,
        group_image_url_square: None,
      },
    };

    let group_permissions = options
      .permissions
      .map(|group_permissions| group_permissions.into());

    let convo = self
      .inner_client
      .create_group(group_permissions, options.into_group_metadata_options())
      .map_err(|e| Error::from_reason(format!("ClientError: {}", e)))?;
    if !account_addresses.is_empty() {
      convo
        .add_members(&self.inner_client, account_addresses)
        .await
        .map_err(|e| Error::from_reason(format!("GroupError: {}", e)))?;
    }
    let out = NapiGroup::new(
      self.inner_client.clone(),
      convo.group_id,
      convo.created_at_ns,
    );

    Ok(out)
  }

  #[napi]
  pub async fn process_streamed_welcome_message(
    &self,
    envelope_bytes: Uint8Array,
  ) -> Result<NapiGroup> {
    let envelope_bytes = envelope_bytes.deref().to_vec();
    let group = self
      .inner_client
      .process_streamed_welcome_message(envelope_bytes)
      .await
      .map_err(|e| Error::from_reason(format!("{}", e)))?;
    let out = NapiGroup::new(
      self.inner_client.clone(),
      group.group_id,
      group.created_at_ns,
    );
    Ok(out)
  }

  #[napi]
  pub async fn sync(&self) -> Result<()> {
    self
      .inner_client
      .sync_welcomes()
      .await
      .map_err(|e| Error::from_reason(format!("{}", e)))?;
    Ok(())
  }

  #[napi]
  pub async fn list(&self, opts: Option<NapiListConversationsOptions>) -> Result<Vec<NapiGroup>> {
    let opts = match opts {
      Some(options) => options,
      None => NapiListConversationsOptions {
        created_after_ns: None,
        created_before_ns: None,
        limit: None,
      },
    };
    let convo_list: Vec<NapiGroup> = self
      .inner_client
      .find_groups(
        None,
        opts.created_after_ns,
        opts.created_before_ns,
        opts.limit,
      )
      .map_err(|e| Error::from_reason(format!("{}", e)))?
      .into_iter()
      .map(|group| {
        NapiGroup::new(
          self.inner_client.clone(),
          group.group_id,
          group.created_at_ns,
        )
      })
      .collect();

    Ok(convo_list)
  }

  #[napi(ts_args_type = "callback: (err: null | Error, result: NapiGroup) => void")]
  pub fn stream(&self, callback: JsFunction) -> Result<NapiStreamCloser> {
    let tsfn: ThreadsafeFunction<NapiGroup, ErrorStrategy::CalleeHandled> =
      callback.create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))?;
    let client = self.inner_client.clone();
    let stream_closer = RustXmtpClient::stream_conversations_with_callback(
      client.clone(),
      move |convo| {
        tsfn.call(
          Ok(NapiGroup::new(
            client.clone(),
            convo.group_id,
            convo.created_at_ns,
          )),
          ThreadsafeFunctionCallMode::Blocking,
        );
      },
      || {}, // on_close_callback
    )
    .map_err(|e| Error::from_reason(format!("{}", e)))?;

    Ok(NapiStreamCloser::new(
      stream_closer.close_fn,
      stream_closer.is_closed_atomic,
    ))
  }

  #[napi(ts_args_type = "callback: (err: null | Error, result: NapiMessage) => void")]
  pub fn stream_all_messages(&self, callback: JsFunction) -> Result<NapiStreamCloser> {
    let tsfn: ThreadsafeFunction<NapiMessage, ErrorStrategy::CalleeHandled> =
      callback.create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))?;
    let stream_closer = RustXmtpClient::stream_all_messages_with_callback_sync(
      self.inner_client.clone(),
      move |message| {
        tsfn.call(Ok(message.into()), ThreadsafeFunctionCallMode::Blocking);
      },
    )
    .map_err(|e| Error::from_reason(format!("{}", e)))?;

    Ok(NapiStreamCloser::new(
      stream_closer.close_fn,
      stream_closer.is_closed_atomic,
    ))
  }
}
