/*
 * Object
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.object.admin.rest.client', and version '1.0.45'.
 *
 * The version of the OpenAPI document: v1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models::ObjectValidationRule};
use headless_common::{models::Page, reqwest};

/// struct for typed errors of method [`delete_object_validation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteObjectValidationRuleError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_object_validation_rule_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteObjectValidationRuleBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_object_definition_by_external_reference_code_object_validation_rules_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectDefinitionByExternalReferenceCodeObjectValidationRulesPageError {
    DefaultResponse(Page<ObjectValidationRule>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_object_definition_object_validation_rules_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectDefinitionObjectValidationRulesPageError {
    DefaultResponse(Page<ObjectValidationRule>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_object_validation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectValidationRuleError {
    DefaultResponse(Box<crate::models::ObjectValidationRule>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`patch_object_validation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchObjectValidationRuleError {
    DefaultResponse(Box<crate::models::ObjectValidationRule>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_object_definition_by_external_reference_code_object_validation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostObjectDefinitionByExternalReferenceCodeObjectValidationRuleError {
    DefaultResponse(Box<crate::models::ObjectValidationRule>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_object_definition_object_validation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostObjectDefinitionObjectValidationRuleError {
    DefaultResponse(Box<crate::models::ObjectValidationRule>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_object_definition_object_validation_rule_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostObjectDefinitionObjectValidationRuleBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_object_validation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutObjectValidationRuleError {
    DefaultResponse(Box<crate::models::ObjectValidationRule>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_object_validation_rule_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutObjectValidationRuleBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

pub fn delete_object_validation_rule(
    configuration: &configuration::Configuration,
    object_validation_rule_id: &str,
) -> Result<(), Error<DeleteObjectValidationRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-validation-rules/{objectValidationRuleId}",
        local_var_configuration.base_path,
        objectValidationRuleId = crate::apis::urlencode(object_validation_rule_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteObjectValidationRuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn delete_object_validation_rule_batch(
    configuration: &configuration::Configuration,
    callback_url: Option<&str>,
    body: Option<serde_json::Value>,
) -> Result<(), Error<DeleteObjectValidationRuleBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-validation-rules/batch",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteObjectValidationRuleBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_object_definition_by_external_reference_code_object_validation_rules_page(
    configuration: &configuration::Configuration,
    external_reference_code: &str,
    page: Option<&str>,
    page_size: Option<&str>,
    search: Option<&str>,
) -> Result<
    Page<ObjectValidationRule>,
    Error<GetObjectDefinitionByExternalReferenceCodeObjectValidationRulesPageError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-validation-rules", local_var_configuration.base_path, externalReferenceCode=crate::apis::urlencode(external_reference_code));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder =
            local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<
            GetObjectDefinitionByExternalReferenceCodeObjectValidationRulesPageError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_object_definition_object_validation_rules_page(
    configuration: &configuration::Configuration,
    object_definition_id: &str,
    page: Option<&str>,
    page_size: Option<&str>,
    search: Option<&str>,
) -> Result<Page<ObjectValidationRule>, Error<GetObjectDefinitionObjectValidationRulesPageError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-definitions/{objectDefinitionId}/object-validation-rules",
        local_var_configuration.base_path,
        objectDefinitionId = crate::apis::urlencode(object_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder =
            local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetObjectDefinitionObjectValidationRulesPageError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_object_validation_rule(
    configuration: &configuration::Configuration,
    object_validation_rule_id: &str,
) -> Result<crate::models::ObjectValidationRule, Error<GetObjectValidationRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-validation-rules/{objectValidationRuleId}",
        local_var_configuration.base_path,
        objectValidationRuleId = crate::apis::urlencode(object_validation_rule_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetObjectValidationRuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn patch_object_validation_rule(
    configuration: &configuration::Configuration,
    object_validation_rule_id: &str,
    object_validation_rule: Option<crate::models::ObjectValidationRule>,
) -> Result<crate::models::ObjectValidationRule, Error<PatchObjectValidationRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-validation-rules/{objectValidationRuleId}",
        local_var_configuration.base_path,
        objectValidationRuleId = crate::apis::urlencode(object_validation_rule_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_validation_rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PatchObjectValidationRuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn post_object_definition_by_external_reference_code_object_validation_rule(
    configuration: &configuration::Configuration,
    external_reference_code: &str,
    object_validation_rule: Option<crate::models::ObjectValidationRule>,
) -> Result<
    crate::models::ObjectValidationRule,
    Error<PostObjectDefinitionByExternalReferenceCodeObjectValidationRuleError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-validation-rules", local_var_configuration.base_path, externalReferenceCode=crate::apis::urlencode(external_reference_code));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_validation_rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<
            PostObjectDefinitionByExternalReferenceCodeObjectValidationRuleError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn post_object_definition_object_validation_rule(
    configuration: &configuration::Configuration,
    object_definition_id: &str,
    object_validation_rule: Option<crate::models::ObjectValidationRule>,
) -> Result<crate::models::ObjectValidationRule, Error<PostObjectDefinitionObjectValidationRuleError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-definitions/{objectDefinitionId}/object-validation-rules",
        local_var_configuration.base_path,
        objectDefinitionId = crate::apis::urlencode(object_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_validation_rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostObjectDefinitionObjectValidationRuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn post_object_definition_object_validation_rule_batch(
    configuration: &configuration::Configuration,
    object_definition_id: &str,
    callback_url: Option<&str>,
    body: Option<serde_json::Value>,
) -> Result<(), Error<PostObjectDefinitionObjectValidationRuleBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-definitions/{objectDefinitionId}/object-validation-rules/batch",
        local_var_configuration.base_path,
        objectDefinitionId = crate::apis::urlencode(object_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PostObjectDefinitionObjectValidationRuleBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn put_object_validation_rule(
    configuration: &configuration::Configuration,
    object_validation_rule_id: &str,
    object_validation_rule: Option<crate::models::ObjectValidationRule>,
) -> Result<crate::models::ObjectValidationRule, Error<PutObjectValidationRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-validation-rules/{objectValidationRuleId}",
        local_var_configuration.base_path,
        objectValidationRuleId = crate::apis::urlencode(object_validation_rule_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_validation_rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutObjectValidationRuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn put_object_validation_rule_batch(
    configuration: &configuration::Configuration,
    callback_url: Option<&str>,
    body: Option<serde_json::Value>,
) -> Result<(), Error<PutObjectValidationRuleBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-validation-rules/batch",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PutObjectValidationRuleBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
