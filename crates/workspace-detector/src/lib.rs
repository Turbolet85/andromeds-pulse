pub mod contract;
pub mod detect;
mod marker;
mod vcs;

pub use contract::{
    Error, MAX_WORKSPACE_KEY_BYTES, VcsMetadata, VcsType, WORKSPACE_KEY_BASENAME, WorkspaceContext,
    publish_workspace_key, read_published_workspace_key, workspace_key, workspace_key_path,
};
pub use detect::detect;
