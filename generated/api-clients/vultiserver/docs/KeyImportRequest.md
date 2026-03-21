# KeyImportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Vault name | 
**session_id** | **uuid::Uuid** | Unique session identifier (UUID) | 
**hex_encryption_key** | **String** | 32-byte hex-encoded encryption key | 
**hex_chain_code** | **String** | Hex-encoded chain code | 
**local_party_id** | Option<**String**> | Local TSS party identifier | [optional]
**encryption_password** | **String** | Encryption password for the vault | 
**email** | **String** | Email address for vault backup delivery | 
**chains** | **Vec<String>** | List of blockchain chains to import (e.g. BTC, ETH) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


