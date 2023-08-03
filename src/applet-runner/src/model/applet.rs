use std::ops::Deref;

use application_models::applets::Applet;
use http::Method;

use crate::{AppletCookies, AppletForm, AppletQueryString};

#[derive(Debug)]
pub struct AppletRunner {
    method: Method,
    query_string: Option<AppletQueryString>,
    cookies: Option<AppletCookies>,
    form: Option<AppletForm>,
    artifact: AppletRunnerArtifact,
}

impl AppletRunner {
    pub fn new(
        method: Method,
        query_string: Option<AppletQueryString>,
        cookies: Option<AppletCookies>,
        form: Option<AppletForm>,
        artifact: AppletRunnerArtifact,
    ) -> Self {
        Self { method, query_string, cookies, form, artifact }
    }
    // pub fn get_code(&self) -> &String {
    //     &self.artifact
    // }
    // pub fn get_artifact(&self) -> &AppletRunnerArtifact {
    //     &self.artifact
    // }
    pub fn take(
        self,
    ) -> (
        Method,
        Option<AppletQueryString>,
        Option<AppletCookies>,
        Option<AppletForm>,
        AppletRunnerArtifact,
    ) {
        let Self { method, query_string, cookies, form, artifact } = self;
        (method, query_string, cookies, form, artifact)
    }
}

#[derive(Debug)]
pub struct AppletRunnerArtifact(Applet);

impl From<Applet> for AppletRunnerArtifact {
    fn from(artifact: Applet) -> Self {
        Self(artifact)
    }
}

impl Deref for AppletRunnerArtifact {
    type Target = Applet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AppletRunnerArtifact {
    pub fn take(self) -> Applet {
        self.0
    }
}
