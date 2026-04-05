# VaultCreateCheckResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **Status** | \"ongoing\" while the ceremony is running, \"complete\" when finished (enum: ongoing, complete) | 
**public_key_ecdsa** | Option<**String**> | Hex-encoded ECDSA public key (present when status is \"complete\") | [optional]
**public_key_eddsa** | Option<**String**> | Hex-encoded EdDSA public key (present when status is \"complete\") | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


