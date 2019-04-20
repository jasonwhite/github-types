// Copyright (c) 2019 Jason White
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Events are used by repository webhooks.
//!
//! See: https://developer.github.com/v3/activity/events/types/

use serde::{
    de::{self, Deserializer},
    Deserialize,
};

use std::fmt;
use std::str::FromStr;

use crate::{
    AppEvent, Comment, DateTime, Installation, Issue, Label, Oid, PullRequest,
    Repository, Review, ShortRepo, User,
};

/// GitHub events that are specified in the X-Github-Event header.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EventType {
    /// (Special event.) Any time any event is triggered (Wildcard Event).
    Wildcard,

    /// (Special event.) Sent when a webhook is added.
    Ping,

    /// Triggered when a check run is `created`, `rerequested`, `completed`, or
    /// has a `requested_action`.
    CheckRun,

    /// Triggered when a check suite is `completed`, `requested`, or
    /// `rerequested`.
    CheckSuite,

    /// Any time a Commit is commented on.
    CommitComment,

    /// Triggered when the body or comment of an issue or pull request includes
    /// a URL that matches a configured content reference domain. Only GitHub
    /// Apps can receive this event.
    ContentReference,

    /// Any time a Branch or Tag is created.
    Create,

    /// Any time a Branch or Tag is deleted.
    Delete,

    /// Any time a Repository has a new deployment created from the API.
    Deployment,

    /// Any time a deployment for a Repository has a status update from the
    /// API.
    DeploymentStatus,

    /// Any time a Repository is forked.
    Fork,

    /// Triggered when someone revokes their authorization of a GitHub App.
    GitHubAppAuthorization,

    /// Any time a Wiki page is updated.
    Gollum,

    /// Any time a GitHub App is installed or uninstalled.
    Installation,

    /// Same as `Installation`, but deprecated. This event is sent alongside
    /// the `Installation` event, but can always be ignored.
    IntegrationInstallation,

    /// Any time a repository is added or removed from an installation.
    InstallationRepositories,

    /// Same as `InstallationRepositories`, but deprecated. This event is sent
    /// alongside the `InstallationRepositories` event, but can always be
    /// ignored.
    IntegrationInstallationRepositories,

    /// Any time a comment on an issue is created, edited, or deleted.
    IssueComment,

    /// Any time an Issue is assigned, unassigned, labeled, unlabeled,
    /// opened, edited, milestoned, demilestoned, closed, or reopened.
    Issues,

    /// Any time a Label is created, edited, or deleted.
    Label,

    /// Any time a user purchases, cancels, or changes their GitHub
    /// Marketplace plan.
    MarketplacePurchase,

    /// Any time a User is added or removed as a collaborator to a
    /// Repository, or has their permissions modified.
    Member,

    /// Any time a User is added or removed from a team. Organization hooks
    /// only.
    Membership,

    /// Any time a Milestone is created, closed, opened, edited, or deleted.
    Milestone,

    /// Any time a user is added, removed, or invited to an Organization.
    /// Organization hooks only.
    Organization,

    /// Any time an organization blocks or unblocks a user. Organization
    /// hooks only.
    OrgBlock,

    /// Any time a Pages site is built or results in a failed build.
    PageBuild,

    /// Any time a Project Card is created, edited, moved, converted to an
    /// issue, or deleted.
    ProjectCard,

    /// Any time a Project Column is created, edited, moved, or deleted.
    ProjectColumn,

    /// Any time a Project is created, edited, closed, reopened, or deleted.
    Project,

    /// Any time a Repository changes from private to public.
    Public,

    /// Any time a pull request is assigned, unassigned, labeled, unlabeled,
    /// opened, edited, closed, reopened, or synchronized (updated due to a
    /// new push in the branch that the pull request is tracking). Also any
    /// time a pull request review is requested, or a review request is
    /// removed.
    PullRequest,

    /// Any time a comment on a pull request's unified diff is created,
    /// edited, or deleted (in the Files Changed tab).
    PullRequestReviewComment,

    /// Any time a pull request review is submitted, edited, or dismissed.
    PullRequestReview,

    /// Any Git push to a Repository, including editing tags or branches.
    /// Commits via API actions that update references are also counted.
    /// This is the default event.
    Push,

    /// Any time a Release is published in a Repository.
    Release,

    /// Any time a Repository is created, deleted (organization hooks
    /// only), archived, unarchived, made public, or made private.
    Repository,

    /// Triggered when a successful, cancelled, or failed repository import
    /// finishes for a GitHub organization or a personal repository. To receive
    /// this event for a personal repository, you must create an empty
    /// repository prior to the import. This event can be triggered using
    /// either the GitHub Importer or the Source imports API.
    RepositoryImport,

    /// Triggered when a security alert is created, dismissed, or resolved.
    RepositoryVulnerabilityAlert,

    /// Triggered when a new security advisory is published, updated, or
    /// withdrawn. A security advisory provides information about
    /// security-related vulnerabilities in software on GitHub. Security
    /// Advisory webhooks are available to GitHub Apps only.
    SecurityAdvisory,

    /// Any time a Repository has a status update from the API.
    Status,

    /// Any time a team is created, deleted, modified, or added to or
    /// removed from a repository. Organization hooks only
    Team,

    /// Any time a team is added or modified on a Repository.
    TeamAdd,

    /// Any time a User stars a Repository.
    Watch,
}

