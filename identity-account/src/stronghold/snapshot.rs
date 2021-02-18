// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;
use iota_stronghold::StrongholdFlags;
use std::time::Duration;

use crate::error::Result;
use crate::stronghold::Store;
use crate::stronghold::Vault;
use crate::stronghold::Password;
use crate::stronghold::Runtime;

#[derive(Debug)]
pub enum SnapshotStatus {
  /// Snapshot is locked. This means that the password must be set again.
  Locked,
  /// Snapshot is unlocked. The duration is the amount of time left before it locks again.
  Unlocked(Duration),
}

impl SnapshotStatus {
  pub(crate) fn locked() -> Self {
    Self::Locked
  }

  pub(crate) fn unlocked(duration: Duration) -> Self {
    Self::Unlocked(duration)
  }
}

#[derive(Debug)]
pub struct Snapshot {
  path: PathBuf,
}

impl Snapshot {
  pub fn new<P>(path: &P) -> Self
  where
    P: AsRef<Path> + ?Sized,
  {
    Self {
      path: path.as_ref().to_path_buf(),
    }
  }

  pub fn vault<T>(&self, name: &T, flags: &[StrongholdFlags]) -> Vault
  where
    T: AsRef<[u8]> + ?Sized,
  {
    Vault {
      flags: flags.to_vec(),
      name: name.as_ref().to_vec(),
      path: self.path.clone(),
    }
  }

  pub fn store<T>(&self, name: &T, flags: &[StrongholdFlags]) -> Store
  where
    T: AsRef<[u8]> + ?Sized,
  {
    Store {
      flags: flags.to_vec(),
      name: name.as_ref().to_vec(),
      path: self.path.clone(),
    }
  }

  pub async fn status(&self) -> SnapshotStatus {
    Runtime::snapshot_status(&self.path).await
  }

  pub async fn set_password(&self, password: Password) {
    Runtime::set_password(&self.path, password).await;
  }

  pub async fn load(&self, password: Password) -> Result<()> {
    let mut runtime: _ = Runtime::lock().await?;

    self.set_password(password).await;

    runtime.set_snapshot(&self.path).await?;

    let status: SnapshotStatus = self.status().await;

    Runtime::emit_change(&self.path, status).await;

    Ok(())
  }

  pub async fn unload(&self, persist: bool) -> Result<()> {
    Runtime::lock().await?.write(&self.path, persist).await
  }
}
