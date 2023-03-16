use crate::models::{create_strategy::CreateStrategy, import_task::ImportStrategy};
use headless_common::url::Url;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct ImportTaskParams<'a, T>
where
    T: Display,
{
    pub callback_url: Option<&'a Url>,
    pub create_strategy: Option<CreateStrategy>,
    pub external_reference_code: Option<&'a str>,
    pub field_name_mapping: Option<HashMap<T, &'a str>>,
    pub import_strategy: Option<ImportStrategy>,
    pub task_item_delegate_name: Option<&'a str>,
}

impl<'a, T> ImportTaskParams<'a, T>
where
    T: Display,
{
    pub fn new() -> Self {
        Self {
            field_name_mapping: None,
            ..Default::default()
        }
    }
}

impl<'a, T> Default for ImportTaskParams<'a, T>
where
    T: Display,
{
    fn default() -> Self {
        Self {
            callback_url: Default::default(),
            create_strategy: Default::default(),
            external_reference_code: Default::default(),
            field_name_mapping: Default::default(),
            import_strategy: Default::default(),
            task_item_delegate_name: Default::default(),
        }
    }
}
