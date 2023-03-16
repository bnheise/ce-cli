# ObjectDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_entry_restricted** | Option<**bool**> |  | [optional]
**account_entry_restricted_object_field_name** | Option<**String**> |  | [optional]
**actions** | Option<[**::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>**](map.md)> |  | [optional][readonly]
**active** | Option<**bool**> |  | [optional]
**date_created** | Option<**String**> |  | [optional][readonly]
**date_modified** | Option<**String**> |  | [optional][readonly]
**default_language_id** | Option<**String**> |  | [optional]
**enable_categorization** | Option<**bool**> |  | [optional]
**enable_comments** | Option<**bool**> |  | [optional]
**enable_object_entry_history** | Option<**bool**> |  | [optional]
**external_reference_code** | Option<**String**> |  | [optional]
**id** | Option<**i64**> |  | [optional][readonly]
**label** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**object_actions** | Option<[**Vec<crate::models::ObjectAction>**](ObjectAction.md)> |  | [optional]
**object_fields** | Option<[**Vec<crate::models::ObjectField>**](ObjectField.md)> |  | [optional]
**object_layouts** | Option<[**Vec<crate::models::ObjectLayout>**](ObjectLayout.md)> |  | [optional]
**object_relationships** | Option<[**Vec<crate::models::ObjectRelationship>**](ObjectRelationship.md)> |  | [optional]
**object_views** | Option<[**Vec<crate::models::ObjectView>**](ObjectView.md)> |  | [optional]
**panel_app_order** | Option<**String**> |  | [optional]
**panel_category_key** | Option<**String**> |  | [optional]
**parameter_required** | Option<**bool**> |  | [optional][readonly]
**plural_label** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**portlet** | Option<**bool**> |  | [optional]
**rest_context_path** | Option<**String**> |  | [optional][readonly]
**scope** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::Status**](Status.md)> |  | [optional]
**storage_type** | Option<**String**> |  | [optional]
**system** | Option<**bool**> |  | [optional][readonly]
**title_object_field_name** | Option<**String**> |  | [optional]
**x_class_name** | Option<**String**> |  | [optional][readonly][default to com.liferay.object.admin.rest.dto.v1_0.ObjectDefinition]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