impl EventType {
    /// Returns a static string for the event name.
    pub fn name(self) -> &'static str {
        match self {
            EventType::Wildcard => "*",
            EventType::Ping => "ping",
            EventType::CheckRun => "check_run",
            EventType::CheckSuite => "check_suite",
            EventType::CommitComment => "commit_comment",
            EventType::ContentReference => "content_reference",
            EventType::Create => "create",
            EventType::Delete => "delete",
            EventType::Deployment => "deployment",
            EventType::DeploymentStatus => "deployment_status",
            EventType::Fork => "fork",
            EventType::GitHubAppAuthorization => "github_app_authorization",
            EventType::Gollum => "gollum",
            EventType::Installation => "installation",
            EventType::IntegrationInstallation => "integration_installation",
            EventType::InstallationRepositories => "installation_repositories",
            EventType::IntegrationInstallationRepositories => {
                "integration_installation_repositories"
            }
            EventType::IssueComment => "issue_comment",
            EventType::Issues => "issues",
            EventType::Label => "label",
            EventType::MarketplacePurchase => "marketplace_purchase",
            EventType::Member => "member",
            EventType::Membership => "membership",
            EventType::Milestone => "milestone",
            EventType::Organization => "organization",
            EventType::OrgBlock => "org_block",
            EventType::PageBuild => "page_build",
            EventType::ProjectCard => "project_card",
            EventType::ProjectColumn => "project_column",
            EventType::Project => "project",
            EventType::Public => "public",
            EventType::PullRequest => "pull_request",
            EventType::PullRequestReview => "pull_request_review",
            EventType::PullRequestReviewComment => {
                "pull_request_review_comment"
            }
            EventType::Push => "push",
            EventType::Release => "release",
            EventType::Repository => "repository",
            EventType::RepositoryImport => "repository_import",
            EventType::RepositoryVulnerabilityAlert => {
                "repository_vulnerability_alert"
            }
            EventType::SecurityAdvisory => "security_advisory",
            EventType::Status => "status",
            EventType::Team => "team",
            EventType::TeamAdd => "team_add",
            EventType::Watch => "watch",
        }
    }
}

