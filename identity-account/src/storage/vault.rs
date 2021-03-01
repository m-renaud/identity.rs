// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use core::ops::DerefMut;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use identity_core::crypto::PublicKey;
use identity_core::crypto::KeyType;

use crate::error::Result;
use crate::storage::VaultAdapter;
use crate::utils::EncryptionKey;

// =============================================================================
// =============================================================================

#[derive(Clone)]
pub struct StorageVault {
  data: Arc<Mutex<dyn VaultAdapter>>,
}

impl StorageVault {
  pub(crate) fn new(storage: Box<dyn VaultAdapter>) -> Self {
    Self {
      data: Arc::new(Mutex::new(storage)),
    }
  }

  pub(crate) async fn lock(&self) -> MutexGuard<'_, dyn VaultAdapter> {
    self.data.lock().await
  }

  pub(crate) async fn all(&self) -> Result<Vec<Vec<u8>>> {
    self.data.lock().await.all().await
  }

  pub(crate) async fn get(&self, resource_id: &[u8]) -> Result<Vec<u8>> {
    self.data.lock().await.get(resource_id).await
  }

  pub(crate) async fn set(&self, resource_id: &[u8], resource: &[u8]) -> Result<()> {
    self.data.lock().await.set(resource_id, resource).await
  }

  pub(crate) async fn del(&self, resource_id: &[u8]) -> Result<()> {
    self.data.lock().await.del(resource_id).await
  }

  pub(crate) async fn generate_public_key(
    &self,
    type_: KeyType,
    identity: usize,
    fragment: &str,
  ) -> Result<PublicKey> {
    self
      .data
      .lock()
      .await
      .generate_public_key(type_, identity, fragment)
      .await
  }
}

impl Debug for StorageVault {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("StorageVault")
  }
}
