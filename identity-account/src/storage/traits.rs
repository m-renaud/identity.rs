// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use std::path::Path;

use crate::error::Result;
use crate::utils::fs;

#[async_trait::async_trait]
pub trait StorageAdapter: Send + Sync {
  async fn all(&mut self) -> Result<Vec<Vec<u8>>>;

  async fn get(&mut self, resource_id: &[u8]) -> Result<Vec<u8>>;

  async fn set(&mut self, resource_id: &[u8], resource: &[u8]) -> Result<()>;

  async fn del(&mut self, resource_id: &[u8]) -> Result<()>;

  // TODO: Move to VaultAdapter trait
  async fn generate_public_key(&self, type_: KeyType, identity: usize, fragment: &str) -> Result<PublicKey>;

  fn storage_path(&self) -> &Path;

  fn storage_root(&self) -> &Path {
    let path: &Path = self.storage_path();

    if fs::maybe_file(path) {
      path.parent().unwrap_or(path)
    } else {
      path
    }
  }
}

macro_rules! impl_storage_deref {
  ($trait:ident) => {
    #[async_trait::async_trait]
    impl StorageAdapter for Box<dyn $trait> {
      async fn all(&mut self) -> Result<Vec<Vec<u8>>> {
        (**self).all().await
      }

      async fn get(&mut self, resource_id: &[u8]) -> Result<Vec<u8>> {
        (**self).get(resource_id).await
      }

      async fn set(&mut self, resource_id: &[u8], resource: &[u8]) -> Result<()> {
        (**self).set(resource_id, resource).await
      }

      async fn del(&mut self, resource_id: &[u8]) -> Result<()> {
        (**self).del(resource_id).await
      }

      async fn generate_public_key(&self, type_: KeyType, identity: usize, fragment: &str) -> Result<PublicKey> {
        (**self).generate_public_key(type_, identity, fragment).await
      }

      fn storage_path(&self) -> &Path {
        (**self).storage_path()
      }
    }
  };
}

impl_storage_deref!(StorageAdapter);
impl_storage_deref!(VaultAdapter);

use crate::storage::StrongholdAdapter;
use crate::storage::EncryptedStorage;

mod private {
  pub trait Sealed {}
}

impl private::Sealed for StrongholdAdapter {}

impl<S> private::Sealed for EncryptedStorage<S> where S: StorageAdapter {}

impl private::Sealed for Box<dyn VaultAdapter> {}

pub trait VaultAdapter: private::Sealed + StorageAdapter {}

impl VaultAdapter for StrongholdAdapter {}

impl<S> VaultAdapter for EncryptedStorage<S> where S: StorageAdapter {}

impl VaultAdapter for Box<dyn VaultAdapter> {}
