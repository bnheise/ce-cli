use super::{field_collection::FieldCollection, filter::FilterExpression, sort::SortCollection};
use std::fmt::Display;

/// Params common to Page requests
#[derive(Clone, Debug, PartialEq, Default)]

pub struct PageParams<'a, T, S>
where
    T: Display,
    S: AsRef<str> + Display,
{
    /// A list of field names to aggregate
    pub agregation_terms: Option<FieldCollection<T>>,
    /// A set of filter expressions to limit what data is returned
    pub filter: Option<FilterExpression<'a, T, S>>,
    /// The page number you want to display
    pub page: Option<i64>,
    /// The number of items you want to display on a page
    pub page_size: Option<i64>,
    /// A string of characters to seach for
    pub search: Option<S>,
    /// The fields you want to sort on and the sort order, ascending or descending
    pub sort: Option<SortCollection<'a>>,
    /// The names of the children that you want to display
    pub nested_fields: Option<Vec<S>>,
    /// The depth of children that you want to display. The maximum is 5.
    pub nested_fields_depth: Option<i64>,
    /// Disregard hierarchical classifications.
    pub flatten: Option<bool>,
    /// Restrict the fields to be returned by adding the field names to this
    /// collection
    pub fields: Option<FieldCollection<T>>,
}
