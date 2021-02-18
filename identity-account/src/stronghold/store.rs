// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;
use std::path::PathBuf;
use iota_stronghold::StrongholdFlags;
use iota_stronghold::Location;

use crate::error::Result;
use crate::error::PleaseDontMakeYourOwnResult;
use crate::stronghold::Runtime;

#[derive(Debug)]
pub struct Store {
  pub(crate) flags: Vec<StrongholdFlags>,
  pub(crate) name: Vec<u8>,
  pub(crate) path: PathBuf,
}

impl Store {
  /// Gets a record.
  pub async fn get(&self, location: Location) -> Result<Vec<u8>> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(&self.path).await?;
    runtime.load_actor(&self.path, &self.name, &self.flags).await?;

    let (data, status): (Vec<u8>, _) = runtime.read_from_store(location).await;

    status.to_result()?;

    Ok(data)
  }

  /// Adds a record.
  pub async fn set(&self, location: Location, payload: Vec<u8>, ttl: Option<Duration>) -> Result<()> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(&self.path).await?;
    runtime.load_actor(&self.path, &self.name, &self.flags).await?;
    runtime.write_to_store(location, payload, ttl).await.to_result()?;

    Ok(())
  }

  /// Removes a record.
  pub async fn del(&self, location: Location) -> Result<()> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(&self.path).await?;
    runtime.load_actor(&self.path, &self.name, &self.flags).await?;
    runtime.delete_from_store(location).await.to_result()?;

    Ok(())
  }
}