impl FromStr for EventType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(EventType::Wildcard),
            "ping" => Ok(EventType::Ping),
            "check_run" => Ok(EventType::CheckRun),
            "check_suite" => Ok(EventType::CheckSuite),
            "commit_comment" => Ok(EventType::CommitComment),
            "content_reference" => Ok(EventType::ContentReference),
            "create" => Ok(EventType::Create),
            "delete" => Ok(EventType::Delete),
            "deployment" => Ok(EventType::Deployment),
            "deployment_status" => Ok(EventType::DeploymentStatus),
            "fork" => Ok(EventType::Fork),
            "github_app_authorization" => Ok(EventType::GitHubAppAuthorization),
            "gollum" => Ok(EventType::Gollum),
            "installation" => Ok(EventType::Installation),
            "integration_installation" => {
                Ok(EventType::IntegrationInstallation)
            }
            "installation_repositories" => {
                Ok(EventType::InstallationRepositories)
            }
            "integration_installation_repositories" => {
                Ok(EventType::IntegrationInstallationRepositories)
            }
            "issue_comment" => Ok(EventType::IssueComment),
            "issues" => Ok(EventType::Issues),
            "label" => Ok(EventType::Label),
            "marketplace_purchase" => Ok(EventType::MarketplacePurchase),
            "member" => Ok(EventType::Member),
            "membership" => Ok(EventType::Membership),
            "milestone" => Ok(EventType::Milestone),
            "organization" => Ok(EventType::Organization),
            "org_block" => Ok(EventType::OrgBlock),
            "page_build" => Ok(EventType::PageBuild),
            "project_card" => Ok(EventType::ProjectCard),
            "project_column" => Ok(EventType::ProjectColumn),
            "project" => Ok(EventType::Project),
            "public" => Ok(EventType::Public),
            "pull_request" => Ok(EventType::PullRequest),
            "pull_request_review_comment" => {
                Ok(EventType::PullRequestReviewComment)
            }
            "pull_request_review" => Ok(EventType::PullRequestReview),
            "push" => Ok(EventType::Push),
            "release" => Ok(EventType::Release),
            "repository" => Ok(EventType::Repository),
            "repository_import" => Ok(EventType::RepositoryImport),
            "repository_vulnerability_alert" => {
                Ok(EventType::RepositoryVulnerabilityAlert)
            }
            "security_advisory" => Ok(EventType::SecurityAdvisory),
            "status" => Ok(EventType::Status),
            "team" => Ok(EventType::Team),
            "team_add" => Ok(EventType::TeamAdd),
            "watch" => Ok(EventType::Watch),
            _ => Err("invalid GitHub event"),
        }
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// An event with a corresponding payload.
///
/// For documentation on each of these events, see:
/// https://developer.github.com/v3/activity/events/types/
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    Ping(PingEvent),
    // CheckRun(CheckRunEvent),
    // CheckSuite(CheckSuiteEvent),
    CommitComment(CommitCommentEvent),
    // ContentReference(ContentReferenceEvent),
    Create(CreateEvent),
    Delete(DeleteEvent),
    // Deployment(DeploymentEvent),
    // DeploymentStatus(DeploymentStatusEvent),
    // Fork(ForkEvent),
    GitHubAppAuthorization(GitHubAppAuthorizationEvent),
    Gollum(GollumEvent),
    Installation(InstallationEvent),
    InstallationRepositories(InstallationRepositoriesEvent),
    IntegrationInstallation(InstallationEvent),
    IntegrationInstallationRepositories(InstallationRepositoriesEvent),
    IssueComment(IssueCommentEvent),
    Issues(IssuesEvent),
    Label(LabelEvent),
    // MarketplacePurchase(MarketplacePurchaseEvent),
    // Member(MemberEvent),
    // Membership(MembershipEvent),
    // Milestone(MilestoneEvent),
    // Organization(OrganizationEvent),
    // OrgBlock(OrgBlockEvent),
    // PageBuild(PageBuildEvent),
    // ProjectCard(ProjectCardEvent),
    // ProjectColumn(ProjectColumnEvent),
    // Project(ProjectEvent),
    // Public(PublicEvent),
    PullRequest(PullRequestEvent),
    PullRequestReview(PullRequestReviewEvent),
    PullRequestReviewComment(PullRequestReviewCommentEvent),
    Push(PushEvent),
    // Release(ReleaseEvent),
    Repository(RepositoryEvent),
    // RepositoryImport(RepositoryImportEvent),
    // RepositoryVulnerabilityAlert(RepositoryVulnerabilityAlertEvent),
    // SecurityAdvisory(SecurityAdvisoryEvent),
    // Status(StatusEvent),
    // Team(TeamEvent),
    // TeamAdd(TeamAddEvent),
    Watch(WatchEvent),
}

impl AppEvent for Event {
    fn installation(&self) -> Option<u64> {
        match self {
            Event::Ping(e) => e.installation(),
            Event::CommitComment(e) => e.installation(),
            Event::Create(e) => e.installation(),
            Event::Delete(e) => e.installation(),
            Event::GitHubAppAuthorization(e) => e.installation(),
            Event::Gollum(e) => e.installation(),
            Event::Installation(e) => e.installation(),
            Event::InstallationRepositories(e) => e.installation(),
            Event::IntegrationInstallation(e) => e.installation(),
            Event::IntegrationInstallationRepositories(e) => e.installation(),
            Event::IssueComment(e) => e.installation(),
            Event::Issues(e) => e.installation(),
            Event::Label(e) => e.installation(),
            Event::PullRequest(e) => e.installation(),
            Event::PullRequestReview(e) => e.installation(),
            Event::PullRequestReviewComment(e) => e.installation(),
            Event::Push(e) => e.installation(),
            Event::Repository(e) => e.installation(),
            Event::Watch(e) => e.installation(),
        }
    }
}

