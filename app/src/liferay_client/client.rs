use super::{
    client_options::LiferayClientOptions,
    custom_objects::custom_object_endpoints::CustomObjectEndpoints,
    list_type_api::list_type::ListTypeApi, object_admin_api::object_admin::ObjectAdminApi,
};

pub struct LiferayClient {
    base_url: batch_api::reqwest::Url,
    username: String,
    password: String,
}

impl LiferayClient {
    pub fn new(options: LiferayClientOptions) -> Self {
        let LiferayClientOptions {
            base_url,
            username,
            password,
        } = options;

        Self {
            base_url,
            username,
            password,
        }
    }

    pub fn get_object_admin_api(&self) -> ObjectAdminApi {
        ObjectAdminApi::new(&self.base_url, &self.username, &self.password)
    }

    pub fn get_custom_object_api(&self) -> CustomObjectEndpoints {
        CustomObjectEndpoints::new(&self.base_url, &self.username, &self.password)
    }

    pub fn get_list_type_api(&self) -> ListTypeApi {
        ListTypeApi::new(&self.base_url, &self.username, &self.password)
    }
}
