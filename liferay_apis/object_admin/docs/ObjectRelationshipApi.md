# \ObjectRelationshipApi

All URIs are relative to *http://localhost:8080/o/object-admin*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_relationship**](ObjectRelationshipApi.md#delete_object_relationship) | **DELETE** /v1.0/object-relationships/{objectRelationshipId} | 
[**delete_object_relationship_batch**](ObjectRelationshipApi.md#delete_object_relationship_batch) | **DELETE** /v1.0/object-relationships/batch | 
[**get_object_definition_by_external_reference_code_object_relationships_page**](ObjectRelationshipApi.md#get_object_definition_by_external_reference_code_object_relationships_page) | **GET** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-relationships | 
[**get_object_definition_object_relationships_page**](ObjectRelationshipApi.md#get_object_definition_object_relationships_page) | **GET** /v1.0/object-definitions/{objectDefinitionId}/object-relationships | 
[**get_object_relationship**](ObjectRelationshipApi.md#get_object_relationship) | **GET** /v1.0/object-relationships/{objectRelationshipId} | 
[**post_object_definition_by_external_reference_code_object_relationship**](ObjectRelationshipApi.md#post_object_definition_by_external_reference_code_object_relationship) | **POST** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-relationships | 
[**post_object_definition_object_relationship**](ObjectRelationshipApi.md#post_object_definition_object_relationship) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-relationships | 
[**post_object_definition_object_relationship_batch**](ObjectRelationshipApi.md#post_object_definition_object_relationship_batch) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-relationships/batch | 
[**put_object_relationship**](ObjectRelationshipApi.md#put_object_relationship) | **PUT** /v1.0/object-relationships/{objectRelationshipId} | 
[**put_object_relationship_batch**](ObjectRelationshipApi.md#put_object_relationship_batch) | **PUT** /v1.0/object-relationships/batch | 



## delete_object_relationship

> delete_object_relationship(object_relationship_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_relationship_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_relationship_batch

> delete_object_relationship_batch(callback_url, body)


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


## get_object_definition_by_external_reference_code_object_relationships_page

> crate::models::PageObjectRelationship get_object_definition_by_external_reference_code_object_relationships_page(external_reference_code, filter, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**filter** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectRelationship**](PageObjectRelationship.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_object_relationships_page

> crate::models::PageObjectRelationship get_object_definition_object_relationships_page(object_definition_id, filter, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**filter** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectRelationship**](PageObjectRelationship.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_relationship

> crate::models::ObjectRelationship get_object_relationship(object_relationship_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_relationship_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectRelationship**](ObjectRelationship.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_by_external_reference_code_object_relationship

> crate::models::ObjectRelationship post_object_definition_by_external_reference_code_object_relationship(external_reference_code, object_relationship)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**object_relationship** | Option<[**ObjectRelationship**](ObjectRelationship.md)> |  |  |

### Return type

[**crate::models::ObjectRelationship**](ObjectRelationship.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_relationship

> crate::models::ObjectRelationship post_object_definition_object_relationship(object_definition_id, object_relationship)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**object_relationship** | Option<[**ObjectRelationship**](ObjectRelationship.md)> |  |  |

### Return type

[**crate::models::ObjectRelationship**](ObjectRelationship.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_relationship_batch

> post_object_definition_object_relationship_batch(object_definition_id, callback_url, body)


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


## put_object_relationship

> crate::models::ObjectRelationship put_object_relationship(object_relationship_id, object_relationship)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_relationship_id** | **String** |  | [required] |
**object_relationship** | Option<[**ObjectRelationship**](ObjectRelationship.md)> |  |  |

### Return type

[**crate::models::ObjectRelationship**](ObjectRelationship.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_relationship_batch

> put_object_relationship_batch(callback_url, body)


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

