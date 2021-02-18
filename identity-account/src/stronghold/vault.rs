// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use iota_stronghold::StrongholdFlags;
use iota_stronghold::Location;
use iota_stronghold::RecordHint;
use iota_stronghold::VaultFlags;
use iota_stronghold::Procedure;
use iota_stronghold::ProcResult;

use crate::error::Result;
use crate::error::PleaseDontMakeYourOwnResult;
use crate::stronghold::Runtime;
use crate::stronghold::ProcedureResult;

#[derive(Debug)]
pub struct Vault {
  pub(crate) flags: Vec<StrongholdFlags>,
  pub(crate) name: Vec<u8>,
  pub(crate) path: PathBuf,
}

impl Vault {
  /// Inserts a record.
  pub async fn insert(&self, location: Location, payload: Vec<u8>, hint: RecordHint, flags: &[VaultFlags]) -> Result<()> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(&self.path).await?;
    runtime.load_actor(&self.path, &self.name, &self.flags).await?;
    runtime.write_to_vault(location, payload, hint, flags.to_vec()).await.to_result()?;

    Ok(())
  }

  /// Deletes a record.
  pub async fn delete(&self, location: Location, gc: bool) -> Result<()> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(&self.path).await?;
    runtime.load_actor(&self.path, &self.name, &self.flags).await?;
    runtime.delete_data(location, gc).await.to_result()?;

    Ok(())
  }

  /// Executes a runtime `procedure`.
  pub async fn execute(&self, procedure: Procedure) -> Result<ProcedureResult> {
    let mut runtime: _ = Runtime::lock().await?;

    runtime.set_snapshot(&self.path).await?;
    runtime.load_actor(&self.path, &self.name, &self.flags).await?;

    runtime.runtime_exec(procedure).await.to_result()
  }
}
