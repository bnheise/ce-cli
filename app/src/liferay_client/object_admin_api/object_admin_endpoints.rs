use headless_batch_engine::{
    apis::import_task_params::ImportTaskParams,
    models::{create_strategy::CreateStrategy, import_task::ImportStrategy, ImportTask},
};
use headless_common::url::Url;
use object_admin::{
    apis::{
        configuration::Configuration,
        object_definition_api::{post_object_definition_batch, PostObjectDefinitionBatchError},
    },
    models::{object_definition::ObjectDefinitionField, ObjectDefinition},
};

use crate::{
    commands::object::ApiConfig, liferay_client::liferay_client_error::LiferayClientError,
};

pub struct ObjectAdminEndpoints<'a> {
    base_path: &'a Url,
    username: &'a str,
    password: &'a str,
}

impl<'a> ObjectAdminEndpoints<'a> {
    pub fn new(base_path: &'a Url, username: &'a str, password: &'a str) -> Self {
        Self {
            base_path,
            username,
            password,
        }
    }

    pub fn post_object_definition_batch(
        &self,
        body: Vec<ObjectDefinition>,
        options: Option<ImportTaskParams<ObjectDefinitionField>>,
    ) -> Result<ImportTask, LiferayClientError<PostObjectDefinitionBatchError>> {
        let mut configuration = Configuration::new();
        configuration.update_base_path(self.base_path);
        configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));

        let body = serde_json::to_value(body).map_err(|e| LiferayClientError::Serialization {
            type_name: "ObjectDefinition",
            origin: e,
        })?;

        let res = post_object_definition_batch(&configuration, Some(&body), options)?;

        Ok(res)
    }
}
