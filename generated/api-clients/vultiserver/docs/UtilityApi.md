# \UtilityApi

All URIs are relative to *https://api.vultisig.com/router*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_derived_public_key**](UtilityApi.md#get_derived_public_key) | **GET** /getDerivedPublicKey | Derive a public key



## get_derived_public_key

> String get_derived_public_key(public_key, hex_chain_code, derive_path, is_ed_dsa)
Derive a public key

Derives a child public key from a parent public key using the given chain code and derivation path. Supports both ECDSA and EdDSA curves. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_key** | **String** | Base public key (hex-encoded) | [required] |
**hex_chain_code** | **String** | Chain code in hexadecimal format | [required] |
**derive_path** | **String** | BIP-32 style derivation path (e.g. `m/44'/60'/0'/0/0`) | [required] |
**is_ed_dsa** | Option<**bool**> | Use EdDSA curve instead of ECDSA. Defaults to false. |  |[default to false]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

