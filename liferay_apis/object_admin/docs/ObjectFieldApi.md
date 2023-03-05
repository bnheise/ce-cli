# \ObjectFieldApi

All URIs are relative to *http://localhost:8080/o/object-admin*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_field**](ObjectFieldApi.md#delete_object_field) | **DELETE** /v1.0/object-fields/{objectFieldId} | 
[**delete_object_field_batch**](ObjectFieldApi.md#delete_object_field_batch) | **DELETE** /v1.0/object-fields/batch | 
[**get_object_definition_by_external_reference_code_object_fields_page**](ObjectFieldApi.md#get_object_definition_by_external_reference_code_object_fields_page) | **GET** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-fields | 
[**get_object_definition_object_fields_page**](ObjectFieldApi.md#get_object_definition_object_fields_page) | **GET** /v1.0/object-definitions/{objectDefinitionId}/object-fields | 
[**get_object_field**](ObjectFieldApi.md#get_object_field) | **GET** /v1.0/object-fields/{objectFieldId} | 
[**patch_object_field**](ObjectFieldApi.md#patch_object_field) | **PATCH** /v1.0/object-fields/{objectFieldId} | 
[**post_object_definition_by_external_reference_code_object_field**](ObjectFieldApi.md#post_object_definition_by_external_reference_code_object_field) | **POST** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-fields | 
[**post_object_definition_object_field**](ObjectFieldApi.md#post_object_definition_object_field) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-fields | 
[**post_object_definition_object_field_batch**](ObjectFieldApi.md#post_object_definition_object_field_batch) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-fields/batch | 
[**put_object_field**](ObjectFieldApi.md#put_object_field) | **PUT** /v1.0/object-fields/{objectFieldId} | 
[**put_object_field_batch**](ObjectFieldApi.md#put_object_field_batch) | **PUT** /v1.0/object-fields/batch | 



## delete_object_field

> delete_object_field(object_field_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_field_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_field_batch

> delete_object_field_batch(callback_url, body)


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


## get_object_definition_by_external_reference_code_object_fields_page

> crate::models::PageObjectField get_object_definition_by_external_reference_code_object_fields_page(external_reference_code, filter, page, page_size, search, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**filter** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectField**](PageObjectField.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_object_fields_page

> crate::models::PageObjectField get_object_definition_object_fields_page(object_definition_id, filter, page, page_size, search, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**filter** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectField**](PageObjectField.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_field

> crate::models::ObjectField get_object_field(object_field_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_field_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectField**](ObjectField.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_object_field

> crate::models::ObjectField patch_object_field(object_field_id, object_field)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_field_id** | **String** |  | [required] |
**object_field** | Option<[**ObjectField**](ObjectField.md)> |  |  |

### Return type

[**crate::models::ObjectField**](ObjectField.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_by_external_reference_code_object_field

> crate::models::ObjectField post_object_definition_by_external_reference_code_object_field(external_reference_code, object_field)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**object_field** | Option<[**ObjectField**](ObjectField.md)> |  |  |

### Return type

[**crate::models::ObjectField**](ObjectField.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_field

> crate::models::ObjectField post_object_definition_object_field(object_definition_id, object_field)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**object_field** | Option<[**ObjectField**](ObjectField.md)> |  |  |

### Return type

[**crate::models::ObjectField**](ObjectField.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_field_batch

> post_object_definition_object_field_batch(object_definition_id, callback_url, body)


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


## put_object_field

> crate::models::ObjectField put_object_field(object_field_id, object_field)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_field_id** | **String** |  | [required] |
**object_field** | Option<[**ObjectField**](ObjectField.md)> |  |  |

### Return type

[**crate::models::ObjectField**](ObjectField.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_field_batch

> put_object_field_batch(callback_url, body)


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

