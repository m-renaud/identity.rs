// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use crate::storage::StorageAdapter;

pub enum ArchiveStorage {
  Stronghold,
  Custom(Box<dyn StorageAdapter>),
}

impl Debug for ArchiveStorage {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Stronghold => f.write_str("Stronghold"),
      Self::Custom(_) => f.write_str("Custom"),
    }
  }
}
