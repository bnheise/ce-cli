use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

/// Intended to hold a collection of struct fields. Used for the fields parameter
/// of [PageParams](super::page_params::PageParams)
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FieldCollection<T>(Vec<T>)
where
    T: Display;

impl<T> From<Vec<T>> for FieldCollection<T>
where
    T: Display,
{
    fn from(value: Vec<T>) -> Self {
        FieldCollection(value)
    }
}

impl<T> FieldCollection<T>
where
    T: Display,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> Deref for FieldCollection<T>
where
    T: Display,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for FieldCollection<T>
where
    T: Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Display for FieldCollection<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(T::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
