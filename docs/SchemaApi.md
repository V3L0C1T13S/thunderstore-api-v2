# \SchemaApi

All URIs are relative to *https://thunderstore.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**experimental_period_schema_period_channel_period_retrieve**](SchemaApi.md#experimental_period_schema_period_channel_period_retrieve) | **GET** /api/experimental/schema/{channel}/latest/ | 
[**experimental_period_schema_period_channel_period_update**](SchemaApi.md#experimental_period_schema_period_channel_period_update) | **POST** /api/experimental/schema/{channel}/ | 



## experimental_period_schema_period_channel_period_retrieve

> std::path::PathBuf experimental_period_schema_period_channel_period_retrieve(channel)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_schema_period_channel_period_update

> crate::models::SchemaChannelUpdateResponse experimental_period_schema_period_channel_period_update(channel, UNKNOWN_PARAM_NAME)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel** | **String** |  | [required] |
**UNKNOWN_PARAM_NAME** | [****](.md) |  | [required] |

### Return type

[**crate::models::SchemaChannelUpdateResponse**](SchemaChannelUpdateResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

