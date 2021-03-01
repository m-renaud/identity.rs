// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use futures::future::join_all;
use futures::FutureExt;

use crate::archive::ArchiveBuilder;
use crate::error::Error;
use crate::error::Result;
use crate::identity::IdentityHandle;
use crate::identity::IdentityBuilder;
use crate::utils::GenericCache;
use crate::storage::StorageVault;
use crate::identity::Identity;

#[derive(Debug)]
pub struct Archive {
  // client: Arc<Client>,
  pub(crate) vault: StorageVault,

  pub(crate) encrypted_identities: Vec<String>,
  pub(crate) decrypted_identities: GenericCache<IdentityHandle>,
  // encrypted_credentials: Vec<String>,
  // decrypted_credentials: GenericCache<CredentialHandle>,
  // encrypted_presentations: Vec<String>,
  // decrypted_presentations: GenericCache<PresentationHandle>,
}

impl Archive {
  pub fn builder() -> ArchiveBuilder {
    ArchiveBuilder::new()
  }

  pub fn new(vault: StorageVault) -> Self {
    Self {
      vault,
      encrypted_identities: Vec::new(),
      decrypted_identities: GenericCache::new(),
    }
  }

  pub fn create_identity(&self) -> Result<IdentityBuilder> {
    self.assert_encryption()?;

    let cache: GenericCache<IdentityHandle> = self.decrypted_identities.clone();
    let vault: StorageVault = self.vault.clone();

    Ok(IdentityBuilder::new(cache, vault))
  }

  pub async fn identities(&self) -> Result<Vec<IdentityHandle>> {
    self.assert_encryption()?;

    let guard: _ = self.decrypted_identities.read().await;

    let futures: _ = guard
      .values()
      .map(|identity| identity.index().map(move |index| (index, identity)));

    let mut output: Vec<(usize, &IdentityHandle)> = join_all(futures).await;

    output.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(output.into_iter().map(|(_, identity)| identity.clone()).collect())
  }

  pub(crate) async fn initialize(&self) -> Result<()> {
    let identities: Vec<Vec<u8>> = self.vault.all().await?;
    let mut cache: _ = self.decrypted_identities.write().await;

    for data in identities {
      let vault: StorageVault = self.vault.clone();
      let identity: Identity = Identity::load(&data, vault)?;
      let identity_id: String = identity.id.to_string();

      cache.insert(identity_id, IdentityHandle::new(identity));
    }

    Ok(())
  }

  fn assert_encryption(&self) -> Result<()> {
    if !self.encrypted_identities.is_empty() {
      return Err(Error::PartialStorageEncryption);
    }

    // if !self.encrypted_credentials.is_empty() {
    //   return Err(Error::PartialStorageEncryption);
    // }

    // if !self.encrypted_presentations.is_empty() {
    //   return Err(Error::PartialStorageEncryption);
    // }

    Ok(())
  }
}
