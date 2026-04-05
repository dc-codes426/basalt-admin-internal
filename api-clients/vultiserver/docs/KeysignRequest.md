# KeysignRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_key** | **String** | Public key identifying the vault backup file | 
**messages** | **Vec<String>** | Hex-encoded messages to sign | 
**session** | **uuid::Uuid** | Unique session identifier (UUID) | 
**hex_encryption_key** | **String** | 32-byte hex-encoded encryption key | 
**derive_path** | **String** | BIP-32 derivation path for the signing key | 
**is_ecdsa** | Option<**bool**> | Use ECDSA signing (true) or EdDSA (false) | [optional][default to false]
**vault_password** | **String** | Password to decrypt the vault backup | 
**chain** | Option<**String**> | Target blockchain (e.g. BTC, ETH) | [optional]
**mldsa** | Option<**bool**> | Use ML-DSA (post-quantum Dilithium) signing | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


