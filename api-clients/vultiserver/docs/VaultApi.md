# \VaultApi

All URIs are relative to *https://api.vultisig.com/router*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_vault_creation**](VaultApi.md#check_vault_creation) | **POST** /vault/create/check | Check vault creation status
[**create_mldsa_vault**](VaultApi.md#create_mldsa_vault) | **POST** /vault/mldsa | Add ML-DSA key to existing vault
[**create_vault**](VaultApi.md#create_vault) | **POST** /vault/create | Create a new vault (keygen)
[**exist_vault**](VaultApi.md#exist_vault) | **GET** /vault/exist/{publicKeyECDSA} | Check if a vault exists
[**get_vault**](VaultApi.md#get_vault) | **GET** /vault/get/{publicKeyECDSA} | Retrieve vault metadata
[**import_vault**](VaultApi.md#import_vault) | **POST** /vault/import | Import an existing vault
[**migrate_vault**](VaultApi.md#migrate_vault) | **POST** /vault/migrate | Migrate vault from GG20 to DKLS
[**resend_vault_email**](VaultApi.md#resend_vault_email) | **POST** /vault/resend | Resend vault backup email
[**reshare_vault**](VaultApi.md#reshare_vault) | **POST** /vault/reshare | Reshare vault key shares
[**verify_code**](VaultApi.md#verify_code) | **GET** /vault/verify/{publicKeyECDSA}/{code} | Verify email confirmation code



## check_vault_creation

> models::VaultCreateCheckResponse check_vault_creation(vault_create_check_request)
Check vault creation status

Polls the status of an in-progress vault key generation ceremony. Returns \"ongoing\" while the ceremony is still running, or the resulting public keys once the ceremony completes. The server retains ceremony state until this endpoint is called and returns a completed result. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_create_check_request** | [**VaultCreateCheckRequest**](VaultCreateCheckRequest.md) |  | [required] |

### Return type

[**models::VaultCreateCheckResponse**](VaultCreateCheckResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_mldsa_vault

> create_mldsa_vault(create_mldsa_request)
Add ML-DSA key to existing vault

Adds a post-quantum ML-DSA (Dilithium) key to an existing vault. The vault must already exist in block storage. Enqueues an async `key:createMldsa` task with a 5-minute timeout. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_mldsa_request** | [**CreateMldsaRequest**](CreateMldsaRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vault

> create_vault(vault_create_request)
Create a new vault (keygen)

Initiates a new vault key generation ceremony. Enqueues an async `key:generation` (GG20) or `key:generationDKLS` (DKLS) task with a 7-minute timeout. The server participates as one party in the TSS protocol. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_create_request** | [**VaultCreateRequest**](VaultCreateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exist_vault

> exist_vault(public_key_ecdsa)
Check if a vault exists

Checks whether a vault backup file exists for the given ECDSA public key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_key_ecdsa** | **String** | 66-character hex-encoded ECDSA public key identifying the vault | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vault

> models::VaultGetResponse get_vault(public_key_ecdsa, x_password)
Retrieve vault metadata

Retrieves and decrypts a vault backup file by its ECDSA public key. The encryption password must be provided via the `x-password` header (base64-encoded or plaintext). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_key_ecdsa** | **String** | 66-character hex-encoded ECDSA public key identifying the vault | [required] |
**x_password** | **String** | Encryption password for vault decryption. May be base64-encoded or sent as plaintext; the server attempts base64 decoding first.  | [required] |

### Return type

[**models::VaultGetResponse**](VaultGetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_vault

> import_vault(key_import_request)
Import an existing vault

Imports an existing vault via key import. The lib_type is forced to `KeyImport` (2). Enqueues an async `key:import` task with a 7-minute timeout. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_import_request** | [**KeyImportRequest**](KeyImportRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_vault

> migrate_vault(migration_request)
Migrate vault from GG20 to DKLS

Converts an existing GG20 vault to the DKLS protocol. Enqueues an async `key:migrate` task with a 7-minute timeout. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migration_request** | [**MigrationRequest**](MigrationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_vault_email

> resend_vault_email(vault_resend_request)
Resend vault backup email

Resends the vault backup file to the specified email address. Subject to a 3-minute cooldown between resends per vault. Enqueues a `key:resendVaultShareEmail` task. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vault_resend_request** | [**VaultResendRequest**](VaultResendRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reshare_vault

> reshare_vault(reshare_request)
Reshare vault key shares

Redistributes vault shares among a new set of parties. Enqueues an async `key:reshare` (GG20) or `key:reshareDKLS` (DKLS) task with an 8-minute timeout. Requires a minimum 6-character encryption password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reshare_request** | [**ReshareRequest**](ReshareRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_code

> verify_code(public_key_ecdsa, code)
Verify email confirmation code

Verifies a confirmation code sent via email for the given vault. The code is compared against a value stored in Redis. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_key_ecdsa** | **String** | 66-character hex-encoded ECDSA public key identifying the vault | [required] |
**code** | **String** | Verification code received via email | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

