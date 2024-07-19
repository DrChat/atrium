// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.label.defs` namespace.
///Metadata tag on an atproto resource (eg, repo or record).
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelData {
    ///Optionally, CID specifying the specific version of 'uri' resource this label applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<crate::types::string::Cid>,
    ///Timestamp when this label was created.
    pub cts: crate::types::string::Datetime,
    ///Timestamp at which this label expires (no longer applies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<crate::types::string::Datetime>,
    ///If true, this is a negation label, overwriting a previous label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neg: Option<bool>,
    ///Signature of dag-cbor encoded label.
    #[serde(default)]
    #[serde(with = "serde_bytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sig: Option<Vec<u8>>,
    ///DID of the actor who created this label.
    pub src: crate::types::string::Did,
    ///AT URI of the record, repository (account), or other resource that this label applies to.
    pub uri: String,
    ///The short string name of the value or type of this label.
    pub val: String,
    ///The AT Protocol version of the label object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<i64>,
}
pub type Label = crate::types::Object<LabelData>;
pub type LabelValue = String;
///Declares a label value and its expected interpertations and behaviors.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelValueDefinitionData {
    ///Does the user need to have adult content enabled in order to configure this label?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adult_only: Option<bool>,
    ///What should this label hide in the UI, if applied? 'content' hides all of the target; 'media' hides the images/video/audio; 'none' hides nothing.
    pub blurs: String,
    ///The default setting for this label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_setting: Option<String>,
    ///The value of the label being defined. Must only include lowercase ascii and the '-' character ([a-z-]+).
    pub identifier: String,
    pub locales: Vec<LabelValueDefinitionStrings>,
    ///How should a client visually convey this label? 'inform' means neutral and informational; 'alert' means negative and warning; 'none' means show nothing.
    pub severity: String,
}
pub type LabelValueDefinition = crate::types::Object<LabelValueDefinitionData>;
///Strings which describe the label in the UI, localized into a specific language.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelValueDefinitionStringsData {
    ///A longer description of what the label means and why it might be applied.
    pub description: String,
    ///The code of the language these strings are written in.
    pub lang: crate::types::string::Language,
    ///A short human-readable name for the label.
    pub name: String,
}
pub type LabelValueDefinitionStrings = crate::types::Object<
    LabelValueDefinitionStringsData,
>;
///Metadata tag on an atproto record, published by the author within the record. Note that schemas should use #selfLabels, not #selfLabel.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SelfLabelData {
    ///The short string name of the value or type of this label.
    pub val: String,
}
pub type SelfLabel = crate::types::Object<SelfLabelData>;
///Metadata tags on an atproto record, published by the author within the record.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SelfLabelsData {
    pub values: Vec<SelfLabel>,
}
pub type SelfLabels = crate::types::Object<SelfLabelsData>;
