# \BatchApi

All URIs are relative to *https://api.vultisig.com/router*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vault_batch**](BatchApi.md#create_vault_batch) | **POST** /vault/batch/keygen | Batch vault creation
[**import_vault_batch**](BatchApi.md#import_vault_batch) | **POST** /vault/batch/import | Batch vault import
[**reshare_vault_batch**](BatchApi.md#reshare_vault_batch) | **POST** /vault/batch/reshare | Batch vault reshare



## create_vault_batch

> create_vault_batch(batch_vault_request)
Batch vault creation

Initiates vault creation across multiple protocols in a single request. Enqueues an async `key:keygenBatch` task with a 7-minute timeout. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_vault_request** | [**BatchVaultRequest**](BatchVaultRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_vault_batch

> import_vault_batch(batch_import_request)
Batch vault import

Imports vaults across multiple protocols in a single request. Protocols must not contain duplicates. Enqueues an async `key:importBatch` task with a 7-minute timeout. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_import_request** | [**BatchImportRequest**](BatchImportRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reshare_vault_batch

> reshare_vault_batch(batch_reshare_request)
Batch vault reshare

Initiates vault resharing across multiple protocols in a single request. Enqueues an async `key:reshareBatch` task with an 8-minute timeout. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_reshare_request** | [**BatchReshareRequest**](BatchReshareRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

