// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;

use crate::archive::Archive;
use crate::archive::ArchiveStorage;
use crate::error::Error;
use crate::error::Result;
use crate::storage::StorageVault;
use crate::storage::StorageAdapter;
use crate::utils::EncryptionKey;
use crate::utils::derive_encryption_key;
use crate::storage::StrongholdAdapter;
use crate::storage::VaultAdapter;
use crate::storage::EncryptedStorage;
use crate::utils::fs;

const STORAGE: Option<ArchiveStorage> = Some(ArchiveStorage::Stronghold);
const STORAGE_PATH: &str = "./storage";

#[derive(Debug)]
pub struct ArchiveBuilder {
  storage: Option<ArchiveStorage>,
  storage_path: PathBuf,
  storage_password: Option<EncryptionKey>,
}

impl ArchiveBuilder {
  pub fn new() -> Self {
    Self {
      storage: STORAGE,
      storage_path: STORAGE_PATH.into(),
      storage_password: None,
    }
  }

  pub fn storage<'a, P>(mut self, storage: ArchiveStorage, password: P) -> Self
  where
    P: Into<Option<&'a str>>,
  {
    self.storage = storage.into();
    self.storage_password = password.into().map(derive_encryption_key);
    self
  }

  pub fn storage_path<P>(mut self, storage_path: &P) -> Self
  where
    P: AsRef<Path> + ?Sized,
  {
    self.storage_path = storage_path.as_ref().into();
    self
  }

  pub async fn build(self) -> Result<Archive> {
    let adapter: Box<dyn VaultAdapter> = match self.storage {
      Some(ArchiveStorage::Stronghold) => {
        let path: PathBuf = fs::database_file(&self.storage_path, "identity.vault");

        fs::ensure_directory(&path)?;

        Box::new(StrongholdAdapter::new(&path, self.storage_password).await?)
      }
      Some(ArchiveStorage::Custom(adapter)) => {
        Box::new(EncryptedStorage::new(adapter, self.storage_password))
      }
      None => {
        return Err(Error::MissingStorageAdapter);
      }
    };

    let storage: StorageVault = StorageVault::new(adapter);
    let archive: Archive = Archive::new(storage);

    archive.initialize().await?;

    Ok(archive)
  }
}
