pub mod client;
pub mod client_options;
pub mod custom_objects;
pub mod liferay_client_error;
pub mod list_type_api;
pub mod object_admin_api;
trait Api {}

pub fn clean_json(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Array(arr) => arr.iter_mut().for_each(clean_json),
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

            obj.iter_mut().for_each(|(_, val)| clean_json(val))
        }
        _ => (),
    }
}
