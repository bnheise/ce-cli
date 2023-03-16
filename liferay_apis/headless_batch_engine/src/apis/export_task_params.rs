use std::fmt::Display;

use headless_common::{api::field_collection::FieldCollection, url::Url};

#[derive(Debug, Clone)]
pub struct ExportTaskParams<'a, T>
where
    T: Display,
{
    pub callback_url: Option<&'a Url>,
    pub external_reference_code: Option<&'a str>,
    pub field_names: Option<&'a FieldCollection<T>>,
    pub task_item_delegate_name: Option<&'a str>,
}

impl<'a, T> ExportTaskParams<'a, T>
where
    T: Display,
{
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<'a, T> Default for ExportTaskParams<'a, T>
where
    T: Display,
{
    fn default() -> Self {
        Self {
            callback_url: Default::default(),
            external_reference_code: Default::default(),
            field_names: None,
            task_item_delegate_name: Default::default(),
        }
    }
}
