use crate::{error::Error, git, project_repository, projects::ProjectId};

use super::{branch::Ownership, BranchId, GITBUTLER_INTEGRATION_REFERENCE};

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("head is detached")]
    DetachedHead,
    #[error("head is {0}")]
    InvalidHead(String),
    #[error("integration commit not found")]
    NoIntegrationCommit,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<VerifyError> for crate::error::Error {
    fn from(value: VerifyError) -> Self {
        match value {
            VerifyError::DetachedHead => crate::error::Error::UserError {
                code: crate::error::Code::ProjectHead,
                message: format!(
                    "Project in detached head state. Please checkout {0} to continue.",
                    GITBUTLER_INTEGRATION_REFERENCE.branch()
                ),
            },
            VerifyError::InvalidHead(head) => crate::error::Error::UserError {
                code: crate::error::Code::ProjectHead,
                message: format!(
                    "Project is on {}. Please checkout {} to continue.",
                    head,
                    GITBUTLER_INTEGRATION_REFERENCE.branch()
                ),
            },
            VerifyError::NoIntegrationCommit => crate::error::Error::UserError {
                code: crate::error::Code::ProjectHead,
                message: "GibButler's integration commit not found on head.".to_string(),
            },
            VerifyError::Other(error) => {
                tracing::error!(?error);
                crate::error::Error::Unknown
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteBranchError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ResetBranchError {
    #[error("commit {0} not in the branch")]
    CommitNotFoundInBranch(git::Oid),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ApplyBranchError {
    #[error("project")]
    Conflict(ProjectConflictError),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error("branch conflicts with other branches - sorry bro.")]
    BranchConflicts(BranchId),
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UnapplyOwnershipError {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("project is in conflict state")]
    Conflict(ProjectConflictError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UnapplyBranchError {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("project is in conflict state")]
    Conflict(ProjectConflictError),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum FlushAppliedVbranchesError {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ListVirtualBranchesError {
    #[error("project")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateVirtualBranchError {
    #[error("project")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum MergeVirtualBranchUpstreamError {
    #[error("project")]
    Conflict(ProjectConflictError),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("will not commit conflicted files")]
    Conflicted(ProjectConflictError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PushError {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error(transparent)]
    Remote(#[from] project_repository::RemoteError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum IsRemoteBranchMergableError {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("branch not found")]
    BranchNotFound(git::RemoteRefname),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum IsVirtualBranchMergeable {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AmendError {
    #[error("target ownership not found")]
    TargetOwnerhshipNotFound(Ownership),
    #[error("branch has no commits")]
    BranchHasNoCommits,
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error("project is in conflict state")]
    Conflict(ProjectConflictError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
#[derive(Debug, thiserror::Error)]
pub enum CherryPickError {
    #[error("target commit {0} not found ")]
    CommitNotFound(git::Oid),
    #[error("can not cherry pick not applied branch")]
    NotApplied,
    #[error("project is in conflict state")]
    Conflict(ProjectConflictError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SquashError {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("commit {0} not in the branch")]
    CommitNotFound(git::Oid),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error("project is in conflict state")]
    Conflict(ProjectConflictError),
    #[error("can not squash root commit")]
    CantSquashRootCommit,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
#[derive(Debug, thiserror::Error)]
pub enum GetBaseBranchDataError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SetBaseBranchError {
    #[error("branch {0} not found")]
    BranchNotFound(git::RemoteRefname),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
#[derive(Debug, thiserror::Error)]
pub enum UpdateBaseBranchError {
    #[error("no default target set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateVirtualBranchFromBranchError {
    #[error("merge conflict")]
    MergeConflict,
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("{0} not found")]
    BranchNotFound(git::Refname),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct ProjectConflictError {
    pub project_id: ProjectId,
}

impl From<ProjectConflictError> for Error {
    fn from(value: ProjectConflictError) -> Self {
        Error::UserError {
            code: crate::error::Code::ProjectConflict,
            message: format!("project {} is in a conflicted state", value.project_id),
        }
    }
}

#[derive(Debug)]
pub struct DefaultTargetNotSetError {
    pub project_id: ProjectId,
}

impl From<DefaultTargetNotSetError> for Error {
    fn from(value: DefaultTargetNotSetError) -> Self {
        Error::UserError {
            code: crate::error::Code::ProjectConflict,
            message: format!(
                "project {} does not have a default target set",
                value.project_id
            ),
        }
    }
}

#[derive(Debug)]
pub struct BranchNotFoundError {
    pub project_id: ProjectId,
    pub branch_id: BranchId,
}

impl From<BranchNotFoundError> for Error {
    fn from(value: BranchNotFoundError) -> Self {
        Error::UserError {
            code: crate::error::Code::Branches,
            message: format!("branch {} not found", value.branch_id),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateBranchError {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error("branch not found")]
    BranchNotFound(BranchNotFoundError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<UpdateBranchError> for Error {
    fn from(value: UpdateBranchError) -> Self {
        match value {
            UpdateBranchError::DefaultTargetNotSet(error) => error.into(),
            UpdateBranchError::BranchNotFound(error) => error.into(),
            UpdateBranchError::Other(error) => {
                tracing::error!(?error, "update branch error");
                Error::Unknown
            }
        }
    }
}

impl From<CreateVirtualBranchFromBranchError> for Error {
    fn from(value: CreateVirtualBranchFromBranchError) -> Self {
        match value {
            CreateVirtualBranchFromBranchError::DefaultTargetNotSet(error) => error.into(),
            CreateVirtualBranchFromBranchError::MergeConflict => Error::UserError {
                message: "Merge conflict".to_string(),
                code: crate::error::Code::Branches,
            },
            CreateVirtualBranchFromBranchError::BranchNotFound(name) => Error::UserError {
                message: format!("Branch {} not found", name),
                code: crate::error::Code::Branches,
            },
            CreateVirtualBranchFromBranchError::Other(error) => {
                tracing::error!(?error, "create virtual branch from branch error");
                Error::Unknown
            }
        }
    }
}

impl From<CommitError> for Error {
    fn from(value: CommitError) -> Self {
        match value {
            CommitError::BranchNotFound(error) => error.into(),
            CommitError::DefaultTargetNotSet(error) => error.into(),
            CommitError::Conflicted(error) => error.into(),
            CommitError::Other(error) => {
                tracing::error!(?error, "commit error");
                Error::Unknown
            }
        }
    }
}

impl From<IsRemoteBranchMergableError> for Error {
    fn from(value: IsRemoteBranchMergableError) -> Self {
        match value {
            IsRemoteBranchMergableError::BranchNotFound(name) => Error::UserError {
                message: format!("Remote branch {} not found", name),
                code: crate::error::Code::Branches,
            },
            IsRemoteBranchMergableError::DefaultTargetNotSet(error) => error.into(),
            IsRemoteBranchMergableError::Other(error) => {
                tracing::error!(?error, "is remote branch mergable error");
                Error::Unknown
            }
        }
    }
}

impl From<DeleteBranchError> for Error {
    fn from(value: DeleteBranchError) -> Self {
        match value {
            DeleteBranchError::Other(error) => {
                tracing::error!(?error, "delete branch error");
                Error::Unknown
            }
        }
    }
}

impl From<ApplyBranchError> for Error {
    fn from(value: ApplyBranchError) -> Self {
        match value {
            ApplyBranchError::DefaultTargetNotSet(error) => error.into(),
            ApplyBranchError::Conflict(error) => error.into(),
            ApplyBranchError::BranchNotFound(error) => error.into(),
            ApplyBranchError::BranchConflicts(id) => Error::UserError {
                message: format!("Branch {} is in a conflicing state", id),
                code: crate::error::Code::Branches,
            },
            ApplyBranchError::Other(error) => {
                tracing::error!(?error, "apply branch error");
                Error::Unknown
            }
        }
    }
}

impl From<IsVirtualBranchMergeable> for Error {
    fn from(value: IsVirtualBranchMergeable) -> Self {
        match value {
            IsVirtualBranchMergeable::BranchNotFound(error) => error.into(),
            IsVirtualBranchMergeable::DefaultTargetNotSet(error) => error.into(),
            IsVirtualBranchMergeable::Other(error) => {
                tracing::error!(?error, "is remote branch mergable error");
                Error::Unknown
            }
        }
    }
}

impl From<ListVirtualBranchesError> for Error {
    fn from(value: ListVirtualBranchesError) -> Self {
        match value {
            ListVirtualBranchesError::DefaultTargetNotSet(error) => error.into(),
            ListVirtualBranchesError::Other(error) => {
                tracing::error!(?error, "list virtual branches error");
                Error::Unknown
            }
        }
    }
}

impl From<CreateVirtualBranchError> for Error {
    fn from(value: CreateVirtualBranchError) -> Self {
        match value {
            CreateVirtualBranchError::DefaultTargetNotSet(error) => error.into(),
            CreateVirtualBranchError::Other(error) => {
                tracing::error!(?error, "create virtual branch error");
                Error::Unknown
            }
        }
    }
}

impl From<GetBaseBranchDataError> for Error {
    fn from(value: GetBaseBranchDataError) -> Self {
        match value {
            GetBaseBranchDataError::Other(error) => {
                tracing::error!(?error, "get base branch data error");
                Error::Unknown
            }
        }
    }
}

impl From<ListRemoteCommitFilesError> for Error {
    fn from(value: ListRemoteCommitFilesError) -> Self {
        match value {
            ListRemoteCommitFilesError::CommitNotFound(oid) => Error::UserError {
                message: format!("Commit {} not found", oid),
                code: crate::error::Code::Branches,
            },
            ListRemoteCommitFilesError::Other(error) => {
                tracing::error!(?error, "list remote commit files error");
                Error::Unknown
            }
        }
    }
}

impl From<SetBaseBranchError> for Error {
    fn from(value: SetBaseBranchError) -> Self {
        match value {
            SetBaseBranchError::BranchNotFound(name) => Error::UserError {
                message: format!("remote branch '{}' not found", name),
                code: crate::error::Code::Branches,
            },
            SetBaseBranchError::Other(error) => {
                tracing::error!(?error, "set base branch error");
                Error::Unknown
            }
        }
    }
}

impl From<MergeVirtualBranchUpstreamError> for Error {
    fn from(value: MergeVirtualBranchUpstreamError) -> Self {
        match value {
            MergeVirtualBranchUpstreamError::DefaultTargetNotSet(error) => error.into(),
            MergeVirtualBranchUpstreamError::BranchNotFound(error) => error.into(),
            MergeVirtualBranchUpstreamError::Conflict(error) => error.into(),
            MergeVirtualBranchUpstreamError::Other(error) => {
                tracing::error!(?error, "merge virtual branch upstream error");
                Error::Unknown
            }
        }
    }
}

impl From<UpdateBaseBranchError> for Error {
    fn from(value: UpdateBaseBranchError) -> Self {
        match value {
            UpdateBaseBranchError::DefaultTargetNotSet(error) => error.into(),
            UpdateBaseBranchError::Other(error) => {
                tracing::error!(?error, "update base branch error");
                Error::Unknown
            }
        }
    }
}

impl From<UnapplyOwnershipError> for Error {
    fn from(value: UnapplyOwnershipError) -> Self {
        match value {
            UnapplyOwnershipError::DefaultTargetNotSet(error) => error.into(),
            UnapplyOwnershipError::Conflict(error) => error.into(),
            UnapplyOwnershipError::Other(error) => {
                tracing::error!(?error, "unapply ownership error");
                Error::Unknown
            }
        }
    }
}

impl From<AmendError> for Error {
    fn from(value: AmendError) -> Self {
        match value {
            AmendError::Conflict(error) => error.into(),
            AmendError::BranchNotFound(error) => error.into(),
            AmendError::BranchHasNoCommits => Error::UserError {
                message: "Branch has no commits - there is nothing to amend to".to_string(),
                code: crate::error::Code::Branches,
            },
            AmendError::DefaultTargetNotSet(error) => error.into(),
            AmendError::TargetOwnerhshipNotFound(_) => Error::UserError {
                message: "target ownership not found".to_string(),
                code: crate::error::Code::Branches,
            },
            AmendError::Other(error) => {
                tracing::error!(?error, "amend error");
                Error::Unknown
            }
        }
    }
}

impl From<ResetBranchError> for Error {
    fn from(value: ResetBranchError) -> Self {
        match value {
            ResetBranchError::BranchNotFound(error) => error.into(),
            ResetBranchError::DefaultTargetNotSet(error) => error.into(),
            ResetBranchError::CommitNotFoundInBranch(oid) => Error::UserError {
                code: crate::error::Code::Branches,
                message: format!("commit {} not found", oid),
            },
            ResetBranchError::Other(error) => {
                tracing::error!(?error, "reset branch error");
                Error::Unknown
            }
        }
    }
}

impl From<UnapplyBranchError> for Error {
    fn from(value: UnapplyBranchError) -> Self {
        match value {
            UnapplyBranchError::Conflict(error) => error.into(),
            UnapplyBranchError::DefaultTargetNotSet(error) => error.into(),
            UnapplyBranchError::BranchNotFound(error) => error.into(),
            UnapplyBranchError::Other(error) => {
                tracing::error!(?error, "unapply branch error");
                Error::Unknown
            }
        }
    }
}

impl From<PushError> for Error {
    fn from(value: PushError) -> Self {
        match value {
            PushError::Remote(error) => error.into(),
            PushError::BranchNotFound(error) => error.into(),
            PushError::DefaultTargetNotSet(error) => error.into(),
            PushError::Other(error) => {
                tracing::error!(?error, "push error");
                Error::Unknown
            }
        }
    }
}

impl From<FlushAppliedVbranchesError> for Error {
    fn from(value: FlushAppliedVbranchesError) -> Self {
        match value {
            FlushAppliedVbranchesError::DefaultTargetNotSet(error) => error.into(),
            FlushAppliedVbranchesError::Other(error) => {
                tracing::error!(?error, "flush applied branches error");
                Error::Unknown
            }
        }
    }
}

impl From<CherryPickError> for Error {
    fn from(value: CherryPickError) -> Self {
        match value {
            CherryPickError::NotApplied => Error::UserError {
                message: "can not cherry pick non applied branch".to_string(),
                code: crate::error::Code::Branches,
            },
            CherryPickError::Conflict(error) => error.into(),
            CherryPickError::CommitNotFound(oid) => Error::UserError {
                message: format!("commit {oid} not found"),
                code: crate::error::Code::Branches,
            },
            CherryPickError::Other(error) => {
                tracing::error!(?error, "cherry pick error");
                Error::Unknown
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ListRemoteCommitFilesError {
    #[error("failed to find commit {0}")]
    CommitNotFound(git::Oid),
    #[error("failed to find commit")]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ListRemoteBranchesError {
    #[error("default target not set")]
    DefaultTargetNotSet(DefaultTargetNotSetError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<ListRemoteBranchesError> for Error {
    fn from(value: ListRemoteBranchesError) -> Self {
        match value {
            ListRemoteBranchesError::DefaultTargetNotSet(error) => error.into(),
            ListRemoteBranchesError::Other(error) => {
                tracing::error!(?error, "list remote branches error");
                Error::Unknown
            }
        }
    }
}

impl From<SquashError> for Error {
    fn from(value: SquashError) -> Self {
        match value {
            SquashError::DefaultTargetNotSet(error) => error.into(),
            SquashError::BranchNotFound(error) => error.into(),
            SquashError::Conflict(error) => error.into(),
            SquashError::CantSquashRootCommit => Error::UserError {
                message: "can not squash root branch commit".to_string(),
                code: crate::error::Code::Branches,
            },
            SquashError::CommitNotFound(oid) => Error::UserError {
                message: format!("commit {oid} not found"),
                code: crate::error::Code::Branches,
            },
            SquashError::Other(error) => {
                tracing::error!(?error, "squash error");
                Error::Unknown
            }
        }
    }
}