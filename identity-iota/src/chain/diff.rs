// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::slice::Iter;
use iota::Message;
use iota::MessageId;

use crate::{
    chain::{AuthChain, DocumentChain},
    did::{DocumentDiff, IotaDID},
    error::{Error, Result},
    tangle::{MessageExt, MessageIdExt, MessageIndex, TangleRef as _},
};

#[derive(Debug)]
pub struct DiffChain {
    inner: Vec<DocumentDiff>,
}

impl DiffChain {
    /// Constructs a new `DiffChain` for the given `AuthChain` from a slice of `Message`s.
    pub fn try_from_messages(auth: &AuthChain, messages: &[Message]) -> Result<Self> {
        if messages.is_empty() {
            return Ok(Self::new());
        }

        let did: &IotaDID = auth.current().id();

        let mut index: MessageIndex<DocumentDiff> = messages
            .iter()
            .flat_map(|message| message.try_extract_diff(did))
            .collect();

        let mut this: Self = Self::new();

        while let Some(mut list) = index.remove(DocumentChain::__diff_message_id(auth, &this)) {
            'inner: while let Some(next) = list.pop() {
                if auth.current().verify_data(&next).is_ok() {
                    this.inner.push(next);
                    break 'inner;
                }
            }
        }

        Ok(this)
    }

    /// Creates a new `DiffChain`.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Returns the total number of diffs in the chain.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the diff chain is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Empties the diff chain, removing all diffs.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns an iterator yielding references to `DocumentDiff`s.
    pub fn iter(&self) -> Iter<'_, DocumentDiff> {
        self.inner.iter()
    }

    /// Returns the `MessageId` of the latest diff if the chain, if any.
    pub fn current_message_id(&self) -> Option<&MessageId> {
        self.inner.last().map(|diff| diff.message_id())
    }

    /// Adds a new diff to the diff chain.
    ///
    /// # Errors
    ///
    /// Fails if the diff signature is invalid or the Tangle message
    /// references within the diff are invalid.
    pub fn try_push(&mut self, auth: &AuthChain, diff: DocumentDiff) -> Result<()> {
        self.check_validity(auth, &diff)?;

        // SAFETY: we performed the necessary validation in `check_validity`.
        unsafe {
            self.push_unchecked(diff);
        }

        Ok(())
    }

    /// Adds a new diff to the diff chain with performing validation checks.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check the validity of
    /// the signature or Tangle references of the `DocumentDiff`.
    pub unsafe fn push_unchecked(&mut self, diff: DocumentDiff) {
        self.inner.push(diff);
    }

    /// Returns `true` if the `DocumentDiff` can be added to the diff chain.
    pub fn is_valid(&self, auth: &AuthChain, diff: &DocumentDiff) -> bool {
        self.check_validity(auth, diff).is_ok()
    }

    /// Checks if the `DocumentDiff` can be added to the diff chain.
    ///
    /// # Errors
    ///
    /// Fails if the `DocumentDiff` is not a valid addition.
    pub fn check_validity(&self, auth: &AuthChain, diff: &DocumentDiff) -> Result<()> {
        if auth.current().verify_data(diff).is_err() {
            return Err(Error::ChainError {
                error: "Invalid Signature",
            });
        }

        if diff.message_id().is_null() {
            return Err(Error::ChainError {
                error: "Invalid Message Id",
            });
        }

        if diff.previous_message_id().is_null() {
            return Err(Error::ChainError {
                error: "Invalid Previous Message Id",
            });
        }

        if diff.previous_message_id() != DocumentChain::__diff_message_id(auth, self) {
            return Err(Error::ChainError {
                error: "Invalid Previous Message Id",
            });
        }

        Ok(())
    }
}

impl Default for DiffChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        chain::{AuthChain, DocumentChain},
        crypto::KeyPair,
        did::{DocumentDiff, IotaDocument},
        tangle::TangleRef,
    };
    use identity_core::{
        did_doc::{MethodBuilder, MethodData, MethodRef, MethodType},
        proof::JcsEd25519Signature2020,
    };
    use iota::MessageId;

    #[test]
    fn test_diff_chain() {
        let mut chain: DocumentChain;
        let mut keys: Vec<KeyPair> = Vec::new();

        // =========================================================================
        // Create Initial Document
        // =========================================================================

        {
            let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder().build().unwrap();

            document.sign(keypair.secret()).unwrap();
            //document.publish_with_client(&client).await?;
            document.set_message_id(MessageId::new([8; 32]));

            chain = DocumentChain::new(AuthChain::new(document).unwrap());
            keys.push(keypair);
        }

        // =========================================================================
        // Push Auth Chain Update
        // =========================================================================

        {
            let mut new: IotaDocument = chain.current().clone();
            let keypair: KeyPair = JcsEd25519Signature2020::new_keypair();

            let authentication: MethodRef = MethodBuilder::default()
                .id(chain.id().join("#key-2").unwrap().into())
                .controller(chain.id().clone().into())
                .key_type(MethodType::Ed25519VerificationKey2018)
                .key_data(MethodData::new_b58(keypair.public()))
                .build()
                .map(Into::into)
                .unwrap();

            unsafe {
                new.as_document_mut().authentication_mut().clear();
                new.as_document_mut().authentication_mut().append(authentication.into());
            }

            new.set_updated_now();
            new.set_previous_message_id(chain.auth_message_id().clone());

            chain.current().sign_data(&mut new, keys[0].secret()).unwrap();
            //new.publish_with_client(&client).await?;

            keys.push(keypair);
            chain.try_push_auth(new).unwrap();
        }

        // =========================================================================
        // Push Diff Chain Update
        // =========================================================================

        {
            let new: IotaDocument = {
                let mut this: IotaDocument = chain.current().clone();
                this.properties_mut().insert("foo".into(), 123.into());
                this.properties_mut().insert("bar".into(), 456.into());
                this.set_updated_now();
                this
            };

            let message_id: MessageId = chain.diff_message_id().clone();
            let mut diff: DocumentDiff = chain.current().diff(&new, keys[1].secret(), message_id).unwrap();
            diff.set_message_id(message_id);
            assert!(chain.try_push_diff(diff).is_ok());
        }
    }
}
