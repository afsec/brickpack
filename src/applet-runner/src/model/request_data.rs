use http::Method;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AppletRequestData {
    oid: String,
    method: Method,
    query_string: Option<AppletQueryString>,
    cookies: Option<AppletCookies>,
    form: Option<AppletForm>,
}

impl AppletRequestData {
    pub fn new(oid: String, method: Method) -> Self {
        Self {
            oid,
            method,
            query_string: Default::default(),
            cookies: Default::default(),
            form: Default::default(),
        }
    }

    pub fn get_oid(&self) -> &String {
        &self.oid
    }
    pub fn get_method(&self) -> &Method {
        &self.method
    }
    pub fn get_query_string(&self) -> &Option<AppletQueryString> {
        &self.query_string
    }
    pub fn get_cookies(&self) -> &Option<AppletCookies> {
        &self.cookies
    }
    pub fn get_form(&self) -> &Option<AppletForm> {
        &self.form
    }
    pub fn query_string(&mut self, value: AppletQueryString) {
        self.query_string = Some(value);
    }
    pub fn cookies(&mut self, value: AppletCookies) {
        self.cookies = Some(value);
    }
    pub fn form(&mut self, value: AppletForm) {
        self.form = Some(value);
    }
    pub fn take(
        self,
    ) -> (String, Method, Option<AppletQueryString>, Option<AppletCookies>, Option<AppletForm>)
    {
        let Self { oid, method, query_string, cookies, form } = self;
        (oid, method, query_string, cookies, form)
    }
}

#[derive(Debug, Deserialize)]
pub struct AppletQueryString(HashMap<String, String>);
impl AppletQueryString {
    // pub fn get(&self) -> &HashMap<String, String> {
    //     &self.0
    // }
    pub fn take(self) -> HashMap<String, String> {
        self.0
    }
}
impl From<HashMap<String, String>> for AppletQueryString {
    fn from(hashmap: HashMap<String, String>) -> Self {
        Self(hashmap)
    }
}

#[derive(Debug)]
pub struct AppletCookies(HashMap<String, String>);
impl AppletCookies {
    // pub fn get(&self) -> &HashMap<String, String> {
    //     &self.0
    // }
    pub fn take(self) -> HashMap<String, String> {
        self.0
    }
}
impl From<HashMap<String, String>> for AppletCookies {
    fn from(hashmap: HashMap<String, String>) -> Self {
        Self(hashmap)
    }
}

#[derive(Deserialize, Debug)]
pub struct AppletForm(HashMap<String, String>);
impl AppletForm {
    // pub fn get(&self) -> &HashMap<String, String> {
    //     &self.0
    // }
    pub fn take(self) -> HashMap<String, String> {
        self.0
    }
}
impl From<HashMap<String, String>> for AppletForm {
    fn from(hashmap: HashMap<String, String>) -> Self {
        Self(hashmap)
    }
}

// pub struct AppletJson(HashMap<String, String>);
// impl AppletJson {
//     pub fn get(&self) -> &HashMap<String, String> {
//         &self.0
//     }
// }
// impl From<HashMap<String, String>> for AppletJson {
//     fn from(hashmap: HashMap<String, String>) -> Self {
//         Self(hashmap)
//     }
// }
