# BatchImportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Vault name | 
**session_id** | **uuid::Uuid** | Unique session identifier (UUID) | 
**hex_encryption_key** | **String** | 32-byte hex-encoded encryption key | 
**local_party_id** | Option<**String**> | Local TSS party identifier | [optional]
**encryption_password** | **String** | Encryption password for the vault | 
**email** | **String** | Email address for vault backup delivery | 
**protocols** | **HashSet<String>** | List of protocols to import across (must not contain duplicates) | 
**chains** | **Vec<String>** | List of blockchain chains to import | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


