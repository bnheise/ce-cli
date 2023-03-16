use std::{fs, path::Path};

use headless_admin_list_type::models::ListTypeDefinition;
use object_admin::models::{object_field::BusinessType, ObjectDefinition, ObjectRelationship};

use crate::{error::CliError, liferay_client::clean_json};

pub struct DataDir<'a> {
    base_data_dir: &'a str,
}

impl<'a> DataDir<'a> {
    pub fn init(base_data_dir: &'a str) -> Self {
        Self { base_data_dir }
    }

    pub fn load_picklist_data(&self) -> Result<Vec<ListTypeDefinition>, CliError> {
        let piclist_path = Path::new(self.base_data_dir).join("picklists");

        Ok(fs::read_dir(piclist_path)?
            .flat_map(|entry| {
                let entry = entry.map_err(|e| CliError::FileSystemError(e.to_string()))?;
                let path = entry.path();

                let content = fs::read_to_string(path)?;
                Ok::<ListTypeDefinition, CliError>(serde_json::from_str::<ListTypeDefinition>(
                    &content,
                )?)
            })
            .collect::<Vec<_>>())
    }

    pub fn load_object_data(&self) -> Result<Vec<serde_json::Value>, CliError> {
        let data_path = Path::new(self.base_data_dir).join("data");

        Ok(fs::read_dir(data_path)?
            .flat_map(|entry| {
                let entry = entry.map_err(|e| CliError::FileSystemError(e.to_string()))?;
                let path = entry.path();

                let content = fs::read_to_string(path)?;
                Ok::<serde_json::Value, CliError>(serde_json::from_str::<serde_json::Value>(
                    &content,
                )?)
            })
            .collect::<Vec<_>>())
    }

    pub fn load_object_definitions(
        &self,
        clean_for_post: bool,
    ) -> Result<Vec<ObjectDefinition>, CliError> {
        let definitions_path = Path::new(self.base_data_dir).join("definitions");
        let mut object_definitions = Vec::new();

        for entry in fs::read_dir(definitions_path)? {
            let entry = entry?;
            let path = entry.path();
            let file = fs::read_to_string(&path)?;

            let mut as_value = serde_json::from_str::<serde_json::Value>(&file)?;
            if clean_for_post {
                clean_json(&mut as_value);
            }

            let mut object_def = serde_json::from_value::<ObjectDefinition>(as_value)?;

            if clean_for_post {
                object_def.actions = None;
                object_def.active = None;
                object_def.date_created = None;
                object_def.date_modified = None;
                Self::clean_fields(&mut object_def);
                Self::clean_relationships(&mut object_def.object_relationships)
            }

            let erc = object_def
                .external_reference_code
                .clone()
                .unwrap_or_default();
            println!("Found definition: {erc}");
            object_definitions.push(object_def);
        }
        Ok(object_definitions)
    }

    fn clean_fields(object_def: &mut ObjectDefinition) {
        object_def.object_fields = object_def.object_fields.as_mut().map(|fields| {
            fields
                .iter_mut()
                .filter(|field| {
                    if let Some(business_type) = field.business_type {
                        business_type != BusinessType::Relationship
                    } else {
                        true
                    }
                })
                .map(|field| field.to_owned())
                .collect::<Vec<_>>()
        });
    }

    fn clean_relationships(relationships: &mut Option<Vec<ObjectRelationship>>) {
        if let Some(relationships) = relationships.as_mut() {
            relationships.iter_mut().for_each(|mut relation| {
                relation.object_definition_id1 = None;
                relation.object_definition_id2 = None;
                relation.id = None;
                relation.parameter_object_field_id = None;
                relation.actions = None;
            })
        }
    }
}
