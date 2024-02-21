# \ExperimentalApi

All URIs are relative to *https://thunderstore.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_experimental_community_category_list**](ExperimentalApi.md#api_experimental_community_category_list) | **GET** /api/experimental/community/{community}/category/ | 
[**api_experimental_community_list**](ExperimentalApi.md#api_experimental_community_list) | **GET** /api/experimental/community/ | 
[**api_experimental_current_user_list**](ExperimentalApi.md#api_experimental_current_user_list) | **GET** /api/experimental/current-user/ | 
[**api_experimental_package_list**](ExperimentalApi.md#api_experimental_package_list) | **GET** /api/experimental/package/ | 
[**api_experimental_submission_upload_create**](ExperimentalApi.md#api_experimental_submission_upload_create) | **POST** /api/experimental/submission/upload/ | 
[**api_experimental_submission_upload_list**](ExperimentalApi.md#api_experimental_submission_upload_list) | **GET** /api/experimental/submission/upload/ | 
[**experimental_package_read**](ExperimentalApi.md#experimental_package_read) | **GET** /api/experimental/package/{namespace}/{name}/ | 
[**experimental_package_version_changelog_read**](ExperimentalApi.md#experimental_package_version_changelog_read) | **GET** /api/experimental/package/{namespace}/{name}/{version}/changelog/ | 
[**experimental_package_version_read**](ExperimentalApi.md#experimental_package_version_read) | **GET** /api/experimental/package/{namespace}/{name}/{version}/ | 
[**experimental_package_version_readme_read**](ExperimentalApi.md#experimental_package_version_readme_read) | **GET** /api/experimental/package/{namespace}/{name}/{version}/readme/ | 
[**experimental_period_auth_period_complete**](ExperimentalApi.md#experimental_period_auth_period_complete) | **POST** /api/experimental/auth/complete/{provider}/ | 
[**experimental_period_auth_period_delete**](ExperimentalApi.md#experimental_period_auth_period_delete) | **POST** /api/experimental/auth/delete/ | Drop provided session from database.
[**experimental_period_auth_period_overwolf_period_login**](ExperimentalApi.md#experimental_period_auth_period_overwolf_period_login) | **POST** /api/experimental/auth/overwolf/login/ | Login user with information from Overwolf API using the received JWT
[**experimental_period_auth_period_validate**](ExperimentalApi.md#experimental_period_auth_period_validate) | **GET** /api/experimental/auth/validate/ | 
[**experimental_period_community_period_current**](ExperimentalApi.md#experimental_period_community_period_current) | **GET** /api/experimental/current-community/ | 
[**experimental_period_frontend_period_community_period_package**](ExperimentalApi.md#experimental_period_frontend_period_community_period_package) | **GET** /api/experimental/frontend/c/{community_identifier}/p/{package_namespace}/{package_name}/ | 
[**experimental_period_frontend_period_community_period_packages**](ExperimentalApi.md#experimental_period_frontend_period_community_period_packages) | **GET** /api/experimental/frontend/c/{community_identifier}/packages/ | 
[**experimental_period_frontend_period_frontpage**](ExperimentalApi.md#experimental_period_frontend_period_frontpage) | **GET** /api/experimental/frontend/frontpage/ | 
[**experimental_period_frontend_period_render_markdown**](ExperimentalApi.md#experimental_period_frontend_period_render_markdown) | **POST** /api/experimental/frontend/render-markdown/ | 
[**experimental_period_modpacks_period_legacyprofile_period_create**](ExperimentalApi.md#experimental_period_modpacks_period_legacyprofile_period_create) | **POST** /api/experimental/legacyprofile/create/ | 
[**experimental_period_modpacks_period_legacyprofile_period_retrieve**](ExperimentalApi.md#experimental_period_modpacks_period_legacyprofile_period_retrieve) | **GET** /api/experimental/legacyprofile/get/{key}/ | 
[**experimental_period_package_index**](ExperimentalApi.md#experimental_period_package_index) | **GET** /api/experimental/package-index/ | 
[**experimental_period_package_listing_period_approve**](ExperimentalApi.md#experimental_period_package_listing_period_approve) | **POST** /api/experimental/package-listing/{id}/approve/ | 
[**experimental_period_package_listing_period_reject**](ExperimentalApi.md#experimental_period_package_listing_period_reject) | **POST** /api/experimental/package-listing/{id}/reject/ | 
[**experimental_period_package_listing_period_update**](ExperimentalApi.md#experimental_period_package_listing_period_update) | **POST** /api/experimental/package-listing/{id}/update/ | 
[**experimental_period_package_period_submit**](ExperimentalApi.md#experimental_period_package_period_submit) | **POST** /api/experimental/submission/submit/ | 
[**experimental_period_package_period_submit_async**](ExperimentalApi.md#experimental_period_package_period_submit_async) | **POST** /api/experimental/submission/submit-async/ | 
[**experimental_period_package_period_submit_async_period_poll**](ExperimentalApi.md#experimental_period_package_period_submit_async_period_poll) | **GET** /api/experimental/submission/poll-async/{submission_id}/ | 
[**experimental_period_submission_period_validate_period_icon**](ExperimentalApi.md#experimental_period_submission_period_validate_period_icon) | **POST** /api/experimental/submission/validate/icon/ | 
[**experimental_period_submission_period_validate_period_manifest_v1**](ExperimentalApi.md#experimental_period_submission_period_validate_period_manifest_v1) | **POST** /api/experimental/submission/validate/manifest-v1/ | 
[**experimental_period_submission_period_validate_period_readme**](ExperimentalApi.md#experimental_period_submission_period_validate_period_readme) | **POST** /api/experimental/submission/validate/readme/ | 



## api_experimental_community_category_list

> crate::models::ApiExperimentalCommunityCategoryList200Response api_experimental_community_category_list(community, cursor)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community** | **String** |  | [required] |
**cursor** | Option<**String**> | The pagination cursor value. |  |

### Return type

[**crate::models::ApiExperimentalCommunityCategoryList200Response**](api_experimental_community_category_list_200_response.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_experimental_community_list

> crate::models::ApiExperimentalCommunityList200Response api_experimental_community_list(cursor)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The pagination cursor value. |  |

### Return type

[**crate::models::ApiExperimentalCommunityList200Response**](api_experimental_community_list_200_response.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_experimental_current_user_list

> api_experimental_current_user_list()


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


## api_experimental_package_list

> crate::models::ApiExperimentalPackageList200Response api_experimental_package_list(cursor)


Lists all available packages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The pagination cursor value. |  |

### Return type

[**crate::models::ApiExperimentalPackageList200Response**](api_experimental_package_list_200_response.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_experimental_submission_upload_create

> api_experimental_submission_upload_create()


Uploads a package. Requires multipart/form-data.

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


## api_experimental_submission_upload_list

> api_experimental_submission_upload_list()


Uploads a package. Requires multipart/form-data.

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


## experimental_package_read

> crate::models::PackageExperimental experimental_package_read(name, namespace)


Get a single package

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**crate::models::PackageExperimental**](PackageExperimental.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_package_version_changelog_read

> crate::models::MarkdownResponse experimental_package_version_changelog_read(name, namespace, version)


Get a package verion's changelog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**version** | **String** |  | [required] |

### Return type

[**crate::models::MarkdownResponse**](MarkdownResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_package_version_read

> crate::models::PackageVersionExperimental experimental_package_version_read(name, namespace, version)


Get a single package version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**version** | **String** |  | [required] |

### Return type

[**crate::models::PackageVersionExperimental**](PackageVersionExperimental.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_package_version_readme_read

> crate::models::MarkdownResponse experimental_package_version_readme_read(name, namespace, version)


Get a package verion's readme

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**version** | **String** |  | [required] |

### Return type

[**crate::models::MarkdownResponse**](MarkdownResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_auth_period_complete

> crate::models::ResponseBody experimental_period_auth_period_complete(provider, data)


Complete OAuth login process initiated by a client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** |  | [required] |
**data** | [**RequestBody**](RequestBody.md) |  | [required] |

### Return type

[**crate::models::ResponseBody**](ResponseBody.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_auth_period_delete

> experimental_period_auth_period_delete()
Drop provided session from database.

The session is provided in Authorization header and is processed by UserSessionTokenAuthentication, which places the session key into request.auth.

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


## experimental_period_auth_period_overwolf_period_login

> crate::models::OwLoginResponseBody experimental_period_auth_period_overwolf_period_login(data)
Login user with information from Overwolf API using the received JWT

Used by Thunderstore Mod Manager. Not to be confused with OAuth login process triggered from a browser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**OwLoginRequestBody**](OwLoginRequestBody.md) |  | [required] |

### Return type

[**crate::models::OwLoginResponseBody**](OwLoginResponseBody.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_auth_period_validate

> experimental_period_auth_period_validate()


Check that valid session key is provided in Authorization header.

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


## experimental_period_community_period_current

> crate::models::Community experimental_period_community_period_current()


Fetch the Community of the queried domain

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Community**](Community.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_frontend_period_community_period_package

> crate::models::PackageDetailViewContent experimental_period_frontend_period_community_period_package(community_identifier, package_name, package_namespace)


Return details about a single Package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |
**package_name** | **String** |  | [required] |
**package_namespace** | **String** |  | [required] |

### Return type

[**crate::models::PackageDetailViewContent**](PackageDetailViewContent.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_frontend_period_community_period_packages

> crate::models::CommunityPackageList experimental_period_frontend_period_community_period_packages(community_identifier)


Return paginated list of community's packages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**community_identifier** | **String** |  | [required] |

### Return type

[**crate::models::CommunityPackageList**](CommunityPackageList.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_frontend_period_frontpage

> crate::models::FrontPageContent experimental_period_frontend_period_frontpage()


Return information required to render the site's front page.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FrontPageContent**](FrontPageContent.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_frontend_period_render_markdown

> crate::models::RenderMarkdownResponse experimental_period_frontend_period_render_markdown(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**RenderMarkdownParams**](RenderMarkdownParams.md) |  | [required] |

### Return type

[**crate::models::RenderMarkdownResponse**](RenderMarkdownResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_modpacks_period_legacyprofile_period_create

> crate::models::LegacyProfileCreateResponse experimental_period_modpacks_period_legacyprofile_period_create(UNKNOWN_PARAM_NAME)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**UNKNOWN_PARAM_NAME** | [****](.md) |  | [required] |

### Return type

[**crate::models::LegacyProfileCreateResponse**](LegacyProfileCreateResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_modpacks_period_legacyprofile_period_retrieve

> std::path::PathBuf experimental_period_modpacks_period_legacyprofile_period_retrieve(key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_package_index

> Vec<crate::models::PackageIndexEntry> experimental_period_package_index()


Response is a stream of newline delimited JSON.  Download links are not included in the response. The client is expected to build them using the following pattern: https://thunderstore.io/package/download/{namespace}/{name}/{version_number}/

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PackageIndexEntry>**](PackageIndexEntry.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_package_listing_period_approve

> experimental_period_package_listing_period_approve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this package listing. | [required] |

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_package_listing_period_reject

> crate::models::PackageListingRejectRequest experimental_period_package_listing_period_reject(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this package listing. | [required] |
**data** | [**PackageListingRejectRequest**](PackageListingRejectRequest.md) |  | [required] |

### Return type

[**crate::models::PackageListingRejectRequest**](PackageListingRejectRequest.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_package_listing_period_update

> crate::models::PackageListingUpdateResponse experimental_period_package_listing_period_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this package listing. | [required] |
**data** | [**PackageListingUpdateRequest**](PackageListingUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::PackageListingUpdateResponse**](PackageListingUpdateResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_package_period_submit

> crate::models::PackageSubmissionResult experimental_period_package_period_submit(data)


Submits a pre-uploaded package by upload uuid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**PackageSubmissionMetadata**](PackageSubmissionMetadata.md) |  | [required] |

### Return type

[**crate::models::PackageSubmissionResult**](PackageSubmissionResult.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_package_period_submit_async

> crate::models::PackageSubmissionStatus experimental_period_package_period_submit_async(data)


Submits a pre-uploaded package by upload uuid asynchronously.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**PackageSubmissionMetadata**](PackageSubmissionMetadata.md) |  | [required] |

### Return type

[**crate::models::PackageSubmissionStatus**](PackageSubmissionStatus.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_package_period_submit_async_period_poll

> crate::models::PackageSubmissionResult experimental_period_package_period_submit_async_period_poll(submission_id)


Polls the status of an async package submission by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** |  | [required] |

### Return type

[**crate::models::PackageSubmissionResult**](PackageSubmissionResult.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_submission_period_validate_period_icon

> crate::models::ValidatorResponse experimental_period_submission_period_validate_period_icon(data)


Validates a package icon.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**IconValidatorParams**](IconValidatorParams.md) |  | [required] |

### Return type

[**crate::models::ValidatorResponse**](ValidatorResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_submission_period_validate_period_manifest_v1

> crate::models::ValidatorResponse experimental_period_submission_period_validate_period_manifest_v1(data)


Validates a package manifest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**ManifestV1ValidatorParams**](ManifestV1ValidatorParams.md) |  | [required] |

### Return type

[**crate::models::ValidatorResponse**](ValidatorResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimental_period_submission_period_validate_period_readme

> crate::models::ValidatorResponse experimental_period_submission_period_validate_period_readme(data)


Validates a package readme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**ReadmeValidatorParams**](ReadmeValidatorParams.md) |  | [required] |

### Return type

[**crate::models::ValidatorResponse**](ValidatorResponse.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

