// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use identity_core::crypto::KeyPair;
use identity_iota::did::Document;
use identity_iota::did::DID;
use identity_iota::did::Method;

use crate::error::Error;
use crate::error::Result;
use crate::identity::Identity;
use crate::identity::IdentityHandle;
use crate::utils::GenericCache;
use crate::utils::GenericReadGuard;
use crate::storage::StorageVault;

#[derive(Debug)]
pub struct IdentityBuilder {
  cache: GenericCache<IdentityHandle>,
  vault: StorageVault,
  name: Option<String>,
  key_type: KeyType,
  persist: bool,
}

impl IdentityBuilder {
  pub fn new(
    cache: GenericCache<IdentityHandle>,
    vault: StorageVault,
  ) -> Self {
    Self {
      cache,
      vault,
      name: None,
      key_type: KeyType::Ed25519,
      persist: true,
    }
  }

  pub fn name<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.name = Some(value.into());
    self
  }

  pub fn key_type(mut self, value: KeyType) -> Self {
    self.key_type = value;
    self
  }

  pub fn persist(mut self, value: bool) -> Self {
    self.persist = value;
    self
  }

  pub async fn build(self) -> Result<IdentityHandle> {
    let cache: GenericReadGuard<IdentityHandle> = self.cache.read().await;

    let mut index: usize = 0;
    for identity in cache.values() {
      let guard: _ = identity.read().await;

      if guard.persist() && guard.index() >= index {
        index = guard.index() + 1;
      }
    }

    let name: String = self.name.unwrap_or_else(|| format!("Identity {}", index));

    for identity in cache.values() {
      if identity.name().await == name {
        return Err(Error::IdentityDuplicateName);
      }
    }

    let document: Document = 'outer: loop {
      let public: PublicKey = self
        .vault
        .generate_public_key(self.key_type, index, "authentication")
        .await?;

      // Generate a new DID URL from the public key
      //
      // TODO: Allow creating DIDs from KeyType/PublicKey
      let keypair: KeyPair = (self.key_type, public, Vec::new().into()).into();
      let did: DID = DID::new(keypair.public().as_ref())?;

      // Ensure we didn't generate a duplicate DID
      for identity in cache.values() {
        if identity.document().await.id().tag() == did.tag() {
          continue 'outer;
        }
      }

      let method: Method = Method::from_did(did, &keypair, "authentication")?;

      // SAFETY: We just created a valid authentication method.
      break unsafe {
        Document::from_authentication_unchecked(method)
      };
    };

    let identity: Identity = Identity {
      id: document.id().clone(),
      index,
      name,
      created_at: Timestamp::now(),
      updated_at: Timestamp::now(),
      last_sync: None,
      persist: self.persist,
      vault: self.vault.clone(),
      document,
    };

    if identity.persist {
      // Flush the Identity state to disk
      identity.flush().await?;

      // Convert to a thread-safe handle; keep an owned copy of the id
      let id: String = identity.id.to_string();
      let handle: IdentityHandle = IdentityHandle::new(identity);

      // Drop the read-only cache guard
      drop(cache);

      // Add the new Identity to the cache
      self.cache.write().await.insert(id, handle.clone());

      // TODO: Publish to Tangle

      Ok(handle)
    } else {
      Ok(IdentityHandle::new(identity))
    }
  }
}
