use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SortCollection<'a>(Vec<SortParameter<'a>>);

impl<'a> SortCollection<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<'a> Deref for SortCollection<'a> {
    type Target = Vec<SortParameter<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for SortCollection<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Params common to Page requests
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SortParameter<'a> {
    field: &'a str,
    sort_order: SortOrder,
}

impl<'a> SortParameter<'a> {
    pub fn new(field: &'a str) -> Self {
        Self {
            field,
            ..Default::default()
        }
    }

    pub fn set_sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = sort_order;
        self
    }

    pub fn set_descending(mut self) -> Self {
        self.sort_order = SortOrder::Descending;
        self
    }

    pub fn set_ascending(mut self) -> Self {
        self.sort_order = SortOrder::Ascending;
        self
    }
}

impl<'a> Display for SortParameter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.field, self.sort_order)
    }
}

impl<'a> Display for SortCollection<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascending => write!(f, "asc"),
            Self::Descending => write!(f, "desc"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::api::sort::{SortCollection, SortParameter};

    #[test]
    fn single() {
        let mut coll = SortCollection::new();
        coll.push(SortParameter::new("field1"));
        let actual = coll.to_string();
        let expected = "field1:asc";
        assert_eq!(actual, expected);
    }

    #[test]
    fn list() {
        let mut coll = SortCollection::new();
        coll.push(SortParameter::new("field1"));
        coll.push(SortParameter::new("field2").set_descending());
        let actual = coll.to_string();
        let expected = "field1:asc,field2:desc";
        assert_eq!(actual, expected);
    }
}
