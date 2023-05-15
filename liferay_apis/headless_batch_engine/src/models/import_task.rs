/*
 * Headless Batch Engine
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.headless.batch.engine.client', and version '1.0.11'.
 *
 * The version of the OpenAPI document: v1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImportTask {
    /// The item class name for which data will be processed in batch.
    #[serde(rename = "className", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// The file content type.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The end time of import task operation.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The error message in case of import task's failed execution.
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The status of import task's execution.
    #[serde(rename = "executeStatus", skip_serializing_if = "Option::is_none")]
    pub execute_status: Option<ExecuteStatus>,
    /// The optional external key of this account.
    #[serde(
        rename = "externalReferenceCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_code: Option<String>,
    #[serde(rename = "failedItems", skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<crate::models::FailedItem>>,
    /// The task's ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Defines if import task will fail when error occurs or continue importing rest of the items.
    #[serde(rename = "importStrategy", skip_serializing_if = "Option::is_none")]
    pub import_strategy: Option<ImportStrategy>,
    /// The operation of import task.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    /// Number of items processed by import task opeartion.
    #[serde(
        rename = "processedItemsCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub processed_items_count: Option<i32>,
    /// The start time of import task operation.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Total number of items that will be processed by import task operation.
    #[serde(rename = "totalItemsCount", skip_serializing_if = "Option::is_none")]
    pub total_items_count: Option<i32>,
    #[serde(rename = "x-class-name", skip_serializing_if = "Option::is_none")]
    pub x_class_name: Option<String>,
}

impl ImportTask {
    pub fn new() -> ImportTask {
        ImportTask {
            class_name: None,
            content_type: None,
            end_time: None,
            error_message: None,
            execute_status: None,
            external_reference_code: None,
            failed_items: None,
            id: None,
            import_strategy: None,
            operation: None,
            processed_items_count: None,
            start_time: None,
            total_items_count: None,
            x_class_name: None,
        }
    }
}

/// The status of import task's execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExecuteStatus {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "STARTED")]
    Started,
}

impl Default for ExecuteStatus {
    fn default() -> ExecuteStatus {
        Self::Completed
    }
}
/// Defines if import task will fail when error occurs or continue importing rest of the items.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImportStrategy {
    #[serde(rename = "ON_ERROR_CONTINUE")]
    Continue,
    #[serde(rename = "ON_ERROR_FAIL")]
    Fail,
}

impl Display for ImportStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportStrategy::Continue => write!(f, "ON_ERROR_CONTINUE"),
            ImportStrategy::Fail => write!(f, "ON_ERROR_FAIL"),
        }
    }
}

impl Default for ImportStrategy {
    fn default() -> ImportStrategy {
        Self::Continue
    }
}
/// The operation of import task.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "CREATE")]
    Create,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "UPDATE")]
    Update,
}

impl Default for Operation {
    fn default() -> Operation {
        Self::Create
    }
}