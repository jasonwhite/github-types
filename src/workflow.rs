use serde::{Deserialize, Serialize};

use crate::{DateTime, InstallationPermissions, User};

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowCompletion {
    Success,
    Failure,
    Skipped,
    Cancelled,
    ActionRequired,
    Neutral,
    TimedOut,
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    Queued,
    InProgress,
    Waiting,
    Completed,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStepConclusion {
    Success,
    Failure,
    Skipped,
    Cancelled,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStepStatus {
    Queued,
    InProgress,
    Completed,
    Failure,
    Pending,
}
#[derive(
    Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
pub struct WorkflowStep {
    pub completed_at: Option<DateTime>,
    pub name: String,
    pub number: u64,
    pub started_at: Option<DateTime>,
    pub conclusion: Option<WorkflowStepConclusion>,
    pub status: Option<WorkflowStepStatus>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorkflowJob {
    pub check_run_url: String,
    pub completed_at: Option<DateTime>,
    pub conclusion: Option<WorkflowCompletion>,
    pub created_at: DateTime,
    pub head_sha: String,
    pub html_url: String,
    pub id: u64,
    pub labels: Vec<String>,
    pub name: String,
    pub node_id: String,
    pub run_attempt: u64,
    pub run_id: u64,
    pub run_url: String,
    /// The ID of the runner group that is running this job. This will be null as long as workflow_job[status] is queued.
    pub runner_group_id: Option<u64>,
    /// The name of the runner group that is running this job. This will be null as long as workflow_job[status] is queued.
    pub runner_group_name: Option<String>,
    /// The ID of the runner that is running this job. This will be null as long as workflow_job[status] is queued.
    pub runner_id: Option<u64>,
    /// The name of the runner that is running this job. This will be null as long as workflow_job[status] is queued.
    pub runner_name: Option<String>,
    pub started_at: DateTime,
    /// The current status of the job. Can be `queued`, `in_progress`, `waiting`, or `completed`.
    pub status: WorkflowStatus,
    pub head_branch: Option<String>,
    pub workflow_name: Option<String>,
    pub steps: Vec<WorkflowStep>,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeploymentPerformedViaGithubApp {
    pub id: u64,
    pub slug: String,
    pub node_id: String,
    pub owner: User,
    pub name: String,
    pub description: Option<String>,
    pub external_url: String,
    pub html_url: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub permissions: InstallationPermissions,
    pub events: Vec<String>,
    pub installation_counts: u64,
    pub client_id: String,
    pub client_secret: String,
    pub webhook_secret: Option<String>,
    pub pem: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Deployment {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub sha: String,
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub task: String,
    pub payload: String,
    pub original_environment: String,
    pub environment: String,
    pub description: Option<String>,
    pub creator: User,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub statuses_url: String,
    pub repository_url: String,
    pub transient_environment: String,
    pub performed_via_github_app: DeploymentPerformedViaGithubApp,
}
