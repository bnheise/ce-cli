# \ObjectActionApi

All URIs are relative to *http://localhost:8080/o/object-admin*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_action**](ObjectActionApi.md#delete_object_action) | **DELETE** /v1.0/object-actions/{objectActionId} | 
[**delete_object_action_batch**](ObjectActionApi.md#delete_object_action_batch) | **DELETE** /v1.0/object-actions/batch | 
[**get_object_action**](ObjectActionApi.md#get_object_action) | **GET** /v1.0/object-actions/{objectActionId} | 
[**get_object_definition_by_external_reference_code_object_actions_page**](ObjectActionApi.md#get_object_definition_by_external_reference_code_object_actions_page) | **GET** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-actions | 
[**get_object_definition_object_actions_page**](ObjectActionApi.md#get_object_definition_object_actions_page) | **GET** /v1.0/object-definitions/{objectDefinitionId}/object-actions | 
[**patch_object_action**](ObjectActionApi.md#patch_object_action) | **PATCH** /v1.0/object-actions/{objectActionId} | 
[**post_object_definition_by_external_reference_code_object_action**](ObjectActionApi.md#post_object_definition_by_external_reference_code_object_action) | **POST** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-actions | 
[**post_object_definition_object_action**](ObjectActionApi.md#post_object_definition_object_action) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-actions | 
[**post_object_definition_object_action_batch**](ObjectActionApi.md#post_object_definition_object_action_batch) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-actions/batch | 
[**put_object_action**](ObjectActionApi.md#put_object_action) | **PUT** /v1.0/object-actions/{objectActionId} | 
[**put_object_action_batch**](ObjectActionApi.md#put_object_action_batch) | **PUT** /v1.0/object-actions/batch | 



## delete_object_action

> delete_object_action(object_action_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_action_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_action_batch

> delete_object_action_batch(callback_url, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callback_url** | Option<**String**> |  |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_action

> crate::models::ObjectAction get_object_action(object_action_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_action_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectAction**](ObjectAction.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_by_external_reference_code_object_actions_page

> crate::models::PageObjectAction get_object_definition_by_external_reference_code_object_actions_page(external_reference_code, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectAction**](PageObjectAction.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_object_actions_page

> crate::models::PageObjectAction get_object_definition_object_actions_page(object_definition_id, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectAction**](PageObjectAction.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_object_action

> crate::models::ObjectAction patch_object_action(object_action_id, object_action)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_action_id** | **String** |  | [required] |
**object_action** | Option<[**ObjectAction**](ObjectAction.md)> |  |  |

### Return type

[**crate::models::ObjectAction**](ObjectAction.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_by_external_reference_code_object_action

> crate::models::ObjectAction post_object_definition_by_external_reference_code_object_action(external_reference_code, object_action)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**object_action** | Option<[**ObjectAction**](ObjectAction.md)> |  |  |

### Return type

[**crate::models::ObjectAction**](ObjectAction.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_action

> crate::models::ObjectAction post_object_definition_object_action(object_definition_id, object_action)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**object_action** | Option<[**ObjectAction**](ObjectAction.md)> |  |  |

### Return type

[**crate::models::ObjectAction**](ObjectAction.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_action_batch

> post_object_definition_object_action_batch(object_definition_id, callback_url, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**callback_url** | Option<**String**> |  |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_action

> crate::models::ObjectAction put_object_action(object_action_id, object_action)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_action_id** | **String** |  | [required] |
**object_action** | Option<[**ObjectAction**](ObjectAction.md)> |  |  |

### Return type

[**crate::models::ObjectAction**](ObjectAction.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_action_batch

> put_object_action_batch(callback_url, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callback_url** | Option<**String**> |  |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

