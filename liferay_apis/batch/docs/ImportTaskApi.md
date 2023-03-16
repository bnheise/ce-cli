# \ImportTaskApi

All URIs are relative to *http://localhost:8080/o/headless-batch-engine*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_import_task1**](ImportTaskApi.md#delete_import_task1) | **DELETE** /v1.0/import-task/{className} | 
[**get_import_task**](ImportTaskApi.md#get_import_task) | **GET** /v1.0/import-task/{importTaskId} | 
[**get_import_task_by_external_reference_code**](ImportTaskApi.md#get_import_task_by_external_reference_code) | **GET** /v1.0/import-task/by-external-reference-code/{externalReferenceCode} | 
[**get_import_task_by_external_reference_code_content**](ImportTaskApi.md#get_import_task_by_external_reference_code_content) | **GET** /v1.0/import-task/by-external-reference-code/{externalReferenceCode}/content | 
[**get_import_task_by_external_reference_code_failed_item_report**](ImportTaskApi.md#get_import_task_by_external_reference_code_failed_item_report) | **GET** /v1.0/import-task/by-external-reference-code/{externalReferenceCode}/failed-items/report | 
[**get_import_task_content**](ImportTaskApi.md#get_import_task_content) | **GET** /v1.0/import-task/{importTaskId}/content | 
[**get_import_task_failed_item_report**](ImportTaskApi.md#get_import_task_failed_item_report) | **GET** /v1.0/import-task/{importTaskId}/failed-items/report | 
[**post_import_task1**](ImportTaskApi.md#post_import_task1) | **POST** /v1.0/import-task/{className} | 
[**put_import_task1**](ImportTaskApi.md#put_import_task1) | **PUT** /v1.0/import-task/{className} | 



## delete_import_task1

> crate::models::ImportTask delete_import_task1(class_name, callback_url, external_reference_code, import_strategy, task_item_delegate_name)


Uploads a new file for deleting items in batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**class_name** | **String** |  | [required] |
**callback_url** | Option<**String**> |  |  |
**external_reference_code** | Option<**String**> |  |  |
**import_strategy** | Option<**String**> |  |  |
**task_item_delegate_name** | Option<**String**> |  |  |

### Return type

[**crate::models::ImportTask**](ImportTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_task

> crate::models::ImportTask get_import_task(import_task_id)


Retrieves the import task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_task_id** | **String** |  | [required] |

### Return type

[**crate::models::ImportTask**](ImportTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_task_by_external_reference_code

> crate::models::ImportTask get_import_task_by_external_reference_code(external_reference_code)


Retrieves the import task by external reference code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |

### Return type

[**crate::models::ImportTask**](ImportTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_task_by_external_reference_code_content

> get_import_task_by_external_reference_code_content(external_reference_code)


Retrieves the exported content by external reference code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_task_by_external_reference_code_failed_item_report

> get_import_task_by_external_reference_code_failed_item_report(external_reference_code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_task_content

> get_import_task_content(import_task_id)


Retrieves the exported content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_task_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_task_failed_item_report

> get_import_task_failed_item_report(import_task_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_task_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_import_task1

> crate::models::ImportTask post_import_task1(class_name, callback_url, create_strategy, external_reference_code, field_name_mapping, import_strategy, task_item_delegate_name)


Uploads a new file for creating new items in batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**class_name** | **String** |  | [required] |
**callback_url** | Option<**String**> |  |  |
**create_strategy** | Option<**String**> |  |  |
**external_reference_code** | Option<**String**> |  |  |
**field_name_mapping** | Option<**String**> |  |  |
**import_strategy** | Option<**String**> |  |  |
**task_item_delegate_name** | Option<**String**> |  |  |

### Return type

[**crate::models::ImportTask**](ImportTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_import_task1

> crate::models::ImportTask put_import_task1(class_name, callback_url, external_reference_code, import_strategy, task_item_delegate_name, update_strategy)


Uploads a new file for updating items in batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**class_name** | **String** |  | [required] |
**callback_url** | Option<**String**> |  |  |
**external_reference_code** | Option<**String**> |  |  |
**import_strategy** | Option<**String**> |  |  |
**task_item_delegate_name** | Option<**String**> |  |  |
**update_strategy** | Option<**String**> |  |  |

### Return type

[**crate::models::ImportTask**](ImportTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

