# BatchReshareRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_key** | **String** | Existing vault public key | 
**session_id** | **uuid::Uuid** | Unique session identifier (UUID) | 
**hex_encryption_key** | **String** | 32-byte hex-encoded encryption key | 
**local_party_id** | Option<**String**> | Local TSS party identifier | [optional]
**encryption_password** | **String** | Encryption password for the vault | 
**email** | **String** | Email address for vault backup delivery | 
**old_parties** | **Vec<String>** | List of party IDs from the previous share set | 
**protocols** | **Vec<String>** | List of protocols to reshare across | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


