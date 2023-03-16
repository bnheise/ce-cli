# \ObjectDefinitionApi

All URIs are relative to *http://localhost:8080/o/object-admin*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_definition**](ObjectDefinitionApi.md#delete_object_definition) | **DELETE** /v1.0/object-definitions/{objectDefinitionId} | 
[**delete_object_definition_batch**](ObjectDefinitionApi.md#delete_object_definition_batch) | **DELETE** /v1.0/object-definitions/batch | 
[**get_object_definition**](ObjectDefinitionApi.md#get_object_definition) | **GET** /v1.0/object-definitions/{objectDefinitionId} | 
[**get_object_definition_by_external_reference_code**](ObjectDefinitionApi.md#get_object_definition_by_external_reference_code) | **GET** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode} | 
[**get_object_definitions_page**](ObjectDefinitionApi.md#get_object_definitions_page) | **GET** /v1.0/object-definitions | 
[**patch_object_definition**](ObjectDefinitionApi.md#patch_object_definition) | **PATCH** /v1.0/object-definitions/{objectDefinitionId} | 
[**post_object_definition**](ObjectDefinitionApi.md#post_object_definition) | **POST** /v1.0/object-definitions | 
[**post_object_definition_batch**](ObjectDefinitionApi.md#post_object_definition_batch) | **POST** /v1.0/object-definitions/batch | 
[**post_object_definition_publish**](ObjectDefinitionApi.md#post_object_definition_publish) | **POST** /v1.0/object-definitions/{objectDefinitionId}/publish | 
[**put_object_definition**](ObjectDefinitionApi.md#put_object_definition) | **PUT** /v1.0/object-definitions/{objectDefinitionId} | 
[**put_object_definition_batch**](ObjectDefinitionApi.md#put_object_definition_batch) | **PUT** /v1.0/object-definitions/batch | 
[**put_object_definition_by_external_reference_code**](ObjectDefinitionApi.md#put_object_definition_by_external_reference_code) | **PUT** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode} | 



## delete_object_definition

> delete_object_definition(object_definition_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_definition_batch

> delete_object_definition_batch(callback_url, body)


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


## get_object_definition

> crate::models::ObjectDefinition get_object_definition(object_definition_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectDefinition**](ObjectDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_by_external_reference_code

> crate::models::ObjectDefinition get_object_definition_by_external_reference_code(external_reference_code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |

### Return type

[**crate::models::ObjectDefinition**](ObjectDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definitions_page

> crate::models::PageObjectDefinition get_object_definitions_page(aggregation_terms, filter, page, page_size, search, sort)


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

[**crate::models::PageObjectDefinition**](PageObjectDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_object_definition

> crate::models::ObjectDefinition patch_object_definition(object_definition_id, object_definition)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**object_definition** | Option<[**ObjectDefinition**](ObjectDefinition.md)> |  |  |

### Return type

[**crate::models::ObjectDefinition**](ObjectDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition

> crate::models::ObjectDefinition post_object_definition(object_definition)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition** | Option<[**ObjectDefinition**](ObjectDefinition.md)> |  |  |

### Return type

[**crate::models::ObjectDefinition**](ObjectDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_batch

> post_object_definition_batch(callback_url, body)


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


## post_object_definition_publish

> post_object_definition_publish(object_definition_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_definition

> crate::models::ObjectDefinition put_object_definition(object_definition_id, object_definition)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**object_definition** | Option<[**ObjectDefinition**](ObjectDefinition.md)> |  |  |

### Return type

[**crate::models::ObjectDefinition**](ObjectDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_definition_batch

> put_object_definition_batch(callback_url, body)


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


## put_object_definition_by_external_reference_code

> crate::models::ObjectDefinition put_object_definition_by_external_reference_code(external_reference_code, object_definition)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**object_definition** | Option<[**ObjectDefinition**](ObjectDefinition.md)> |  |  |

### Return type

[**crate::models::ObjectDefinition**](ObjectDefinition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

