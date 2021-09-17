---
title: Creating a Decentralized Identity
sidebar_label: Create and Publish
description: DID Documents and publishing them to the Tangle
image: /img/Identity_icon.png
keywords:
- Documents
- DID
- Tangle
- Publish
---

When someone or something wants to benefit from Self-Sovereign Identity, they must first create a Decentralized Identity. This identity consists of many parts that have different functions. This page will cover both the basics and the details about identity creation, storage, and publishing to the Tangle. 

The example below utilizes the high-level account module of the IOTA Identity framework to create an identity. The account is the easiest method of using IOTA Identity. It is recommended to use the account for your use cases, a lower-level API is also available, providing more flexibility at the cost of more complexity. 

### Creating an Identity using the Account

:::tip Using Replit

Select your programming language of choice and press the green play button to execute the example. 

:::

import CodeSnippet from '../../../src/components/CodeSnippetComponent'

<CodeSnippet nodeReplitLink="https://repl.it/@abdulmth/Create-did?lite=true"
rustReplitLink="https://replit.com/@JelleMillenaar1/accountbasic?lite=true"></CodeSnippet> 

The first step in this example is the creation of an account. This acts as a stateful object that manages one or more identities. The account provides an interface to execute high-level operations on identities, such as creating, updating, and storing them.  

Next, the identity is created and published to the IOTA Tangle. This operation will generate a private key, storing it in the account, generating a DID, DID Document, and publishing it to the Tangle. Once it is uploaded to the Tangle, it becomes immutable, meaning that this version of the identity can never be altered or removed. The only way to update or delete an identity is by publishing a new version, which we will discuss in the next section. This immutability is what makes a Decentralized Identity solution based on Distributed Ledger Technology (DLT) trustworthy. The public keys inside the DID Document can never be changed without having access to the private key, allowing the users to completely control their own identities. The rest of the example shows how to retrieve (resolve) the identity from the Tangle and how it can be deleted. 

### Identity Generation Process

The generation of an identity starts with a randomly generated asymmetric key pair. This can be generated by the IOTA Identity framework or can be provided as a parameter during the creation process. The public key is hashed using the `Blake2b-256` algorithm. This hash becomes the DID, creating a permanent and provable link between the initial keypair and the DID. The public key is then embedded into the initial DID Document and is used for verifying signatures created with the corresponding private key. This process can be observed and manipulated in depth by using the low-level API available for the IOTA Identity framework. This low-level API can be found here (TODO: Link) and is only recommended for complex use cases that require maximum flexibility in the framework. 