#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

pub use did_doc;
pub use did_url;
pub use identity_diff;
pub use serde_json::json;

#[macro_use]
pub mod common;
pub mod convert;
pub mod crypto;
pub mod error;
pub mod proof;
pub mod resolver;
pub mod utils;
pub mod vc;

pub use error::{Error, Result};

pub mod patch {
  use did_url::DID;
  use did_doc::Signature;
  use did_doc::Method;
  use did_doc::MethodScope;
  use did_doc::Service;

  use crate::common::OneOrMany;
  use crate::common::Value;

  // let p1 = PatchData::insert_method(m1, MethodScope::Authentication);
  // let p2 = PatchData::insert_method(m2, vec![MethodScope::CapabilityDelegation, MethodScope::CapabilityInvocation]);
  // let p3 = ...;

  // let patch = document.patch(vec![p1, p2, p3])?;

  // patch.set_msg(previous.msg());
  // patch.publish().await?;

  pub type JsonPointer = Vec<String>;

  pub type BitFlags = u64;

  #[derive(Serialize, Deserialize)]
  pub struct Patch {
    data: OneOrMany<PatchData>, // patch-specific data
    hash: String, // base58-encoded hash of normalized document with applied patch
    proof: Signature, // signature of patch, signed with auth key
    msgid: Option<String>, // Tangle message id of *prev* patch *OR* none
  }

  #[derive(Serialize, Deserialize)]
  #[serde(tag = "type", content = "data")]
  pub enum PatchData {
    InsertVerificationMethod {
      data: OneOrMany<Purpose<Method>>,
    },
    RemoveVerificationMethod {
      data: OneOrMany<Purpose<DID>>,
    },
    InsertService {
      data: OneOrMany<Service>,
    },
    RemoveService {
      data: OneOrMany<DID>,
    },
    InsertMetadata {
      data: Value,
    },
    RemoveMetadata {
      data: OneOrMany<JsonPointer>,
    },
    MerkleKeyRevokeOne {
      data: u64,
    },
    MerkleKeyRevokeSet {
      data: Vec<BitFlags>,
    },
  }

  impl PatchData {
    pub fn insert_method<S>(
      method: Method,
      purpose: Option<S>,
    ) -> Self
    where
      // S: Into<OneOrMany<MethodScope>>,
      S: Into<OneOrMany<String>>,
    {
      Self::InsertVerificationMethod {
        data: OneOrMany::One(Purpose {
          data: method,
          purpose: purpose.map(Into::into),
        })
      }
    }
  }

  #[derive(Serialize, Deserialize)]
  pub struct Purpose<T> {
    #[serde(flatten)]
    data: T,
    // purpose: Option<OneOrMany<MethodScope>>,
    purpose: Option<OneOrMany<String>>,
  }
}
