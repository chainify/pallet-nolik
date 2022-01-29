# Pallet Nolik
A [Substrate](https://substrate.io) FRAME pallet for sending encrypted messages between blockchain accounts

## Overview
Nolik is a protocol for delivering digital content for web 3.0.
It is designed to connect people without any form of censorship or third-party control.
That is possible due to a combination of blockchain and [IPFS](https://ipfs.io) technologies and a [serviceless](#serviceless) approach.

Pallet logic allows the creation of rules for programmable messages and embeds them to any Substrate-based chain.
With that, it is possible to control who to receive messages from on a cryptographic (blockchain) level.
That can be done due to the ability to create white lists (or black lists) of senders.

## Key Features
The protocol allows to:
* Communicate without servers or service as a third-party
* Start messaging without disclosing the identity
* Create an unlimited number of addresses (like emails)
* Stay protected from a middleware attack
* Protect messages from unauthorized access with a decentralized end-to-end encryption
* Prove that the message was sent at a particular date and time
* Prove that the message was sent from a particular sender and stay protected from a phishing attack
* Prove that the message was sent by a particular sender
* Prove that the message was sent to a particular recipient or recipients
* Prove that the message was not modified
* Attach tokens (like NFTs) to messages
* Use different clients to get access to messages

## ServiceLess
This approach stands for removing third parties or any form of centralization.
Regarding Nolik protocol, this means that there are no back-end servers that connect the sender and recipient of a message.
The message is composed, encrypted, sent, and received on the client-side.
The blockchain is used as a transport layer that broadcasts the IPFS hash.
The file with encrypted content is stored in the IPFS network.
The message can be sent and broadcasted only if the sender has a right to do it.
The rights are set by the recipient and configured with a white list and a black list of senders.
This simple but powerful combination allows communication between users without a third party.

