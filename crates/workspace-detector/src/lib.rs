pub mod contract;
pub mod detect;
mod marker;
mod vcs;

pub use contract::{Error, VcsMetadata, VcsType, WorkspaceContext};
pub use detect::detect;
