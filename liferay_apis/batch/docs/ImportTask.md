# ImportTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | Option<**String**> | The item class name for which data will be processed in batch. | [optional]
**content_type** | Option<**String**> | The file content type. | [optional]
**end_time** | Option<**String**> | The end time of import task operation. | [optional]
**error_message** | Option<**String**> | The error message in case of import task's failed execution. | [optional]
**execute_status** | Option<**String**> | The status of import task's execution. | [optional]
**external_reference_code** | Option<**String**> | The optional external key of this account. | [optional]
**failed_items** | Option<[**Vec<crate::models::FailedItem>**](FailedItem.md)> |  | [optional]
**id** | Option<**i64**> | The task's ID. | [optional]
**import_strategy** | Option<**String**> | Defines if import task will fail when error occurs or continue importing rest of the items. | [optional]
**operation** | Option<**String**> | The operation of import task. | [optional]
**processed_items_count** | Option<**i32**> | Number of items processed by import task opeartion. | [optional]
**start_time** | Option<**String**> | The start time of import task operation. | [optional]
**total_items_count** | Option<**i32**> | Total number of items that will be processed by import task operation. | [optional]
**x_class_name** | Option<**String**> |  | [optional][readonly][default to com.liferay.headless.batch.engine.dto.v1_0.ImportTask]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


