# \SigningApi

All URIs are relative to *https://api.vultisig.com/router*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sign_messages**](SigningApi.md#sign_messages) | **POST** /vault/sign | Sign messages with vault key



## sign_messages

> String sign_messages(keysign_request)
Sign messages with vault key

Enqueues a signing operation for one or more messages. Returns a task ID that can be used to poll for results. Enqueues an async `key:sign` (GG20) or `key:signDKLS` (DKLS) task with a 2-minute timeout. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keysign_request** | [**KeysignRequest**](KeysignRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

