/*
 * Object
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.object.admin.rest.client', and version '1.0.45'.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectLayout {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "defaultObjectLayout", skip_serializing_if = "Option::is_none")]
    pub default_object_layout: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "objectDefinitionExternalReferenceCode", skip_serializing_if = "Option::is_none")]
    pub object_definition_external_reference_code: Option<String>,
    #[serde(rename = "objectDefinitionId", skip_serializing_if = "Option::is_none")]
    pub object_definition_id: Option<i64>,
    #[serde(rename = "objectLayoutTabs", skip_serializing_if = "Option::is_none")]
    pub object_layout_tabs: Option<Vec<crate::models::ObjectLayoutTab>>,
    #[serde(rename = "x-class-name", skip_serializing_if = "Option::is_none")]
    pub x_class_name: Option<String>,
}

impl ObjectLayout {
    pub fn new() -> ObjectLayout {
        ObjectLayout {
            actions: None,
            date_created: None,
            date_modified: None,
            default_object_layout: None,
            id: None,
            name: None,
            object_definition_external_reference_code: None,
            object_definition_id: None,
            object_layout_tabs: None,
            x_class_name: None,
        }
    }
}

