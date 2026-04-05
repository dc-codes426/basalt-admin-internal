# BatchVaultRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **uuid::Uuid** | Unique session identifier (UUID) | 
**hex_encryption_key** | **String** | 32-byte hex-encoded encryption key | 
**hex_chain_code** | **String** | Hex-encoded chain code | 
**local_party_id** | Option<**String**> | Local TSS party identifier | [optional]
**encryption_password** | **String** | Encryption password for the vault | 
**lib_type** | Option<[**models::LibType**](LibType.md)> |  | [optional]
**protocols** | **Vec<String>** | List of protocols to run keygen across | 
**public_key** | Option<**String**> | Existing public key (optional, for adding protocols to existing vault) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


