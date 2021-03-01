// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::iter;
use iota_stronghold::StrongholdFlags;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use iota_stronghold::hd::ChainCode;
use std::path::Path;
use std::path::PathBuf;
use iota_stronghold::hd::Chain;
use iota_stronghold::Location;
use iota_stronghold::SLIP10DeriveInput;
use iota_stronghold::RecordHint;
use iota_stronghold::Error as StrongError;
use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use crypto::hashes::Output;
use identity_core::utils::encode_b58;
use identity_core::utils::decode_b58;

use crate::error::Error;
use crate::error::Result;
use crate::storage::StorageAdapter;
use crate::stronghold::Records;
use crate::stronghold::Store;
use crate::stronghold::Vault;
use crate::stronghold::Snapshot;
use crate::utils::EncryptionKey;

const TAG_SEED: u8 = 0x0;
const TAG_SKEY: u8 = 0x1;

const SF_PUBLIC: &[StrongholdFlags] = &[StrongholdFlags::IsReadable(true)];
const SF_SECRET: &[StrongholdFlags] = &[StrongholdFlags::IsReadable(false)];

#[derive(Debug)]
pub struct StrongholdAdapter {
  snapshot: Snapshot,
}

impl StrongholdAdapter {
  pub async fn new<P>(path: &P, password: Option<EncryptionKey>) -> Result<Self>
  where
    P: AsRef<Path> + ?Sized,
  {
    let snapshot: Snapshot = Snapshot::new(path);

    if let Some(password) = password {
      snapshot.load(password).await?;
    }

    Ok(Self { snapshot })
  }
}

#[async_trait::async_trait]
impl StorageAdapter for StrongholdAdapter {
  async fn all(&mut self) -> Result<Vec<Vec<u8>>> {
    self.snapshot.records("identity", &[]).all().await
  }

  async fn get(&mut self, resource_id: &[u8]) -> Result<Vec<u8>> {
    todo!("StrongholdAdapter::get")
  }

  async fn set(&mut self, resource_id: &[u8], resource: &[u8]) -> Result<()> {
    let records: Records<'_> = self.snapshot.records("identity", SF_PUBLIC);

    records.set(resource_id, resource).await?;
    records.flush().await?;

    Ok(())
  }

  async fn del(&mut self, resource_id: &[u8]) -> Result<()> {
    todo!("StrongholdAdapter::del")
  }

  async fn generate_public_key(&self, type_: KeyType, identity: usize, fragment: &str) -> Result<PublicKey> {
    // Open a vault to store keys for *this* Identity
    let scope: String = format!("keystore:{}", identity);
    let vault: Vault = self.snapshot.vault(scope.as_bytes(), SF_SECRET);

    let public: PublicKey = match type_ {
      KeyType::Ed25519 => generate_ed25519(&vault, fragment).await?,
    };

    // Write to disk
    vault.flush().await?;

    Ok(public)
  }

  fn storage_path(&self) -> &Path {
    self.snapshot.path()
  }
}

async fn generate_ed25519(vault: &Vault<'_>, fragment: &str) -> Result<PublicKey> {
  let tag: Output<Blake2b256> = Blake2b256::digest(fragment.as_bytes());

  let tag_seed: Vec<u8> = iter::once(TAG_SEED).chain(tag.iter().copied()).collect();
  let tag_skey: Vec<u8> = iter::once(TAG_SKEY).chain(tag.iter().copied()).collect();

  let hint_seed: RecordHint = __hint(&tag_seed[..24])?;
  let hint_skey: RecordHint = __hint(&tag_skey[..24])?;

  let location_seed: Location = Location::generic("vault", tag_seed);
  let location_skey: Location = Location::generic("vault", tag_skey);

  // Generate a SLIP10 seed as the private key
  vault.slip10_generate(location_seed.clone(), hint_seed, None).await?;

  let chain: Chain = Chain::from_u32_hardened(vec![0, 0, 0]);
  let seed: SLIP10DeriveInput = SLIP10DeriveInput::Seed(location_seed);

  // Use the SLIP10 seed to derive a child key
  let _: ChainCode = vault.slip10_derive(chain, seed, location_skey.clone(), hint_skey).await?;

  // Retrieve the public key of the derived child key
  let public: [u8; 32] = vault.ed25519_public_key(location_skey).await?;
  let public: PublicKey = public.to_vec().into();

  Ok(public)
}

fn __hint<H>(data: &H) -> Result<RecordHint>
where
  H: AsRef<[u8]> + ?Sized,
{
  RecordHint::new(data.as_ref()).map_err(StrongError::from).map_err(Into::into)
}
