use chrono::Utc;
use std::{fmt::Display, num::ParseIntError};

/// Used to recursively model filter expressions and render them as strings.
#[derive(Clone, Debug, PartialEq)]
pub enum FilterExpression<'a, T, S>
where
    T: Display,
    S: AsRef<str>,
{
    Equal(T, FilterValue<S>),
    NotEqual(T, FilterValue<S>),
    GreaterThan(T, FilterValue<S>),
    GreaterOrEqual(T, FilterValue<S>),
    LessThan(T, FilterValue<S>),
    LessOrEqual(T, FilterValue<S>),
    StartsWith(T, FilterValue<S>),
    Not(&'a FilterExpression<'a, T, S>),
    And(
        &'a FilterExpression<'a, T, S>,
        &'a FilterExpression<'a, T, S>,
    ),
    Or(
        &'a FilterExpression<'a, T, S>,
        &'a FilterExpression<'a, T, S>,
    ),
    Contains {
        field: T,
        value: S,
    },
    Group(&'a FilterExpression<'a, T, S>),
    Custom(T),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FilterValue<T>
where
    T: AsRef<str>,
{
    String(T),
    Number(i64),
    Float(f64),
    DateTime(Utc),
    #[default]
    Null,
    Custom(String),
}

impl<T> From<i64> for FilterValue<T>
where
    T: AsRef<str>,
{
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl<T> TryFrom<String> for FilterValue<T>
where
    T: AsRef<str>,
{
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::Number(value.parse::<i64>()?))
    }
}

impl<T> Display for FilterValue<T>
where
    T: AsRef<str> + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(val) => write!(f, "'{val}'"),
            Self::Number(val) => write!(f, "{val}"),
            Self::Float(val) => write!(f, "{val}"),
            Self::DateTime(val) => write!(f, "{val}"),
            Self::Null => write!(f, "null"),
            Self::Custom(val) => write!(f, "{val}"),
        }
    }
}

impl<'a, T, S> Display for FilterExpression<'a, T, S>
where
    T: Display,
    S: AsRef<str> + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Equal(field, value) => write!(f, "{field} eq {value}"),
            Self::NotEqual(field, value) => write!(f, "{field} ne {value}"),
            Self::GreaterThan(field, value) => write!(f, "{field} gt {value}"),
            Self::GreaterOrEqual(field, value) => write!(f, "{field} ge {value}"),
            Self::LessThan(field, value) => write!(f, "{field} lt {value}"),
            Self::LessOrEqual(field, value) => write!(f, "{field} le {value}"),
            Self::StartsWith(field, value) => {
                write!(f, "startsWith({field}, {value})")
            }
            Self::Not(op) => write!(f, "not ({})", op),
            Self::And(left, right) => write!(f, "{} and {}", left, right),
            Self::Or(left, right) => write!(f, "{} or {}", left, right),
            Self::Contains { field, value } => {
                write!(f, "contains({field}, '{value}')")
            }
            Self::Group(op) => {
                write!(f, "({})", op)
            }
            Self::Custom(custom) => write!(f, "{custom}"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use convert_case::Case;
    use convert_case::Casing;
    use field_types::FieldName;

    use super::{FilterExpression, FilterValue};

    #[derive(Debug, FieldName)]
    #[allow(dead_code)]
    struct TestStruct {
        field_1: String,
        field_2: String,
    }

    impl Display for TestStructFieldName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.name().to_case(Case::Camel))
        }
    }

    #[test]
    fn string_test1() {
        let actual =
            FilterExpression::Equal(&TestStructFieldName::Field1, FilterValue::String("hi"))
                .to_string();
        let expected = "field1 eq 'hi'";
        assert_eq!(actual, expected);
    }

    #[test]
    fn string_test2() {
        let actual = FilterExpression::Not(&FilterExpression::Contains {
            field: "field2",
            value: "chicken",
        })
        .to_string();
        let expected = "not (contains(field2, 'chicken'))";
        assert_eq!(actual, expected);
    }

    #[test]
    fn and() {
        let actual = FilterExpression::And(
            &FilterExpression::Not(&FilterExpression::Contains {
                field: "field2",
                value: "chicken",
            }),
            &FilterExpression::Equal("field1", FilterValue::String("hi")),
        )
        .to_string();
        let expected = "not (contains(field2, 'chicken')) and field1 eq 'hi'";
        assert_eq!(actual, expected);
    }

    #[test]
    fn group() {
        let actual = FilterExpression::Group(&FilterExpression::And(
            &FilterExpression::Not(&FilterExpression::Contains {
                field: "field2",
                value: "chicken",
            }),
            &FilterExpression::Equal("field1", FilterValue::String("hi")),
        ))
        .to_string();
        let expected = "(not (contains(field2, 'chicken')) and field1 eq 'hi')";
        assert_eq!(actual, expected);
    }
}
