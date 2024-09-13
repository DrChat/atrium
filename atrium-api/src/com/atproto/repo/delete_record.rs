// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.repo.deleteRecord` namespace.
pub const NSID: &str = "com.atproto.repo.deleteRecord";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///The NSID of the record collection.
    pub collection: crate::types::string::Nsid,
    ///The handle or DID of the repo (aka, current account).
    pub repo: crate::types::string::AtIdentifier,
    ///The Record Key.
    pub rkey: String,
    ///Compare and swap with the previous commit by CID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<crate::types::string::Cid>,
    ///Compare and swap with the previous record by CID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_record: Option<crate::types::string::Cid>,
}
pub type Input = crate::types::Object<InputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<crate::com::atproto::repo::defs::CommitMeta>,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    InvalidSwap(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidSwap(msg) => {
                write!(_f, "InvalidSwap")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
