# \DefaultApi

All URIs are relative to *http://networking:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**health**](DefaultApi.md#health) | **GET** /health | Liveness check



## health

> String health()
Liveness check

Simple liveness probe that returns `ok` when the service is running. Called by basalt-admin-internal as part of its dependency health checks. 

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

