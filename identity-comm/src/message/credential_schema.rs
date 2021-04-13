use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialSchemaRequest {
  context: String,
  thread: String,
  callback_url: Url,
  credential_types: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialSchemaRequest {
  pub fn new(context: String, thread: String, callback_url: Url, credential_types: Vec<String>) -> Self {
    Self {
      context,
      thread,
      callback_url,
      credential_types,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the credential schema request's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the credential schema request's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the credential schema request's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the credential schema request's thread.
  pub fn thread_mut(&mut self) -> &mut String {
    &mut self.thread
  }

  /// Get a reference to the credential schema request's thread.
  pub fn thread(&self) -> &String {
    &self.thread
  }

  /// Set the credential schema request's thread.
  pub fn set_thread(&mut self, thread: String) {
    self.thread = thread;
  }

  /// Get a mutable reference to the credential schema request's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Get a reference to the credential schema request's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Set the credential schema request's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the credential schema request's credential types.
  pub fn credential_types_mut(&mut self) -> &mut Vec<String> {
    &mut self.credential_types
  }

  /// Get a reference to the credential schema request's credential types.
  pub fn credential_types(&self) -> &Vec<String> {
    &self.credential_types
  }

  /// Set the credential schema request's credential types.
  pub fn set_credential_types(&mut self, credential_types: Vec<String>) {
    self.credential_types = credential_types;
  }

  /// Get a mutable reference to the credential schema request's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the credential schema request's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the credential schema request's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the credential schema request's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the credential schema request's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the credential schema request's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the credential schema request's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the credential schema request's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the credential schema request's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialSchemaResponse {
  context: String,
  thread: String,
  schemata: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialSchemaResponse {
  pub fn new(context: String, thread: String, schemata: Vec<String>, callback_url: Option<Url>) -> Self {
    Self {
      context,
      thread,
      schemata,
      callback_url,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the credential schema response's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the credential schema response's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the credential schema response's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the credential schema response's thread.
  pub fn thread_mut(&mut self) -> &mut String {
    &mut self.thread
  }

  /// Get a reference to the credential schema response's thread.
  pub fn thread(&self) -> &String {
    &self.thread
  }

  /// Set the credential schema response's thread.
  pub fn set_thread(&mut self, thread: String) {
    self.thread = thread;
  }

  /// Get a mutable reference to the credential schema response's schemata.
  pub fn schemata_mut(&mut self) -> &mut Vec<String> {
    &mut self.schemata
  }

  /// Get a reference to the credential schema response's schemata.
  pub fn schemata(&self) -> &Vec<String> {
    &self.schemata
  }

  /// Set the credential schema response's schemata.
  pub fn set_schemata(&mut self, schemata: Vec<String>) {
    self.schemata = schemata;
  }

  /// Get a mutable reference to the credential schema response's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Option<Url> {
    &mut self.callback_url
  }

  /// Get a reference to the credential schema response's callback url.
  pub fn callback_url(&self) -> &Option<Url> {
    &self.callback_url
  }

  /// Set the credential schema response's callback url.
  pub fn set_callback_url(&mut self, callback_url: Option<Url>) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the credential schema response's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the credential schema response's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the credential schema response's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the credential schema response's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the credential schema response's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the credential schema response's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the credential schema response's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the credential schema response's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the credential schema response's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}