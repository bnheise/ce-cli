use std::fmt::Display;

/*
 * List Type
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'om.liferay.headless.admin.list.type.client', and version '1.0.0'.. A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.headless.admin.list.type.client', and version '1.0.11'.
 *
 * The version of the OpenAPI document: v1.0
 *
 * Generated by: https://openapi-generator.tech
 */
use headless_common::convert_case::Casing;
use headless_common::{convert_case::Case, field_types::FieldName};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, FieldName)]
pub struct ListTypeEntry {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions:
        Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "name_i18n", skip_serializing_if = "Option::is_none")]
    pub name_i18n: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "x-class-name", skip_serializing_if = "Option::is_none")]
    pub x_class_name: Option<String>,
    #[serde(
        rename = "externalReferenceCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_code: Option<String>,
}

impl ListTypeEntry {
    pub fn new() -> ListTypeEntry {
        ListTypeEntry {
            actions: None,
            date_created: None,
            date_modified: None,
            id: None,
            key: None,
            name: None,
            name_i18n: None,
            r#type: None,
            x_class_name: None,
            external_reference_code: None,
        }
    }
}

impl ListTypeEntryFieldName {
    pub const NAME: &'static str = "ListTypeEntryFieldName";
}

impl Display for ListTypeEntryFieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().to_case(Case::Camel))
    }
}

impl Serialize for ListTypeEntryFieldName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ListTypeEntryFieldName::Actions => {
                serializer.serialize_unit_variant(Self::NAME, 0, "actions")
            }
            ListTypeEntryFieldName::DateCreated => {
                serializer.serialize_unit_variant(Self::NAME, 1, "dateCreated")
            }
            ListTypeEntryFieldName::DateModified => {
                serializer.serialize_unit_variant(Self::NAME, 2, "dateModified")
            }
            ListTypeEntryFieldName::Id => serializer.serialize_unit_variant(Self::NAME, 3, "id"),
            ListTypeEntryFieldName::Key => serializer.serialize_unit_variant(Self::NAME, 4, "key"),
            ListTypeEntryFieldName::Name => {
                serializer.serialize_unit_variant(Self::NAME, 5, "name")
            }
            ListTypeEntryFieldName::NameI18n => {
                serializer.serialize_unit_variant(Self::NAME, 6, "nameI18n")
            }
            ListTypeEntryFieldName::RType => {
                serializer.serialize_unit_variant(Self::NAME, 7, "type")
            }
            ListTypeEntryFieldName::XClassName => {
                serializer.serialize_unit_variant(Self::NAME, 8, "x-class-name")
            }
            ListTypeEntryFieldName::ExternalReferenceCode => {
                serializer.serialize_unit_variant(Self::NAME, 9, "externalReferenceCode")
            }
        }
    }
}

use serde::Serialize;
pub use ListTypeEntryFieldName as ListTypeEntryField;
