// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_iota::did::DID;
use identity_iota::did::Document;

use crate::storage::StorageHandle;

#[derive(Debug)]
pub struct Identity {
  pub(crate) id: DID,
  pub(crate) index: usize,
  pub(crate) name: String,
  pub(crate) created_at: Timestamp,
  pub(crate) updated_at: Timestamp,
  pub(crate) last_sync: Option<Timestamp>,
  pub(crate) persist: bool,
  pub(crate) document: Document, // TODO: Replace with DocumentChain
  pub(crate) storage: StorageHandle,
}

impl Identity {
  pub fn id(&self) -> &DID {
    &self.id
  }

  pub fn index(&self) -> usize {
    self.index
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn created_at(&self) -> Timestamp {
    self.created_at
  }

  pub fn updated_at(&self) -> Timestamp {
    self.updated_at
  }

  pub fn last_sync(&self) -> Option<Timestamp> {
    self.last_sync
  }

  pub fn persist(&self) -> bool {
    self.persist
  }

  pub fn document(&self) -> &Document {
    &self.document
  }

  pub(crate) fn storage(&self) -> &StorageHandle {
    &self.storage
  }
}
