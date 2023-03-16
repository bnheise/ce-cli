# \ListTypeEntryApi

All URIs are relative to *http://localhost:8080/o/headless-admin-list-type*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_list_type_entry**](ListTypeEntryApi.md#delete_list_type_entry) | **DELETE** /v1.0/list-type-entries/{listTypeEntryId} | 
[**delete_list_type_entry_batch**](ListTypeEntryApi.md#delete_list_type_entry_batch) | **DELETE** /v1.0/list-type-entries/batch | 
[**get_list_type_definition_list_type_entries_page**](ListTypeEntryApi.md#get_list_type_definition_list_type_entries_page) | **GET** /v1.0/list-type-definitions/{listTypeDefinitionId}/list-type-entries | 
[**get_list_type_entry**](ListTypeEntryApi.md#get_list_type_entry) | **GET** /v1.0/list-type-entries/{listTypeEntryId} | 
[**post_list_type_definition_list_type_entry**](ListTypeEntryApi.md#post_list_type_definition_list_type_entry) | **POST** /v1.0/list-type-definitions/{listTypeDefinitionId}/list-type-entries | 
[**post_list_type_definition_list_type_entry_batch**](ListTypeEntryApi.md#post_list_type_definition_list_type_entry_batch) | **POST** /v1.0/list-type-definitions/{listTypeDefinitionId}/list-type-entries/batch | 
[**put_list_type_entry**](ListTypeEntryApi.md#put_list_type_entry) | **PUT** /v1.0/list-type-entries/{listTypeEntryId} | 
[**put_list_type_entry_batch**](ListTypeEntryApi.md#put_list_type_entry_batch) | **PUT** /v1.0/list-type-entries/batch | 



## delete_list_type_entry

> delete_list_type_entry(list_type_entry_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_entry_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_list_type_entry_batch

> delete_list_type_entry_batch(callback_url, body)


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


## get_list_type_definition_list_type_entries_page

> crate::models::PageListTypeEntry get_list_type_definition_list_type_entries_page(list_type_definition_id, aggregation_terms, filter, page, page_size, search, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_definition_id** | **String** |  | [required] |
**aggregation_terms** | Option<**String**> |  |  |
**filter** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PageListTypeEntry**](PageListTypeEntry.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_type_entry

> crate::models::ListTypeEntry get_list_type_entry(list_type_entry_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_entry_id** | **String** |  | [required] |

### Return type

[**crate::models::ListTypeEntry**](ListTypeEntry.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_type_definition_list_type_entry

> crate::models::ListTypeEntry post_list_type_definition_list_type_entry(list_type_definition_id, list_type_entry)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_definition_id** | **String** |  | [required] |
**list_type_entry** | Option<[**ListTypeEntry**](ListTypeEntry.md)> |  |  |

### Return type

[**crate::models::ListTypeEntry**](ListTypeEntry.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_list_type_definition_list_type_entry_batch

> post_list_type_definition_list_type_entry_batch(list_type_definition_id, callback_url, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_definition_id** | **String** |  | [required] |
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


## put_list_type_entry

> crate::models::ListTypeEntry put_list_type_entry(list_type_entry_id, list_type_entry)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type_entry_id** | **String** |  | [required] |
**list_type_entry** | Option<[**ListTypeEntry**](ListTypeEntry.md)> |  |  |

### Return type

[**crate::models::ListTypeEntry**](ListTypeEntry.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_list_type_entry_batch

> put_list_type_entry_batch(callback_url, body)


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

