use batch_api::{
    models::{import_task::ImportStrategy, ImportTask},
    reqwest::Url,
};
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
    models::{ListTypeDefinition, ListTypeEntry, PageListTypeEntry},
};
use object_admin::models::CreateStrategy;

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
        aggregation_terms: Option<&str>,
        filter: Option<&str>,
        page: Option<&str>,
        page_size: Option<&str>,
        search: Option<&str>,
        sort: Option<&str>,
        fields: Option<Vec<&str>>,
    ) -> Result<PageListTypeEntry, LiferayClientError<GetListTypeDefinitionListTypeEntriesPageError>>
    {
        let mut configuration = Configuration::new();
        configuration.update_base_path(self.base_path);
        configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));

        Ok(get_list_type_definition_list_type_entries_page(
            &configuration,
            list_type_definition_id,
            aggregation_terms,
            filter,
            page,
            page_size,
            search,
            sort,
            fields,
        )?)
    }

    pub fn post_list_type_definition_list_type_entry_batch(
        &self,
        list_type_definition_id: i64,
        list_type_entries: Vec<ListTypeEntry>,
        create_strategy: Option<object_admin::models::CreateStrategy>,
        import_strategy: Option<batch_api::models::import_task::ImportStrategy>,
        callback_url: Option<&str>,
    ) -> Result<ImportTask, LiferayClientError<PostListTypeDefinitionListTypeEntryBatchError>> {
        let mut configuration = Configuration::new();
        configuration.update_base_path(self.base_path);
        configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));

        let mut body = serde_json::to_value(list_type_entries)?;
        clean_json(&mut body);
        let list_type_definition_id = list_type_definition_id.to_string();

        Ok(post_list_type_definition_list_type_entry_batch(
            &configuration,
            &list_type_definition_id,
            callback_url,
            create_strategy,
            import_strategy,
            Some(body),
        )?)
    }

    pub fn put_list_type_entry_batch(
        &self,
        callback_url: Option<&str>,
        body: Option<Vec<ListTypeEntry>>,
        create_strategy: Option<CreateStrategy>,
        import_strategy: Option<ImportStrategy>,
    ) -> Result<ImportTask, LiferayClientError<PutListTypeEntryBatchError>> {
        let mut configuration = Configuration::new();
        configuration.update_base_path(self.base_path);
        configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));

        let body = serde_json::to_value(body)?;
        Ok(put_list_type_entry_batch(
            &configuration,
            callback_url,
            Some(body),
            create_strategy,
            import_strategy,
        )?)
    }

    // pub fn post_list_type_definition_batch(
    //     &self,
    //     body: Vec<ListTypeDefinition>,
    //     create_strategy: Option<object_admin::models::CreateStrategy>,
    //     import_strategy: Option<batch_api::models::import_task::ImportStrategy>,
    // ) -> Result<ImportTask, LiferayClientError<PostListTypeDefinitionBatchError>> {
    //     let mut configuration = Configuration::new();
    //     configuration.update_base_path(self.base_path);
    //     configuration.basic_auth = Some((self.username.to_owned(), Some(self.password.to_owned())));

    //     let mut body =
    //         serde_json::to_value(body).map_err(|e| LiferayClientError::Serialization {
    //             type_name: "ListTypeDefinition",
    //             origin: e,
    //         })?;

    //     clean_json(&mut body);

    //     println!("{}", serde_json::to_string(&body).unwrap_or_default());
    //     let res = post_list_type_definition_batch(
    //         &configuration,
    //         None,
    //         Some(body),
    //         create_strategy.or(Some(CreateStrategy::Upsert)),
    //         import_strategy.or(Some(ImportStrategy::Continue)),
    //     )?;

    //     Ok(res)
    // }
}
