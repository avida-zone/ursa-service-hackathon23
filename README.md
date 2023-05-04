# ursa-service

This repository is mocks the backends of the Verificable Credential (VC) Issuer and the rgAsset Holder (also the holders of the VC).

It abstracts the constructions of crypto materials from [Libursa],
the library behind [AnonCreds].
Note that whilst the general flow is similar to that of [AnonCreds],
this mock environment is NOT COMPATIBLE and should NOT be used with any other form then what is intended,
a Hackathon demonstration of the concept of AVIDA.

## Build

The build version of this can be found on docker hub: belsyuen/avida-api.

## APIs

### `/sub-proof-req-params/?issuer=<issuer_name>`

Returns the material required to setup a [rg-token-contract].
I.e. public key, credential schemas, etc support by the issuer

Each [rg-token-contract] originator may choose a combination of issuers for this.

Note: we assume one - to - one mapping for schemas and issuers.

This information is usually gathered from public ledgers / trust registries

### `/rg-holder-setup/controller_addr/wallet_addr`

- `controller_addr`: Addr of the controller of the [Vectis] Smart Contract Wallet
- `wallet_addr`: Addr of the [Vectis] Smart Contract Wallet

These are important to make sure that the offline credentials belong to these wallets and replay attacks are avoided.

This provides a mock environment for the rg-token holder.
Behind the scenes, all credentials are created and stored.
We help the user create credentials for all the issuers we support in this environment.

This is usually done directly from the user devices and credentials should never be shared.

### `/generate-proof/controller_addr/wallet_addr/nonce/issuer?<issuer_name>`

This is again a mock of the rg-token holder's device,
where we create the proof to be used in the transactions.

The `nonce` is specify for the `wallet_addr` on the rg-token-contract,
which enables the [on-chain verifier] to verify tx that are unique.

----
Copyright 2023 NYMLAB Srl
