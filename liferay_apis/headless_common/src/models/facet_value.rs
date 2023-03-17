use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacetValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_occurrences: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}

impl FacetValue {
    pub fn new() -> FacetValue {
        FacetValue {
            number_of_occurrences: None,
            term: None,
        }
    }
}
