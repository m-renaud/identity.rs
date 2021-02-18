// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::error::Result;
use crate::storage::StorageAdapter;
use crate::storage::StrongholdAdapter;

pub enum ArchiveStorage {
  Stronghold,
  Custom(Box<dyn StorageAdapter>),
}

impl ArchiveStorage {
  pub(crate) fn into_adapter<P>(self, path: &P) -> Box<dyn StorageAdapter>
  where
    P: AsRef<Path> + ?Sized,
  {
    match self {
      Self::Stronghold => Box::new(StrongholdAdapter::new(path)),
      Self::Custom(adapter) => adapter,
    }
  }

  pub(crate) fn database_file(&self, path: PathBuf) -> PathBuf {
    if path.is_file() || path.extension().is_some() {
      return path;
    }

    match self {
      Self::Stronghold => path.join("identity.stronghold"),
      Self::Custom(_) => path,
    }
  }

  pub(crate) fn ensure_path(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }

    Ok(())
  }
}

impl Debug for ArchiveStorage {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Self::Stronghold => f.write_str("Stronghold"),
      Self::Custom(_) => f.write_str("Custom"),
    }
  }
}
