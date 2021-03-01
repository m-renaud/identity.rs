// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;
use identity_core::common::Timestamp;
use identity_iota::did::DID;
use identity_iota::did::Document;

use crate::identity::Identity;
use crate::storage::StorageVault;

#[derive(Clone, Debug)]
pub struct IdentityHandle {
  data: Arc<RwLock<Identity>>,
}

impl IdentityHandle {
  pub fn new(data: Identity) -> Self {
    Self {
      data: Arc::new(RwLock::new(data)),
    }
  }

  pub async fn into_inner(&self) -> Identity {
    self.data.read().await.clone()
  }

  pub async fn id(&self) -> DID {
    self.data.read().await.id.clone()
  }

  pub async fn index(&self) -> usize {
    self.data.read().await.index
  }

  pub async fn name(&self) -> String {
    self.data.read().await.name.clone()
  }

  pub async fn created_at(&self) -> Timestamp {
    self.data.read().await.created_at
  }

  pub async fn updated_at(&self) -> Timestamp {
    self.data.read().await.updated_at
  }

  pub async fn last_sync(&self) -> Option<Timestamp> {
    self.data.read().await.last_sync
  }

  pub async fn persist(&self) -> bool {
    self.data.read().await.persist
  }

  pub async fn document(&self) -> Document {
    self.data.read().await.document.clone()
  }

  pub(crate) async fn vault(&self) -> StorageVault {
    self.data.read().await.vault.clone()
  }

  pub(crate) async fn read(&self) -> RwLockReadGuard<'_, Identity> {
    self.data.read().await
  }

  pub(crate) async fn write(&self) -> RwLockWriteGuard<'_, Identity> {
    self.data.write().await
  }
}
