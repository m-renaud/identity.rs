// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;

use crate::error::Result;

#[async_trait::async_trait]
pub trait StorageAdapter: Send + Sync {
  async fn all(&mut self) -> Result<Vec<Vec<u8>>>;

  async fn get(&mut self, resource_id: &[u8]) -> Result<Vec<u8>>;

  async fn set(&mut self, resource_id: &[u8], resource: &[u8]) -> Result<()>;

  async fn del(&mut self, resource_id: &[u8]) -> Result<()>;

  async fn generate_key(&self, type_: KeyType, identity: usize, fragment: &str) -> Result<PublicKey>;
}

#[async_trait::async_trait]
impl StorageAdapter for Box<dyn StorageAdapter> {
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

  async fn generate_key(&self, type_: KeyType, identity: usize, fragment: &str) -> Result<PublicKey> {
    (**self).generate_key(type_, identity, fragment).await
  }
}
