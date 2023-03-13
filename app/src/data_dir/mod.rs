use std::{fs, path::Path};

use object_admin::models::{object_field::BusinessType, ObjectDefinition};

use crate::error::CliError;

pub struct DataDir<'a> {
    base_data_dir: &'a str,
}

impl<'a> DataDir<'a> {
    pub fn init(base_data_dir: &'a str) -> Self {
        Self { base_data_dir }
    }

    pub fn load_object_definitions(
        &self,
        clean_for_post: bool,
    ) -> Result<Vec<ObjectDefinition>, CliError> {
        println!("Loading object definitions...");

        let definitions_path = Path::new(self.base_data_dir).join("definitions");
        let mut object_definitions = Vec::new();

        for entry in fs::read_dir(definitions_path)? {
            let entry = entry?;
            let path = entry.path();
            let file = fs::read_to_string(&path)?;

            let mut object_def = serde_json::from_str::<ObjectDefinition>(&file)?;

            if clean_for_post {
                object_def = Self::clean_for_post(object_def);
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

    fn clean_for_post(mut object_def: ObjectDefinition) -> ObjectDefinition {
        object_def.actions = None;
        object_def.id = None;
        object_def.active = None;

        if let Some(object_fields) = object_def.object_fields {
            object_def.object_fields = Some(
                object_fields
                    .into_iter()
                    .filter(|field| {
                        if let Some(business_type) = field.business_type {
                            business_type != BusinessType::Relationship
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        }

        if let Some(object_relationships) = object_def.object_relationships {
            object_def.object_relationships = Some(
                object_relationships
                    .into_iter()
                    .map(|mut relation| {
                        relation.object_definition_id1.take();
                        relation.object_definition_id2.take();
                        relation.id.take();
                        relation.parameter_object_field_id.take();
                        relation
                    })
                    .collect::<Vec<_>>(),
            )
        }
        object_def
    }
}
