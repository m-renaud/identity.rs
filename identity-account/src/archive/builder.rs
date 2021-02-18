// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;

use crate::archive::Archive;
use crate::archive::ArchiveStorage;
use crate::error::Error;
use crate::error::Result;
use crate::storage::StorageHandle;
use crate::storage::StorageAdapter;
use crate::utils::EncryptionKey;
use crate::utils::derive_encryption_key;

const STORAGE: Option<ArchiveStorage> = Some(ArchiveStorage::Stronghold);
const STORAGE_PATH: &str = "./storage";

#[derive(Debug)]
pub struct ArchiveBuilder {
  storage: Option<ArchiveStorage>,
  storage_path: PathBuf,
  encryption_key: Option<EncryptionKey>,
}

impl ArchiveBuilder {
  pub fn new() -> Self {
    Self {
      storage: STORAGE,
      storage_path: STORAGE_PATH.into(),
      encryption_key: None,
    }
  }

  pub fn storage<'a, P>(mut self, storage: ArchiveStorage, password: P) -> Self
  where
    P: Into<Option<&'a str>>,
  {
    self.storage = storage.into();
    self.encryption_key = password.into().map(derive_encryption_key);
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
    let (adapter, path): (Box<dyn StorageAdapter>, PathBuf) = match self.storage {
      Some(storage) => {
        let path: PathBuf = storage.database_file(self.storage_path);

        if !matches!(storage, ArchiveStorage::Custom(_)) {
          ArchiveStorage::ensure_path(&path)?;
        }

        (storage.into_adapter(&path), path)
      }
      None => {
        return Err(Error::MissingStorageAdapter);
      }
    };

    let storage: StorageHandle = StorageHandle::new(adapter, path, self.encryption_key);
    let archive: Archive = Archive::new(storage);

    Ok(archive)
  }
}
