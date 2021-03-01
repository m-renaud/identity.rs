// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::convert::TryInto;
use crypto::ciphers::chacha::xchacha20poly1305::XCHACHA20POLY1305_NONCE_SIZE;
use crypto::ciphers::chacha::xchacha20poly1305::XCHACHA20POLY1305_TAG_SIZE;
use crypto::ciphers::chacha::xchacha20poly1305::decrypt;
use crypto::ciphers::chacha::xchacha20poly1305::encrypt;
use crypto::rand::fill;
use std::path::Path;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;

use crate::error::Error;
use crate::error::Result;
use crate::storage::StorageAdapter;
use crate::utils::EncryptionKey;

pub struct EncryptedStorage<S> {
  storage_adapter: S,
  encryption_key: Option<EncryptionKey>,
}

impl<S> EncryptedStorage<S> {
  pub(crate) fn new(
    storage_adapter: S,
    encryption_key: Option<EncryptionKey>,
  ) -> Self {
    Self {
      storage_adapter,
      encryption_key,
    }
  }
}

impl<S> Debug for EncryptedStorage<S> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str("EncryptedStorage { .. }")
  }
}

#[async_trait::async_trait]
impl<S> StorageAdapter for EncryptedStorage<S>
where
  S: StorageAdapter,
{
  async fn all(&mut self) -> Result<Vec<Vec<u8>>> {
    todo!("EncryptedStorage::all")
  }

  async fn get(&mut self, resource_id: &[u8]) -> Result<Vec<u8>> {
    if let Some(ref key) = self.encryption_key {
      decrypt_resource(&self.storage_adapter.get(resource_id).await?, key)
    } else {
      Ok(self.storage_adapter.get(resource_id).await?)
    }
  }

  async fn set(&mut self, resource_id: &[u8], resource: &[u8]) -> Result<()> {
    if let Some(key) = self.encryption_key {
      self.set(resource_id, &encrypt_resource(resource, &key)?).await
    } else {
      self.set(resource_id, resource).await
    }
  }

  async fn del(&mut self, resource_id: &[u8]) -> Result<()> {
    self.storage_adapter.del(resource_id).await
  }

  async fn generate_public_key(&self, type_: KeyType, identity: usize, fragment: &str) -> Result<PublicKey> {
    todo!("EncryptedStorage::generate_public_key")
  }

  fn storage_path(&self) -> &Path {
    self.storage_adapter.storage_path()
  }
}

fn encrypt_resource(resource: &[u8], encryption_key: &EncryptionKey) -> Result<Vec<u8>> {
  let mut nonce: [u8; XCHACHA20POLY1305_NONCE_SIZE] = [0; XCHACHA20POLY1305_NONCE_SIZE];
  let mut tag: [u8; XCHACHA20POLY1305_TAG_SIZE] = [0; XCHACHA20POLY1305_TAG_SIZE];
  let mut ciphertext: Vec<u8> = vec![0; resource.len()];

  fill(&mut nonce)?;
  encrypt(&mut ciphertext, &mut tag, resource, encryption_key, &nonce, &[])?;

  let capacity: usize = XCHACHA20POLY1305_NONCE_SIZE + XCHACHA20POLY1305_TAG_SIZE + ciphertext.len();
  let mut output: Vec<u8> = Vec::with_capacity(capacity);

  output.extend_from_slice(&nonce);
  output.extend_from_slice(&tag);
  output.extend_from_slice(&ciphertext);

  Ok(output)
}

fn decrypt_resource(resource: &[u8], encryption_key: &EncryptionKey) -> Result<Vec<u8>> {
  let nonce: &[u8; XCHACHA20POLY1305_NONCE_SIZE] = resource
    .get(..XCHACHA20POLY1305_NONCE_SIZE)
    .and_then(|value| value.try_into().ok())
    .ok_or(Error::InvalidResourceNonce)?;

  let tag: &[u8; XCHACHA20POLY1305_TAG_SIZE] = resource
    .get(XCHACHA20POLY1305_NONCE_SIZE..XCHACHA20POLY1305_TAG_SIZE)
    .and_then(|value| value.try_into().ok())
    .ok_or(Error::InvalidResourceTag)?;

  let ciphertext: &[u8] = resource
    .get(XCHACHA20POLY1305_NONCE_SIZE + XCHACHA20POLY1305_TAG_SIZE..)
    .ok_or(Error::InvalidResourceCiphertext)?;

  let mut plaintext: Vec<u8> = vec![0; ciphertext.len()];

  decrypt(&mut plaintext, &ciphertext, encryption_key, tag, nonce, &[])?;

  Ok(plaintext)
}
