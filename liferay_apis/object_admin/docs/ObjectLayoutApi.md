# \ObjectLayoutApi

All URIs are relative to *http://localhost:8080/o/object-admin*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_layout**](ObjectLayoutApi.md#delete_object_layout) | **DELETE** /v1.0/object-layouts/{objectLayoutId} | 
[**delete_object_layout_batch**](ObjectLayoutApi.md#delete_object_layout_batch) | **DELETE** /v1.0/object-layouts/batch | 
[**get_object_definition_by_external_reference_code_object_layouts_page**](ObjectLayoutApi.md#get_object_definition_by_external_reference_code_object_layouts_page) | **GET** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-layouts | 
[**get_object_definition_object_layouts_page**](ObjectLayoutApi.md#get_object_definition_object_layouts_page) | **GET** /v1.0/object-definitions/{objectDefinitionId}/object-layouts | 
[**get_object_layout**](ObjectLayoutApi.md#get_object_layout) | **GET** /v1.0/object-layouts/{objectLayoutId} | 
[**post_object_definition_by_external_reference_code_object_layout**](ObjectLayoutApi.md#post_object_definition_by_external_reference_code_object_layout) | **POST** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-layouts | 
[**post_object_definition_object_layout**](ObjectLayoutApi.md#post_object_definition_object_layout) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-layouts | 
[**post_object_definition_object_layout_batch**](ObjectLayoutApi.md#post_object_definition_object_layout_batch) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-layouts/batch | 
[**put_object_layout**](ObjectLayoutApi.md#put_object_layout) | **PUT** /v1.0/object-layouts/{objectLayoutId} | 
[**put_object_layout_batch**](ObjectLayoutApi.md#put_object_layout_batch) | **PUT** /v1.0/object-layouts/batch | 



## delete_object_layout

> delete_object_layout(object_layout_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_layout_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_layout_batch

> delete_object_layout_batch(callback_url, body)


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


## get_object_definition_by_external_reference_code_object_layouts_page

> crate::models::PageObjectLayout get_object_definition_by_external_reference_code_object_layouts_page(external_reference_code, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectLayout**](PageObjectLayout.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_object_layouts_page

> crate::models::PageObjectLayout get_object_definition_object_layouts_page(object_definition_id, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectLayout**](PageObjectLayout.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_layout

> crate::models::ObjectLayout get_object_layout(object_layout_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_layout_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectLayout**](ObjectLayout.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_by_external_reference_code_object_layout

> crate::models::ObjectLayout post_object_definition_by_external_reference_code_object_layout(external_reference_code, object_layout)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**object_layout** | Option<[**ObjectLayout**](ObjectLayout.md)> |  |  |

### Return type

[**crate::models::ObjectLayout**](ObjectLayout.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_layout

> crate::models::ObjectLayout post_object_definition_object_layout(object_definition_id, object_layout)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**object_layout** | Option<[**ObjectLayout**](ObjectLayout.md)> |  |  |

### Return type

[**crate::models::ObjectLayout**](ObjectLayout.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_layout_batch

> post_object_definition_object_layout_batch(object_definition_id, callback_url, body)


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


## put_object_layout

> crate::models::ObjectLayout put_object_layout(object_layout_id, object_layout)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_layout_id** | **String** |  | [required] |
**object_layout** | Option<[**ObjectLayout**](ObjectLayout.md)> |  |  |

### Return type

[**crate::models::ObjectLayout**](ObjectLayout.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_layout_batch

> put_object_layout_batch(callback_url, body)


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

