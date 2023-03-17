use super::super::models::list_type_entry::ListTypeEntryFieldName;
use super::{configuration, Error};
use crate::{apis::ResponseContent, models::ListTypeEntry};
use headless_batch_engine::apis::import_task_params::ImportTaskParams;
use headless_batch_engine::models::ImportTask;
use headless_common::{api::page_params::PageParams, models::Page, reqwest};

/// struct for typed errors of method [`delete_list_type_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteListTypeEntryError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_list_type_entry_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteListTypeEntryBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_list_type_definition_list_type_entries_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetListTypeDefinitionListTypeEntriesPageError {
    DefaultResponse(Page<ListTypeEntry>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_list_type_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetListTypeEntryError {
    DefaultResponse(Box<crate::models::ListTypeEntry>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_list_type_definition_list_type_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostListTypeDefinitionListTypeEntryError {
    DefaultResponse(Box<crate::models::ListTypeEntry>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_list_type_definition_list_type_entry_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostListTypeDefinitionListTypeEntryBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_list_type_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutListTypeEntryError {
    DefaultResponse(Box<crate::models::ListTypeEntry>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_list_type_entry_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutListTypeEntryBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

pub fn delete_list_type_entry(
    configuration: &configuration::Configuration,
    list_type_entry_id: &str,
) -> Result<(), Error<DeleteListTypeEntryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/list-type-entries/{listTypeEntryId}",
        local_var_configuration.base_path,
        listTypeEntryId = crate::apis::urlencode(list_type_entry_id)
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
        let local_var_entity: Option<DeleteListTypeEntryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn delete_list_type_entry_batch(
    configuration: &configuration::Configuration,
    callback_url: Option<&str>,
    body: Option<Vec<ListTypeEntry>>,
) -> Result<(), Error<DeleteListTypeEntryBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/list-type-entries/batch",
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
        let local_var_entity: Option<DeleteListTypeEntryBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_list_type_definition_list_type_entries_page(
    configuration: &configuration::Configuration,
    list_type_definition_id: &str,
    options: Option<PageParams<ListTypeEntryFieldName>>,
) -> Result<Page<ListTypeEntry>, Error<GetListTypeDefinitionListTypeEntriesPageError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/list-type-definitions/{listTypeDefinitionId}/list-type-entries",
        local_var_configuration.base_path,
        listTypeDefinitionId = crate::apis::urlencode(list_type_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(options) = options {
        if let Some(ref local_var_str) = options.agregation_terms {
            local_var_req_builder =
                local_var_req_builder.query(&[("aggregationTerms", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = options.filter {
            local_var_req_builder =
                local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = options.page {
            local_var_req_builder =
                local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = options.page_size {
            local_var_req_builder =
                local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = options.search {
            local_var_req_builder =
                local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = options.sort {
            local_var_req_builder =
                local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_vec) = options.fields {
            local_var_req_builder =
                local_var_req_builder.query(&[("fields", local_var_vec.to_string())]);
        }
        if let Some(ref local_var_vec) = options.flatten {
            local_var_req_builder =
                local_var_req_builder.query(&[("flatten", local_var_vec.to_string())]);
        }
        if let Some(ref local_var_vec) = options.nested_fields_depth {
            local_var_req_builder =
                local_var_req_builder.query(&[("nestedFieldsDepth", local_var_vec.to_string())]);
        }
        if let Some(ref local_var_vec) = options.nested_fields {
            local_var_req_builder =
                local_var_req_builder.query(&[("nestedFields", local_var_vec.to_string())]);
        }
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
        let local_var_entity: Option<GetListTypeDefinitionListTypeEntriesPageError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn get_list_type_entry(
    configuration: &configuration::Configuration,
    list_type_entry_id: &str,
) -> Result<crate::models::ListTypeEntry, Error<GetListTypeEntryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/list-type-entries/{listTypeEntryId}",
        local_var_configuration.base_path,
        listTypeEntryId = crate::apis::urlencode(list_type_entry_id)
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
        let local_var_entity: Option<GetListTypeEntryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn post_list_type_definition_list_type_entry(
    configuration: &configuration::Configuration,
    list_type_definition_id: &str,
    list_type_entry: Option<crate::models::ListTypeEntry>,
) -> Result<crate::models::ListTypeEntry, Error<PostListTypeDefinitionListTypeEntryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/list-type-definitions/{listTypeDefinitionId}/list-type-entries",
        local_var_configuration.base_path,
        listTypeDefinitionId = crate::apis::urlencode(list_type_definition_id)
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
    local_var_req_builder = local_var_req_builder.json(&list_type_entry);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostListTypeDefinitionListTypeEntryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn post_list_type_definition_list_type_entry_batch(
    configuration: &configuration::Configuration,
    list_type_definition_id: &str,
    body: Vec<ListTypeEntry>,
    options: ImportTaskParams<ListTypeEntryFieldName>,
) -> Result<ImportTask, Error<PostListTypeDefinitionListTypeEntryBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/list-type-definitions/{listTypeDefinitionId}/list-type-entries/batch",
        local_var_configuration.base_path,
        listTypeDefinitionId = crate::apis::urlencode(list_type_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = options.callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.create_strategy {
        local_var_req_builder =
            local_var_req_builder.query(&[("createStrategy", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.import_strategy {
        local_var_req_builder =
            local_var_req_builder.query(&[("importStrategy", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.external_reference_code {
        local_var_req_builder =
            local_var_req_builder.query(&[("externalReferenceCode", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.field_name_mapping {
        local_var_req_builder =
            local_var_req_builder.query(&[("fieldNameMapping", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.task_item_delegate_name {
        local_var_req_builder = local_var_req_builder.query(&[(
            "taskItemDelegateName",
            serde_json::to_string(local_var_str)?,
        )]);
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostListTypeDefinitionListTypeEntryBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn put_list_type_entry(
    configuration: &configuration::Configuration,
    list_type_entry_id: &str,
    list_type_entry: Option<crate::models::ListTypeEntry>,
) -> Result<crate::models::ListTypeEntry, Error<PutListTypeEntryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/list-type-entries/{listTypeEntryId}",
        local_var_configuration.base_path,
        listTypeEntryId = crate::apis::urlencode(list_type_entry_id)
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
    local_var_req_builder = local_var_req_builder.json(&list_type_entry);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutListTypeEntryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn put_list_type_entry_batch(
    configuration: &configuration::Configuration,
    body: Option<Vec<ListTypeEntry>>,
    options: ImportTaskParams<ListTypeEntryFieldName>,
) -> Result<ImportTask, Error<PutListTypeEntryBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/list-type-entries/batch",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = options.callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.create_strategy {
        local_var_req_builder =
            local_var_req_builder.query(&[("createStrategy", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.import_strategy {
        local_var_req_builder =
            local_var_req_builder.query(&[("importStrategy", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.external_reference_code {
        local_var_req_builder =
            local_var_req_builder.query(&[("externalReferenceCode", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.field_name_mapping {
        local_var_req_builder =
            local_var_req_builder.query(&[("fieldNameMapping", local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = options.task_item_delegate_name {
        local_var_req_builder = local_var_req_builder.query(&[(
            "taskItemDelegateName",
            serde_json::to_string(local_var_str)?,
        )]);
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutListTypeEntryBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
