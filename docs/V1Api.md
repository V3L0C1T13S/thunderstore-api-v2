# \V1Api

All URIs are relative to *https://thunderstore.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_bot_deprecate_mod_create**](V1Api.md#api_v1_bot_deprecate_mod_create) | **POST** /api/v1/bot/deprecate-mod/ | Deprecates a mod by its package name
[**api_v1_current_user_info_list**](V1Api.md#api_v1_current_user_info_list) | **GET** /api/v1/current-user/info/ | 
[**api_v1_package_list**](V1Api.md#api_v1_package_list) | **GET** /api/v1/package/ | 
[**api_v1_package_metrics**](V1Api.md#api_v1_package_metrics) | **GET** /api/v1/package-metrics/{namespace}/{name}/ | 
[**api_v1_package_metrics_0**](V1Api.md#api_v1_package_metrics_0) | **GET** /c/{community_identifier}/api/v1/package-metrics/{namespace}/{name}/ | 
[**api_v1_package_rate**](V1Api.md#api_v1_package_rate) | **POST** /api/v1/package/{uuid4}/rate/ | 
[**api_v1_package_read**](V1Api.md#api_v1_package_read) | **GET** /api/v1/package/{uuid4}/ | 
[**api_v1_package_version_metrics**](V1Api.md#api_v1_package_version_metrics) | **GET** /api/v1/package-metrics/{namespace}/{name}/{version}/ | 
[**api_v1_package_version_metrics_0**](V1Api.md#api_v1_package_version_metrics_0) | **GET** /c/{community_identifier}/api/v1/package-metrics/{namespace}/{name}/{version}/ | 
[**c_api_v1_bot_deprecate_mod_create**](V1Api.md#c_api_v1_bot_deprecate_mod_create) | **POST** /c/{community_identifier}/api/v1/bot/deprecate-mod/ | Deprecates a mod by its package name
[**c_api_v1_current_user_info_list**](V1Api.md#c_api_v1_current_user_info_list) | **GET** /c/{community_identifier}/api/v1/current-user/info/ | 
[**c_api_v1_package_list**](V1Api.md#c_api_v1_package_list) | **GET** /c/{community_identifier}/api/v1/package/ | 
[**c_api_v1_package_rate**](V1Api.md#c_api_v1_package_rate) | **POST** /c/{community_identifier}/api/v1/package/{uuid4}/rate/ | 
[**c_api_v1_package_read**](V1Api.md#c_api_v1_package_read) | **GET** /c/{community_identifier}/api/v1/package/{uuid4}/ | 



## api_v1_bot_deprecate_mod_create

> api_v1_bot_deprecate_mod_create()
Deprecates a mod by its package name

* Requires JWT authentication. * Only users with special permissions may use this action

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_current_user_info_list

> api_v1_current_user_info_list()


Gets information about the current user, such as rated packages and permissions

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_package_list

> Vec<crate::models::PackageListing> api_v1_package_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PackageListing>**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_package_metrics

> crate::models::PackageMetrics api_v1_package_metrics(name, namespace)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**crate::models::PackageMetrics**](PackageMetrics.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_package_metrics_0

> crate::models::PackageMetrics api_v1_package_metrics_0(community_identifier, name, namespace)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**crate::models::PackageMetrics**](PackageMetrics.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_package_rate

> crate::models::PackageListing api_v1_package_rate(uuid4, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid4** | **String** |  | [required] |
**data** | [**PackageListing**](PackageListing.md) |  | [required] |

### Return type

[**crate::models::PackageListing**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_package_read

> crate::models::PackageListing api_v1_package_read(uuid4)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid4** | **String** |  | [required] |

### Return type

[**crate::models::PackageListing**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_package_version_metrics

> crate::models::PackageVersionMetrics api_v1_package_version_metrics(name, namespace, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**version** | **String** |  | [required] |

### Return type

[**crate::models::PackageVersionMetrics**](PackageVersionMetrics.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_package_version_metrics_0

> crate::models::PackageVersionMetrics api_v1_package_version_metrics_0(community_identifier, name, namespace, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**version** | **String** |  | [required] |

### Return type

[**crate::models::PackageVersionMetrics**](PackageVersionMetrics.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_bot_deprecate_mod_create

> c_api_v1_bot_deprecate_mod_create(community_identifier)
Deprecates a mod by its package name

* Requires JWT authentication. * Only users with special permissions may use this action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_current_user_info_list

> c_api_v1_current_user_info_list(community_identifier)


Gets information about the current user, such as rated packages and permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_package_list

> Vec<crate::models::PackageListing> c_api_v1_package_list(community_identifier)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PackageListing>**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_package_rate

> crate::models::PackageListing c_api_v1_package_rate(community_identifier, uuid4, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |
**uuid4** | **String** |  | [required] |
**data** | [**PackageListing**](PackageListing.md) |  | [required] |

### Return type

[**crate::models::PackageListing**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## c_api_v1_package_read

> crate::models::PackageListing c_api_v1_package_read(community_identifier, uuid4)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |
**uuid4** | **String** |  | [required] |

### Return type

[**crate::models::PackageListing**](PackageListing.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

