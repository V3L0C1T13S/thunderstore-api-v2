# \UsermediaApi

All URIs are relative to *https://thunderstore.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**experimental_period_usermedia_period_abort_upload**](UsermediaApi.md#experimental_period_usermedia_period_abort_upload) | **POST** /api/experimental/usermedia/{uuid}/abort-upload/ | 
[**experimental_period_usermedia_period_finish_upload**](UsermediaApi.md#experimental_period_usermedia_period_finish_upload) | **POST** /api/experimental/usermedia/{uuid}/finish-upload/ | 
[**experimental_period_usermedia_period_initiate_upload**](UsermediaApi.md#experimental_period_usermedia_period_initiate_upload) | **POST** /api/experimental/usermedia/initiate-upload/ | 



## experimental_period_usermedia_period_abort_upload

> crate::models::UserMedia experimental_period_usermedia_period_abort_upload(uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this user media. | [required] |

### Return type

[**crate::models::UserMedia**](UserMedia.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_usermedia_period_finish_upload

> crate::models::UserMedia experimental_period_usermedia_period_finish_upload(uuid, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this user media. | [required] |
**data** | [**UserMediaFinishUploadParams**](UserMediaFinishUploadParams.md) |  | [required] |

### Return type

[**crate::models::UserMedia**](UserMedia.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_usermedia_period_initiate_upload

> crate::models::UserMediaInitiateUploadResponse experimental_period_usermedia_period_initiate_upload(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**UserMediaInitiateUploadParams**](UserMediaInitiateUploadParams.md) |  | [required] |

### Return type

[**crate::models::UserMediaInitiateUploadResponse**](UserMediaInitiateUploadResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

