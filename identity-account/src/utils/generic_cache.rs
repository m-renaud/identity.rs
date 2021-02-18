// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

type Map<T> = HashMap<String, T>;

pub type GenericReadGuard<'a, T> = RwLockReadGuard<'a, Map<T>>;

pub type GenericWriteGuard<'a, T> = RwLockWriteGuard<'a, Map<T>>;

#[derive(Clone, Debug)]
pub struct GenericCache<T> {
  data: Arc<RwLock<Map<T>>>,
}

impl<T> GenericCache<T> {
  pub fn new() -> Self {
    Self {
      data: Arc::new(RwLock::new(Map::new())),
    }
  }
}

impl<T> Deref for GenericCache<T> {
  type Target = RwLock<Map<T>>;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}
