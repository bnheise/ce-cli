use super::facet_value::FacetValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_criteria: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_values: Option<Vec<FacetValue>>,
}

impl Facet {
    pub fn new() -> Facet {
        Facet {
            facet_criteria: None,
            facet_values: None,
        }
    }
}
