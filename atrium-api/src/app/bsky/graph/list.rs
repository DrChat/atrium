// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.graph.list` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<crate::types::BlobRef>,
    pub created_at: crate::types::string::Datetime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<crate::app::bsky::richtext::facet::Main>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::Union<RecordLabelsRefs>>,
    ///Display name for list; can not be empty.
    pub name: String,
    ///Defines the purpose of the list (aka, moderation-oriented or curration-oriented)
    pub purpose: crate::app::bsky::graph::defs::ListPurpose,
}
pub type Record = crate::types::Object<RecordData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum RecordLabelsRefs {
    #[serde(rename = "com.atproto.label.defs#selfLabels")]
    ComAtprotoLabelDefsSelfLabels(Box<crate::com::atproto::label::defs::SelfLabels>),
}
