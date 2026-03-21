# VaultCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Human-readable vault name | 
**session_id** | **uuid::Uuid** | Unique session identifier (UUID) | 
**hex_encryption_key** | **String** | 32-byte hex-encoded encryption key | 
**hex_chain_code** | **String** | Hex-encoded chain code for key derivation | 
**local_party_id** | Option<**String**> | Identifier for the local TSS party (optional; server assigns one if omitted) | [optional]
**encryption_password** | **String** | Password used to encrypt the vault backup | 
**email** | **String** | Email address to send the vault backup to | 
**lib_type** | Option<[**models::LibType**](LibType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


