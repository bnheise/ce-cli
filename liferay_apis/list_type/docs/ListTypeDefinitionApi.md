# \ListTypeDefinitionApi

All URIs are relative to *http://localhost:8080/o/headless-admin-list-type*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_list_type_definition**](ListTypeDefinitionApi.md#delete_list_type_definition) | **DELETE** /v1.0/list-type-definitions/{listTypeDefinitionId} | 
[**delete_list_type_definition_batch**](ListTypeDefinitionApi.md#delete_list_type_definition_batch) | **DELETE** /v1.0/list-type-definitions/batch | 
[**get_list_type_definition**](ListTypeDefinitionApi.md#get_list_type_definition) | **GET** /v1.0/list-type-definitions/{listTypeDefinitionId} | 
[**get_list_type_definitions_page**](ListTypeDefinitionApi.md#get_list_type_definitions_page) | **GET** /v1.0/list-type-definitions | 
[**patch_list_type_definition**](ListTypeDefinitionApi.md#patch_list_type_definition) | **PATCH** /v1.0/list-type-definitions/{listTypeDefinitionId} | 
[**post_list_type_definition**](ListTypeDefinitionApi.md#post_list_type_definition) | **POST** /v1.0/list-type-definitions | 
[**post_list_type_definition_batch**](ListTypeDefinitionApi.md#post_list_type_definition_batch) | **POST** /v1.0/list-type-definitions/batch | 
[**put_list_type_definition**](ListTypeDefinitionApi.md#put_list_type_definition) | **PUT** /v1.0/list-type-definitions/{listTypeDefinitionId} | 
[**put_list_type_definition_batch**](ListTypeDefinitionApi.md#put_list_type_definition_batch) | **PUT** /v1.0/list-type-definitions/batch | 



## delete_list_type_definition

> delete_list_type_definition(list_type_definition_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_definition_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_list_type_definition_batch

> delete_list_type_definition_batch(callback_url, body)


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


## get_list_type_definition

> crate::models::ListTypeDefinition get_list_type_definition(list_type_definition_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_definition_id** | **String** |  | [required] |

### Return type

[**crate::models::ListTypeDefinition**](ListTypeDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_type_definitions_page

> crate::models::PageListTypeDefinition get_list_type_definitions_page(aggregation_terms, filter, page, page_size, search, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aggregation_terms** | Option<**String**> |  |  |
**filter** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PageListTypeDefinition**](PageListTypeDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_list_type_definition

> crate::models::ListTypeDefinition patch_list_type_definition(list_type_definition_id, list_type_definition)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_definition_id** | **String** |  | [required] |
**list_type_definition** | Option<[**ListTypeDefinition**](ListTypeDefinition.md)> |  |  |

### Return type

[**crate::models::ListTypeDefinition**](ListTypeDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_type_definition

> crate::models::ListTypeDefinition post_list_type_definition(list_type_definition)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_definition** | Option<[**ListTypeDefinition**](ListTypeDefinition.md)> |  |  |

### Return type

[**crate::models::ListTypeDefinition**](ListTypeDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_type_definition_batch

> post_list_type_definition_batch(callback_url, body)


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


## put_list_type_definition

> crate::models::ListTypeDefinition put_list_type_definition(list_type_definition_id, list_type_definition)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_definition_id** | **String** |  | [required] |
**list_type_definition** | Option<[**ListTypeDefinition**](ListTypeDefinition.md)> |  |  |

### Return type

[**crate::models::ListTypeDefinition**](ListTypeDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_list_type_definition_batch

> put_list_type_definition_batch(callback_url, body)


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

