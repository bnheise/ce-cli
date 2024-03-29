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
pub struct ObjectLayoutRow {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "objectLayoutColumns", skip_serializing_if = "Option::is_none")]
    pub object_layout_columns: Option<Vec<crate::models::ObjectLayoutColumn>>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "x-class-name", skip_serializing_if = "Option::is_none")]
    pub x_class_name: Option<String>,
}

impl ObjectLayoutRow {
    pub fn new() -> ObjectLayoutRow {
        ObjectLayoutRow {
            id: None,
            object_layout_columns: None,
            priority: None,
            x_class_name: None,
        }
    }
}


