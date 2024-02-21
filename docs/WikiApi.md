# \WikiApi

All URIs are relative to *https://thunderstore.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**experimental_package_wiki_delete**](WikiApi.md#experimental_package_wiki_delete) | **DELETE** /api/experimental/package/{namespace}/{name}/wiki/ | Delete a wiki page
[**experimental_package_wiki_list**](WikiApi.md#experimental_package_wiki_list) | **GET** /api/experimental/package/wikis/ | List package wikis
[**experimental_package_wiki_read**](WikiApi.md#experimental_package_wiki_read) | **GET** /api/experimental/package/{namespace}/{name}/wiki/ | Get a list of all wiki pages
[**experimental_package_wiki_write**](WikiApi.md#experimental_package_wiki_write) | **POST** /api/experimental/package/{namespace}/{name}/wiki/ | Create or update a wiki page
[**experimental_wiki_page_read**](WikiApi.md#experimental_wiki_page_read) | **GET** /api/experimental/wiki/page/{id}/ | Get a wiki page



## experimental_package_wiki_delete

> experimental_package_wiki_delete(name, namespace, data)
Delete a wiki page

Deletes a wiki page by page ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**data** | [**WikiPageDelete**](WikiPageDelete.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_package_wiki_list

> crate::models::PackageWikiListResponse experimental_package_wiki_list(after)
List package wikis

Fetch a bulk of package wikis at once. Supports querying by update time to accommodate local caching.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> |  |  |

### Return type

[**crate::models::PackageWikiListResponse**](PackageWikiListResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_package_wiki_read

> crate::models::Wiki experimental_package_wiki_read(name, namespace)
Get a list of all wiki pages

Returns an index of all the pages included in the wiki

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**crate::models::Wiki**](Wiki.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_package_wiki_write

> crate::models::WikiPage experimental_package_wiki_write(name, namespace, data)
Create or update a wiki page

Creates a new wiki page if a submission is made without the ID field set. If the ID field is set, the respective page will be updated instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**data** | [**WikiPageUpsert**](WikiPageUpsert.md) |  | [required] |

### Return type

[**crate::models::WikiPage**](WikiPage.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_wiki_page_read

> crate::models::WikiPage experimental_wiki_page_read(id)
Get a wiki page

Returns a wiki page object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this wiki page. | [required] |

### Return type

[**crate::models::WikiPage**](WikiPage.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