/// The App installation ID.
#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
pub struct InstallationId {
    pub id: u64,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type")]
pub enum Hook {
    Repository(RepoHook),
    App(AppHook),
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RepoHook {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub events: Vec<EventType>,
    pub config: HookConfig,
    pub updated_at: DateTime,
    pub created_at: DateTime,
    pub url: String,
    pub test_url: String,
    pub ping_url: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HookConfig {
    pub content_type: String,
    pub insecure_ssl: String,
    pub secret: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AppHook {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub events: Vec<Event>,
    pub config: HookConfig,
    pub updated_at: DateTime,
    pub created_at: DateTime,
    pub integration_id: u64,
    pub app_id: u64,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PingEvent {
    pub zen: String,
    pub hook_id: u64,
    pub hook: Hook,
    pub repository: Option<Repository>,
    pub sender: Option<User>,
}

impl AppEvent for PingEvent {}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum CommitCommentAction {
    Created,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CommitCommentEvent {
    pub action: CommitCommentAction,

    /// The comment in question.
    pub comment: Comment,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for CommitCommentEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum CreateRefType {
    Repository,
    Branch,
    Tag,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CreateEvent {
    /// The Git ref type.
    pub ref_type: CreateRefType,

    /// The Git ref string.
    ///
    /// `None` if only a repository was created.
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,

    /// The name of the repository's default branch (usually `master`).
    pub master_branch: String,

    /// The repository's current description.
    pub description: Option<String>,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for CreateEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum DeleteRefType {
    Branch,
    Tag,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeleteEvent {
    /// The Git ref type.
    pub ref_type: DeleteRefType,

    /// The Git ref string.
    #[serde(rename = "ref")]
    pub git_ref: String,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for DeleteEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum GitHubAppAuthorizationAction {
    Revoked,
}

/// Triggered when someone revokes their authorization of a GitHub App. A GitHub
/// App receives this webhook by default and cannot unsubscribe from this event.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GitHubAppAuthorizationEvent {
    pub action: GitHubAppAuthorizationAction,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID.
    pub installation: InstallationId,
}

impl AppEvent for GitHubAppAuthorizationEvent {
    fn installation(&self) -> Option<u64> {
        Some(self.installation.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum PageAction {
    Created,
    Edited,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PageEvent {
    pub page_name: String,
    pub title: String,
    pub summary: Option<String>,
    pub action: PageAction,
    pub sha: Oid,
    pub html_url: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GollumEvent {
    /// The pages that were created or edited.
    pub pages: Vec<PageEvent>,

    /// The repository for which the action took place.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for GollumEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum InstallationAction {
    Created,
    Deleted,
    NewPermissionsAccepted,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstallationEvent {
    pub action: InstallationAction,
    pub installation: Installation,
    pub sender: User,
}

impl AppEvent for InstallationEvent {
    fn installation(&self) -> Option<u64> {
        Some(self.installation.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum InstallationRepositoriesAction {
    Added,
    Removed,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstallationRepositoriesEvent {
    pub action: InstallationRepositoriesAction,
    pub installation: Installation,
    pub repository_selection: String,
    pub repositories_added: Vec<ShortRepo>,
    pub repositories_removed: Vec<ShortRepo>,
    pub sender: User,
}

impl AppEvent for InstallationRepositoriesEvent {
    fn installation(&self) -> Option<u64> {
        Some(self.installation.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IssueCommentEvent {
    /// The action that was performed.
    pub action: IssueCommentAction,

    /// The issue associated with the comment.
    pub issue: Issue,

    /// The comment in question.
    pub comment: Comment,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for IssueCommentEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum IssueAction {
    Opened,
    Edited,
    Deleted,
    Transferred,
    Pinned,
    Unpinned,
    Closed,
    Reopened,
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Milestoned,
    Demilestoned,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ChangeFrom {
    pub from: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IssueChanges {
    /// A change to the body, if any.
    pub body: Option<ChangeFrom>,

    /// A change to the title, if any.
    pub title: Option<ChangeFrom>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IssuesEvent {
    /// The action that was performed.
    pub action: IssueAction,

    /// The issue itself.
    pub issue: Issue,

    /// Changes to the issues (if the action is `Edited`).
    pub changes: Option<IssueChanges>,

    /// The label that was added or removed (if the action is `Labeled` or
    /// `Unlabeled`).
    pub label: Option<Label>,

    /// The optional user who was assigned or unassigned from the issue (if the
    /// action is `Assigned` or `Unassigned`).
    pub assignee: Option<User>,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for IssuesEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum LabelAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LabelChanges {
    /// A change to the body, if any.
    pub color: Option<ChangeFrom>,

    /// A change to the title, if any.
    pub name: Option<ChangeFrom>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LabelEvent {
    /// The action that was performed.
    pub action: LabelAction,

    /// The label itself.
    pub label: Label,

    /// Changes to the issues (if the action is `Edited`).
    pub changes: Option<LabelChanges>,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for LabelEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Assigned,
    Unassigned,
    ReviewRequested,
    ReviewRequestRemoved,
    Labeled,
    Unlabeled,
    Opened,
    Edited,
    Closed,
    Reopened,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PullRequestEvent {
    /// The action that was performed. Can be one of "assigned", "unassigned",
    /// "review_requested", "review_request_removed", "labeled", "unlabeled",
    /// "opened", "edited", "closed", or "reopened". If the action is "closed"
    /// and the `merged` key is `false`, the pull request was closed with
    /// unmerged commits. If the action is "closed" and the `merged` key is
    /// `true`, the pull request was merged. While webhooks are also triggered
    /// when a pull request is synchronized, Events API timelines don't include
    /// pull request events with the "synchronize" action.
    pub action: PullRequestAction,

    /// The pull request number.
    pub number: u64,

    /// The pull request itself.
    pub pull_request: PullRequest,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for PullRequestEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewAction {
    Submitted,
    Edited,
    Dismissed,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PullRequestReviewChanges {
    pub body: Option<ChangeFrom>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PullRequestReviewEvent {
    /// The action that was performed.
    pub action: PullRequestReviewAction,

    /// The review that was affected.
    pub review: Review,

    /// Changes to the review if the action is `Edited`.
    pub changes: Option<PullRequestReviewChanges>,

    /// The pull request itself.
    pub pull_request: PullRequest,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for PullRequestReviewEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PullRequestReviewCommentChanges {
    /// A change to the body, if any.
    pub body: Option<ChangeFrom>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PullRequestReviewCommentEvent {
    pub action: PullRequestReviewCommentAction,

    /// The changes to the comment if the action was `Edited`.
    pub changes: Option<PullRequestReviewCommentChanges>,

    /// The pull request itself.
    pub pull_request: PullRequest,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The comment in question.
    pub comment: Comment,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for PullRequestReviewCommentEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pusher {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PushAuthor {
    pub name: String,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PushCommit {
    pub id: Oid,
    pub tree_id: Oid,
    pub distinct: bool,
    pub message: String,
    pub timestamp: DateTime,
    pub url: String,
    pub author: PushAuthor,
    pub committer: PushAuthor,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PushEvent {
    /// The Git ref string that was pushed.
    #[serde(rename = "ref")]
    pub git_ref: String,

    /// The commit hash of the branch before the push.
    pub before: Oid,

    /// The commit hash of the branch after the push.
    pub after: Oid,

    /// `true` if this is a new branch.
    pub created: bool,

    /// `true` if this branch is being deleted.
    pub deleted: bool,

    /// `true` if this was a force-push.
    pub forced: bool,

    pub base_ref: Option<String>,

    /// The URL to compare the changes with.
    pub compare: String,

    /// The list of commits that were pushed.
    pub commits: Vec<PushCommit>,

    /// The new head commit.
    pub head_commit: Option<PushCommit>,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who pushed the branch. This is the same as the sender, except
    /// with less information.
    pub pusher: Pusher,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for PushEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryAction {
    Created,
    Deleted,
    Archived,
    Unarchived,
    Publicized,
    Privatized,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RepositoryEvent {
    /// The action that was performed.
    pub action: RepositoryAction,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for RepositoryEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum WatchAction {
    Started,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WatchEvent {
    /// The action that was performed.
    pub action: WatchAction,

    /// The repository associated with this event.
    pub repository: Repository,

    /// The user who triggered the event.
    pub sender: User,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl AppEvent for WatchEvent {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}
