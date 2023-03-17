use crate::models::{create_strategy::CreateStrategy, import_task::ImportStrategy};
use headless_common::url::Url;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
pub struct ImportTaskParams<'a, Field>
where
    Field: Display,
{
    pub callback_url: Option<&'a Url>,
    pub create_strategy: Option<CreateStrategy>,
    pub external_reference_code: Option<&'a str>,
    pub field_name_mapping: Option<FieldNameMap<'a, Field>>,
    pub import_strategy: Option<ImportStrategy>,
    pub task_item_delegate_name: Option<&'a str>,
}

impl<'a, Field> ImportTaskParams<'a, Field>
where
    Field: Display,
{
    pub fn new() -> Self {
        Self {
            field_name_mapping: None,
            ..Default::default()
        }
    }
}

impl<'a, Field> Default for ImportTaskParams<'a, Field>
where
    Field: Display,
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

#[derive(Debug, Clone, Default)]
pub struct FieldNameMap<'a, Field>(HashMap<&'a str, Field>)
where
    Field: Display;

impl<'a, Field> FieldNameMap<'a, Field>
where
    Field: Display,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<'a, Field> Deref for FieldNameMap<'a, Field>
where
    Field: Display,
{
    type Target = HashMap<&'a str, Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, Field> DerefMut for FieldNameMap<'a, Field>
where
    Field: Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, Field> Display for FieldNameMap<'a, Field>
where
    Field: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inter = self
            .0
            .iter()
            .map(|(key, val)| (key, val.to_string()))
            .collect::<HashMap<_, _>>();

        write!(f, "{}", serde_json::to_string(&inter).unwrap())
    }
}
