// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod encrypted;
mod stronghold;
mod traits;
mod vault;

pub use self::encrypted::EncryptedStorage;
pub use self::stronghold::StrongholdAdapter;
pub use self::traits::StorageAdapter;
pub use self::traits::VaultAdapter;
pub use self::vault::StorageVault;
