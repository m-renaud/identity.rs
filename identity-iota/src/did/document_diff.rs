// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::{
    convert::{AsJson as _, SerdeInto as _},
    did_doc::{Document, SetSignature, Signature, TrySignature, TrySignatureMut},
    identity_diff::{did_doc::DiffDocument, Diff},
};
use iota::MessageId;

use crate::{
    client::{Client, Network},
    did::{IotaDID, IotaDocument},
    error::Result,
    tangle::TangleRef,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DocumentDiff {
    pub(crate) did: IotaDID,
    pub(crate) diff: String,
    pub(crate) previous_message_id: MessageId,
    pub(crate) proof: Option<Signature>,
    #[serde(skip, default = "MessageId::null")]
    pub(crate) message_id: MessageId,
}

impl DocumentDiff {
    pub fn new(current: &IotaDocument, updated: &IotaDocument, previous_message_id: MessageId) -> Result<Self> {
        let a: Document = current.serde_into()?;
        let b: Document = updated.serde_into()?;
        let diff: String = Diff::diff(&a, &b)?.to_json()?;

        Ok(Self {
            did: current.id().clone(),
            previous_message_id,
            diff,
            proof: None,
            message_id: MessageId::null(),
        })
    }

    /// Returns the DID of associated document.
    pub fn id(&self) -> &IotaDID {
        &self.did
    }

    /// Returns the raw contents of the DID document diff.
    pub fn diff(&self) -> &str {
        &*self.diff
    }

    /// Returns the Tangle message id of the previous DID document diff.
    pub fn previous_message_id(&self) -> &MessageId {
        &self.previous_message_id
    }

    /// Returns a reference to the `DocumentDiff` proof.
    pub fn proof(&self) -> Option<&Signature> {
        self.proof.as_ref()
    }

    /// Publishes the `DocumentDiff` to the Tangle using a default `Client`.
    pub async fn publish(&mut self, message_id: &MessageId) -> Result<()> {
        let network: Network = Network::from_name(self.did.network());
        let client: Client = Client::from_network(network).await?;

        self.publish_with_client(&client, message_id).await
    }

    /// Publishes the `DocumentDiff` to the Tangle using the provided `Client`.
    pub async fn publish_with_client(&mut self, client: &Client, message_id: &MessageId) -> Result<()> {
        let message_id: MessageId = client.publish_diff(message_id, self).await?;

        self.set_message_id(message_id);

        Ok(())
    }

    pub(crate) fn merge(&self, document: &IotaDocument) -> Result<IotaDocument> {
        let data: DiffDocument = DiffDocument::from_json(&self.diff)?;

        document
            .serde_into()
            .and_then(|this: Document| Diff::merge(&this, data).map_err(Into::into))
            .and_then(|this: Document| this.serde_into())
            .map_err(Into::into)
    }
}

impl TangleRef for DocumentDiff {
    fn message_id(&self) -> &MessageId {
        &self.message_id
    }

    fn set_message_id(&mut self, message_id: MessageId) {
        self.message_id = message_id;
    }

    fn previous_message_id(&self) -> &MessageId {
        &self.previous_message_id
    }

    fn set_previous_message_id(&mut self, message_id: MessageId) {
        self.previous_message_id = message_id;
    }
}

impl TrySignature for DocumentDiff {
    fn signature(&self) -> Option<&Signature> {
        self.proof.as_ref()
    }
}

impl TrySignatureMut for DocumentDiff {
    fn signature_mut(&mut self) -> Option<&mut Signature> {
        self.proof.as_mut()
    }
}

impl SetSignature for DocumentDiff {
    fn set_signature(&mut self, value: Signature) {
        self.proof = Some(value);
    }
}