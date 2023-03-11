/*
 * Headless Batch Engine
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.headless.batch.engine.client', and version '1.0.11'.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`delete_import_task1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteImportTask1Error {
    DefaultResponse(crate::models::ImportTask),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_import_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImportTaskError {
    DefaultResponse(crate::models::ImportTask),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_import_task_by_external_reference_code`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImportTaskByExternalReferenceCodeError {
    DefaultResponse(crate::models::ImportTask),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_import_task_by_external_reference_code_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImportTaskByExternalReferenceCodeContentError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_import_task_by_external_reference_code_failed_item_report`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImportTaskByExternalReferenceCodeFailedItemReportError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_import_task_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImportTaskContentError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_import_task_failed_item_report`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImportTaskFailedItemReportError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_import_task1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostImportTask1Error {
    DefaultResponse(crate::models::ImportTask),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_import_task1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutImportTask1Error {
    DefaultResponse(crate::models::ImportTask),
    UnknownValue(serde_json::Value),
}


/// Uploads a new file for deleting items in batch.
pub fn delete_import_task1(configuration: &configuration::Configuration, class_name: &str, callback_url: Option<&str>, external_reference_code: Option<&str>, import_strategy: Option<&str>, task_item_delegate_name: Option<&str>) -> Result<crate::models::ImportTask, Error<DeleteImportTask1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/{className}", local_var_configuration.base_path, className=crate::apis::urlencode(class_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder = local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = external_reference_code {
        local_var_req_builder = local_var_req_builder.query(&[("externalReferenceCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = import_strategy {
        local_var_req_builder = local_var_req_builder.query(&[("importStrategy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = task_item_delegate_name {
        local_var_req_builder = local_var_req_builder.query(&[("taskItemDelegateName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteImportTask1Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the import task.
pub fn get_import_task(configuration: &configuration::Configuration, import_task_id: &str) -> Result<crate::models::ImportTask, Error<GetImportTaskError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/{importTaskId}", local_var_configuration.base_path, importTaskId=crate::apis::urlencode(import_task_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetImportTaskError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the import task by external reference code.
pub fn get_import_task_by_external_reference_code(configuration: &configuration::Configuration, external_reference_code: &str) -> Result<crate::models::ImportTask, Error<GetImportTaskByExternalReferenceCodeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/by-external-reference-code/{externalReferenceCode}", local_var_configuration.base_path, externalReferenceCode=crate::apis::urlencode(external_reference_code));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetImportTaskByExternalReferenceCodeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the exported content by external reference code.
pub fn get_import_task_by_external_reference_code_content(configuration: &configuration::Configuration, external_reference_code: &str) -> Result<(), Error<GetImportTaskByExternalReferenceCodeContentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/by-external-reference-code/{externalReferenceCode}/content", local_var_configuration.base_path, externalReferenceCode=crate::apis::urlencode(external_reference_code));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetImportTaskByExternalReferenceCodeContentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_import_task_by_external_reference_code_failed_item_report(configuration: &configuration::Configuration, external_reference_code: &str) -> Result<(), Error<GetImportTaskByExternalReferenceCodeFailedItemReportError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/by-external-reference-code/{externalReferenceCode}/failed-items/report", local_var_configuration.base_path, externalReferenceCode=crate::apis::urlencode(external_reference_code));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetImportTaskByExternalReferenceCodeFailedItemReportError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the exported content.
pub fn get_import_task_content(configuration: &configuration::Configuration, import_task_id: &str) -> Result<(), Error<GetImportTaskContentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/{importTaskId}/content", local_var_configuration.base_path, importTaskId=crate::apis::urlencode(import_task_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetImportTaskContentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_import_task_failed_item_report(configuration: &configuration::Configuration, import_task_id: &str) -> Result<(), Error<GetImportTaskFailedItemReportError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/{importTaskId}/failed-items/report", local_var_configuration.base_path, importTaskId=crate::apis::urlencode(import_task_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetImportTaskFailedItemReportError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uploads a new file for creating new items in batch.
pub fn post_import_task1(configuration: &configuration::Configuration, class_name: &str, callback_url: Option<&str>, create_strategy: Option<&str>, external_reference_code: Option<&str>, field_name_mapping: Option<&str>, import_strategy: Option<&str>, task_item_delegate_name: Option<&str>) -> Result<crate::models::ImportTask, Error<PostImportTask1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/{className}", local_var_configuration.base_path, className=crate::apis::urlencode(class_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder = local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = create_strategy {
        local_var_req_builder = local_var_req_builder.query(&[("createStrategy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = external_reference_code {
        local_var_req_builder = local_var_req_builder.query(&[("externalReferenceCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_name_mapping {
        local_var_req_builder = local_var_req_builder.query(&[("fieldNameMapping", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = import_strategy {
        local_var_req_builder = local_var_req_builder.query(&[("importStrategy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = task_item_delegate_name {
        local_var_req_builder = local_var_req_builder.query(&[("taskItemDelegateName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostImportTask1Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uploads a new file for updating items in batch.
pub fn put_import_task1(configuration: &configuration::Configuration, class_name: &str, callback_url: Option<&str>, external_reference_code: Option<&str>, import_strategy: Option<&str>, task_item_delegate_name: Option<&str>, update_strategy: Option<&str>) -> Result<crate::models::ImportTask, Error<PutImportTask1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/import-task/{className}", local_var_configuration.base_path, className=crate::apis::urlencode(class_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder = local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = external_reference_code {
        local_var_req_builder = local_var_req_builder.query(&[("externalReferenceCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = import_strategy {
        local_var_req_builder = local_var_req_builder.query(&[("importStrategy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = task_item_delegate_name {
        local_var_req_builder = local_var_req_builder.query(&[("taskItemDelegateName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = update_strategy {
        local_var_req_builder = local_var_req_builder.query(&[("updateStrategy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutImportTask1Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

