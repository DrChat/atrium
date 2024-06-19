// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.server.createAppPassword` namespace.
pub const NSID: &str = "com.atproto.server.createAppPassword";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///A short name for the App Password, to help distinguish them.
    pub name: String,
    ///If an app password has 'privileged' access to possibly sensitive account state. Meant for use with trusted clients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
}
pub type Input = crate::types::Object<InputData>;
pub type Output = AppPassword;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    AccountTakedown(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AccountTakedown(msg) => {
                write!(_f, "AccountTakedown")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppPasswordData {
    pub created_at: crate::types::string::Datetime,
    pub name: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
}
pub type AppPassword = crate::types::Object<AppPasswordData>;
