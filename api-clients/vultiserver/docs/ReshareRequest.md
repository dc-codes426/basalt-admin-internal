# ReshareRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_key** | **String** | Existing vault public key | 
**session_id** | **uuid::Uuid** | Unique session identifier (UUID) | 
**hex_encryption_key** | **String** | 32-byte hex-encoded encryption key | 
**hex_chain_code** | **String** | Hex-encoded chain code | 
**local_party_id** | Option<**String**> | Local TSS party identifier | [optional]
**old_parties** | **Vec<String>** | List of party IDs from the previous share set | 
**encryption_password** | **String** | Encryption password (minimum 6 characters) | 
**old_reshare_prefix** | Option<**String**> | Prefix from previous reshare operation (if any) | [optional]
**lib_type** | Option<[**models::LibType**](LibType.md)> |  | [optional]
**reshare_type** | Option<[**models::ReshareType**](ReshareType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


