// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_core::common::Timestamp;
use identity_core::convert::ToJson;
use identity_iota::did::DID;
use identity_iota::did::Document;

use crate::error::Result;
use crate::storage::StorageVault;

#[derive(Clone, Debug)]
pub struct Identity {
  pub(crate) id: DID,
  pub(crate) index: usize,
  pub(crate) name: String,
  pub(crate) created_at: Timestamp,
  pub(crate) updated_at: Timestamp,
  pub(crate) last_sync: Option<Timestamp>,
  pub(crate) persist: bool,
  pub(crate) document: Document, // TODO: Replace with DocumentChain
  pub(crate) vault: StorageVault,
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

  pub(crate) fn vault(&self) -> &StorageVault {
    &self.vault
  }

  fn __ser(&self) -> IdentitySer<'_> {
    IdentitySer {
      id: &self.id,
      index: self.index,
      name: &self.name,
      created_at: self.created_at,
      updated_at: self.updated_at,
      last_sync: self.last_sync,
      document: &self.document,
    }
  }

  pub(crate) fn load(data: &[u8], vault: StorageVault) -> Result<Self> {
    let data: IdentityDe = IdentityDe::from_json_slice(data)?;

    Ok(Self {
      id: data.id,
      index: data.index,
      name: data.name,
      created_at: data.created_at,
      updated_at: data.updated_at,
      last_sync: data.last_sync,
      persist: true,
      document: data.document,
      vault,
    })
  }

  pub(crate) async fn flush(&self) -> Result<()> {
    if self.persist {
      let data: Vec<u8> = self.__ser().to_json_vec()?;
      let name: &[u8] = self.id.as_str().as_bytes();

      self.vault.set(name, &data).await?;
    }

    Ok(())
  }
}

// =============================================================================
// =============================================================================

#[derive(Serialize)]
struct IdentitySer<'a> {
  id: &'a DID,
  index: usize,
  name: &'a str,
  created_at: Timestamp,
  updated_at: Timestamp,
  last_sync: Option<Timestamp>,
  document: &'a Document,
}

#[derive(Deserialize)]
struct IdentityDe {
  id: DID,
  index: usize,
  name: String,
  created_at: Timestamp,
  updated_at: Timestamp,
  last_sync: Option<Timestamp>,
  document: Document,
}
