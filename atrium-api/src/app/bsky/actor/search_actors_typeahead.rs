// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.actor.searchActorsTypeahead` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU8<100u8>>,
    ///Search query prefix; not a full query string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    ///DEPRECATED: use 'q' instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub actors: Vec<crate::app::bsky::actor::defs::ProfileViewBasic>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
