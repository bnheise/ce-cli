use headless_common::url::Url;

use super::object_admin_endpoints::ObjectAdminEndpoints;

pub struct ObjectAdminApi<'a> {
    base_url: &'a Url,
    username: &'a str,
    password: &'a str,
}

impl<'a> ObjectAdminApi<'a> {
    pub fn new(base_url: &'a Url, username: &'a str, password: &'a str) -> Self {
        Self {
            base_url,
            username,
            password,
        }
    }

    pub fn get_object_admin_endpoints(&self) -> ObjectAdminEndpoints {
        ObjectAdminEndpoints::new(self.base_url, self.username, self.password)
    }
}
