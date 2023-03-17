use headless_common::url::Url;

use super::list_type_endpoints::ListTypeEndpoints;

pub struct ListTypeApi<'a> {
    base_url: &'a Url,
    username: &'a str,
    password: &'a str,
}

impl<'a> ListTypeApi<'a> {
    pub fn new(base_url: &'a Url, username: &'a str, password: &'a str) -> Self {
        Self {
            base_url,
            username,
            password,
        }
    }

    pub fn get_list_type_api_endpoints(&self) -> ListTypeEndpoints {
        ListTypeEndpoints::new(self.base_url, self.username, self.password)
    }
}
