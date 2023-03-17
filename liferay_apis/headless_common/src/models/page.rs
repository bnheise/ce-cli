use super::{action::Action, facet::Facet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A generic type to model any endpoint that supports pagination. These typically start with 'Page'.
/// For example, 'PageCountry' and 'PageRegion' in the Headless Admin Address API.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    /// The items returned in the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<T>>,
    /// The page containing the last items available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_page: Option<i64>,
    /// The total number of items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// The result of requested aggregation terms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<Facet>>,
    /// The current page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// The number of items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// A map of the available actions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<HashMap<String, Action>>,
}

impl<T> Page<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
