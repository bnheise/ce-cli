use headless_admin_list_type::{
    apis::{
        configuration::Configuration,
        list_type_definition_api::{
            put_list_type_definition_by_external_reference_code, PutListTypeDefinitionError,
        },
        list_type_entry_api::{
            get_list_type_definition_list_type_entries_page,
            post_list_type_definition_list_type_entry_batch, put_list_type_entry_batch,
            GetListTypeDefinitionListTypeEntriesPageError,
            PostListTypeDefinitionListTypeEntryBatchError, PutListTypeEntryBatchError,
        },
    },
    models::{list_type_entry::ListTypeEntryField, ListTypeDefinition, ListTypeEntry},
};
use headless_batch_engine::{
    apis::import_task_params::ImportTaskParams,
    models::{create_strategy::CreateStrategy, import_task::ImportStrategy, ImportTask},
};
use headless_common::{api::page_params::PageParams, models::Page, url::Url};

use crate::{
    commands::object::ApiConfig,
    liferay_client::{clean_json, liferay_client_error::LiferayClientError},
};

pub struct ListTypeEndpoints<'a> {
    base_path: &'a Url,
    username: &'a str,
    password: &'a str,
}

impl<'a> ListTypeEndpoints<'a> {
    pub fn new(base_path: &'a Url, username: &'a str, password: &'a str) -> Self {
        Self {
            base_path,
            username,
            password,
        }
    }

    pub fn put_list_type_definition(
        &self,
        external_reference_code: &str,
        mut list_type_definition: Option<ListTypeDefinition>,
    ) -> Result<ListTypeDefinition, LiferayClientError<PutListTypeDefinitionError>> {
        let mut configuration = Configuration::new();
        configuration.update_base_path(self.base_path);
        configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));
        list_type_definition.as_mut().map(|list| {
            list.id = None;
            list
        });
        Ok(put_list_type_definition_by_external_reference_code(
            &configuration,
            external_reference_code,
            list_type_definition,
        )?)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_list_type_definition_list_type_entries_page(
        &self,
        list_type_definition_id: &str,
        options: Option<PageParams<ListTypeEntryField>>,
    ) -> Result<
        Page<ListTypeEntry>,
        LiferayClientError<GetListTypeDefinitionListTypeEntriesPageError>,
    > {
        let mut configuration = Configuration::new();
        configuration.update_base_path(self.base_path);
        configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));

        Ok(get_list_type_definition_list_type_entries_page(
            &configuration,
            list_type_definition_id,
            options,
        )?)
    }

    pub fn post_list_type_definition_list_type_entry_batch(
        &self,
        list_type_definition_id: i64,
        list_type_entries: Vec<ListTypeEntry>,
        create_strategy: Option<CreateStrategy>,
        import_strategy: Option<ImportStrategy>,
        callback_url: Option<&Url>,
    ) -> Result<ImportTask, LiferayClientError<PostListTypeDefinitionListTypeEntryBatchError>> {
        let mut configuration = Configuration::new();
        configuration.update_base_path(self.base_path);
        configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));

        let mut body = serde_json::to_value(list_type_entries)?;
        clean_json(&mut body);
        let body = serde_json::from_value::<Vec<ListTypeEntry>>(body)?;
        let list_type_definition_id = list_type_definition_id.to_string();
        let mut options = ImportTaskParams::new();
        options.create_strategy = create_strategy;
        options.import_strategy = import_strategy;
        options.callback_url = callback_url;

        Ok(post_list_type_definition_list_type_entry_batch(
            &configuration,
            &list_type_definition_id,
            body,
            options,
        )?)
    }

    pub fn put_list_type_entry_batch(
        &self,
        callback_url: Option<&Url>,
        body: Option<Vec<ListTypeEntry>>,
        create_strategy: Option<CreateStrategy>,
        import_strategy: Option<ImportStrategy>,
    ) -> Result<ImportTask, LiferayClientError<PutListTypeEntryBatchError>> {
        let mut configuration = Configuration::new();
        configuration.update_base_path(self.base_path);
        configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));

        let mut options = ImportTaskParams::new();
        options.create_strategy = create_strategy;
        options.callback_url = callback_url;
        options.import_strategy = import_strategy;
        Ok(put_list_type_entry_batch(&configuration, body, options)?)
    }
}
