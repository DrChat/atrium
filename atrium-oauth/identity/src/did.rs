mod common_resolver;
mod plc_resolver;
mod web_resolver;

pub use self::common_resolver::{CommonDidResolver, CommonDidResolverConfig};
use crate::Resolver;
use atrium_api::did_doc::DidDocument;
use atrium_api::types::string::Did;

pub trait DidResolver: Resolver<Input = Did, Output = DidDocument> {}
