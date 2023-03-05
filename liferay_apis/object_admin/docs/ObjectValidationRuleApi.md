# \ObjectValidationRuleApi

All URIs are relative to *http://localhost:8080/o/object-admin*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_validation_rule**](ObjectValidationRuleApi.md#delete_object_validation_rule) | **DELETE** /v1.0/object-validation-rules/{objectValidationRuleId} | 
[**delete_object_validation_rule_batch**](ObjectValidationRuleApi.md#delete_object_validation_rule_batch) | **DELETE** /v1.0/object-validation-rules/batch | 
[**get_object_definition_by_external_reference_code_object_validation_rules_page**](ObjectValidationRuleApi.md#get_object_definition_by_external_reference_code_object_validation_rules_page) | **GET** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-validation-rules | 
[**get_object_definition_object_validation_rules_page**](ObjectValidationRuleApi.md#get_object_definition_object_validation_rules_page) | **GET** /v1.0/object-definitions/{objectDefinitionId}/object-validation-rules | 
[**get_object_validation_rule**](ObjectValidationRuleApi.md#get_object_validation_rule) | **GET** /v1.0/object-validation-rules/{objectValidationRuleId} | 
[**patch_object_validation_rule**](ObjectValidationRuleApi.md#patch_object_validation_rule) | **PATCH** /v1.0/object-validation-rules/{objectValidationRuleId} | 
[**post_object_definition_by_external_reference_code_object_validation_rule**](ObjectValidationRuleApi.md#post_object_definition_by_external_reference_code_object_validation_rule) | **POST** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-validation-rules | 
[**post_object_definition_object_validation_rule**](ObjectValidationRuleApi.md#post_object_definition_object_validation_rule) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-validation-rules | 
[**post_object_definition_object_validation_rule_batch**](ObjectValidationRuleApi.md#post_object_definition_object_validation_rule_batch) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-validation-rules/batch | 
[**put_object_validation_rule**](ObjectValidationRuleApi.md#put_object_validation_rule) | **PUT** /v1.0/object-validation-rules/{objectValidationRuleId} | 
[**put_object_validation_rule_batch**](ObjectValidationRuleApi.md#put_object_validation_rule_batch) | **PUT** /v1.0/object-validation-rules/batch | 



## delete_object_validation_rule

> delete_object_validation_rule(object_validation_rule_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_validation_rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_validation_rule_batch

> delete_object_validation_rule_batch(callback_url, body)


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


## get_object_definition_by_external_reference_code_object_validation_rules_page

> crate::models::PageObjectValidationRule get_object_definition_by_external_reference_code_object_validation_rules_page(external_reference_code, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectValidationRule**](PageObjectValidationRule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_object_validation_rules_page

> crate::models::PageObjectValidationRule get_object_definition_object_validation_rules_page(object_definition_id, page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectValidationRule**](PageObjectValidationRule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_validation_rule

> crate::models::ObjectValidationRule get_object_validation_rule(object_validation_rule_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_validation_rule_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectValidationRule**](ObjectValidationRule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_object_validation_rule

> crate::models::ObjectValidationRule patch_object_validation_rule(object_validation_rule_id, object_validation_rule)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_validation_rule_id** | **String** |  | [required] |
**object_validation_rule** | Option<[**ObjectValidationRule**](ObjectValidationRule.md)> |  |  |

### Return type

[**crate::models::ObjectValidationRule**](ObjectValidationRule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_by_external_reference_code_object_validation_rule

> crate::models::ObjectValidationRule post_object_definition_by_external_reference_code_object_validation_rule(external_reference_code, object_validation_rule)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**object_validation_rule** | Option<[**ObjectValidationRule**](ObjectValidationRule.md)> |  |  |

### Return type

[**crate::models::ObjectValidationRule**](ObjectValidationRule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_validation_rule

> crate::models::ObjectValidationRule post_object_definition_object_validation_rule(object_definition_id, object_validation_rule)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**object_validation_rule** | Option<[**ObjectValidationRule**](ObjectValidationRule.md)> |  |  |

### Return type

[**crate::models::ObjectValidationRule**](ObjectValidationRule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_validation_rule_batch

> post_object_definition_object_validation_rule_batch(object_definition_id, callback_url, body)


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


## put_object_validation_rule

> crate::models::ObjectValidationRule put_object_validation_rule(object_validation_rule_id, object_validation_rule)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_validation_rule_id** | **String** |  | [required] |
**object_validation_rule** | Option<[**ObjectValidationRule**](ObjectValidationRule.md)> |  |  |

### Return type

[**crate::models::ObjectValidationRule**](ObjectValidationRule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_validation_rule_batch

> put_object_validation_rule_batch(callback_url, body)


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

