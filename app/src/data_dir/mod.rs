use std::{fs, path::Path};

use object_admin::models::{object_field::BusinessType, ObjectDefinition, ObjectRelationship};

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

            let mut as_value = serde_json::from_str::<serde_json::Value>(&file)?;
            if clean_for_post {
                Self::clean_json(&mut as_value);
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

            println!("{}", serde_json::to_string(&object_def).unwrap_or_default());

            let erc = object_def
                .external_reference_code
                .clone()
                .unwrap_or_default();
            println!("Found definition: {erc}");
            object_definitions.push(object_def);
        }
        Ok(object_definitions)
    }

    fn clean_json(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Array(arr) => arr.iter_mut().for_each(Self::clean_json),
            serde_json::Value::Object(obj) => {
                let keys_to_remove = obj
                    .iter()
                    .filter(|(key, val)| {
                        val.is_number() && (key.ends_with("id") || key.ends_with("Id"))
                    })
                    .map(|(key, _)| key.to_owned())
                    .collect::<Vec<_>>();

                for key in keys_to_remove.iter() {
                    obj.remove(key);
                }

                obj.iter_mut().for_each(|(_, val)| Self::clean_json(val))
            }
            _ => (),
        }
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

    // fn clean_layouts(layouts: &mut Option<Vec<ObjectLayout>>) {
    //     if let Some(layouts) = layouts.as_mut() {
    //         layouts.iter_mut().for_each(|mut layout| {
    //             layout.id = None;
    //             layout.object_definition_id = None;
    //             layout.actions = None;
    //             Self::clean_layout_tabs(&mut layout.object_layout_tabs);
    //         })
    //     }
    // }

    // fn clean_layout_tabs(tabs: &mut Option<Vec<ObjectLayoutTab>>) {
    //     if let Some(tabs) = tabs.as_mut() {
    //         tabs.iter_mut().for_each(|mut tab| {
    //             tab.id = None;
    //             tab.object_relationship_id = None;
    //             Self::clean_layout_boxes(&mut tab.object_layout_boxes);
    //         })
    //     }
    // }

    // fn clean_layout_boxes(boxes: &mut Option<Vec<ObjectLayoutBox>>) {
    //     if let Some(boxes) = boxes.as_mut() {
    //         boxes.iter_mut().for_each(|mut box_| {
    //             box_.id = None;
    //             // Self::clean_layout_boxes(&mut tab.object_layout_boxes);
    //         })
    //     }
    // }

    // fn clean_actions(actions: &mut Option<Vec<ObjectAction>>) {
    //     if let Some(actions) = actions.as_mut() {
    //         actions.iter_mut().for_each(|mut action| {
    //             action.actions = None;
    //             action.active = None;
    //             action.id = None;
    //         });
    //     }
    // }
}
