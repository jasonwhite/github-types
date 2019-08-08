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

use derive_more::From;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};

use std::fmt;
use std::str::FromStr;

use crate::{
    AppEvent, CheckRun, CheckSuite, Comment, DateTime, Installation, Issue, Label, Oid,
    PullRequest, Repository, Review, ShortRepo, User,
};

/// GitHub events that are specified in the X-Github-Event header.
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// (Special event.) Any time any event is triggered (Wildcard Event).
    #[serde(rename = "*")]
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

// struct EventTypeVisitor;
// impl<'de> ::serde::de::Visitor<'de> for EventTypeVisitor {
// type Value = EventType;
// fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> fmt::Result {
// write!(formatter, "expecting a static string")
// }
// fn visit_borrowed_str<E: ::serde::de::Error>(self, v: &'de str) ->
// Result<Self::Value,E> { EventType::from_str(v).map_err(::serde::de::Error::
// custom) }
// }
//
// impl<'de> Deserialize<'de> for EventType {
// fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// where
// D: Deserializer<'de>,
// {
// deserializer.deserialize_str(EventTypeVisitor)
// }
// }

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
            EventType::PullRequestReviewComment => "pull_request_review_comment",
            EventType::Push => "push",
            EventType::Release => "release",
            EventType::Repository => "repository",
            EventType::RepositoryImport => "repository_import",
            EventType::RepositoryVulnerabilityAlert => "repository_vulnerability_alert",
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
            "integration_installation" => Ok(EventType::IntegrationInstallation),
            "installation_repositories" => Ok(EventType::InstallationRepositories),
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
            "pull_request_review_comment" => Ok(EventType::PullRequestReviewComment),
            "pull_request_review" => Ok(EventType::PullRequestReview),
            "push" => Ok(EventType::Push),
            "release" => Ok(EventType::Release),
            "repository" => Ok(EventType::Repository),
            "repository_import" => Ok(EventType::RepositoryImport),
            "repository_vulnerability_alert" => Ok(EventType::RepositoryVulnerabilityAlert),
            "security_advisory" => Ok(EventType::SecurityAdvisory),
            "status" => Ok(EventType::Status),
            "team" => Ok(EventType::Team),
            "team_add" => Ok(EventType::TeamAdd),
            "watch" => Ok(EventType::Watch),
            _ => Err("invalid GitHub event"),
        }
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
#[derive(Deserialize, From, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum Event<'a> {
    #[serde(borrow)]
    Ping(PingEvent<'a>),
    #[serde(borrow)]
    CheckRun(CheckRunEvent<'a>),
    #[serde(borrow)]
    CheckSuite(CheckSuiteEvent<'a>),
    #[serde(borrow)]
    CommitComment(CommitCommentEvent<'a>),
    // ContentReference(ContentReferenceEvent),
    #[serde(borrow)]
    Create(CreateEvent<'a>),
    #[serde(borrow)]
    Delete(DeleteEvent<'a>),
    // Deployment(DeploymentEvent),
    // DeploymentStatus(DeploymentStatusEvent),
    // Fork(ForkEvent),
    #[serde(borrow)]
    GitHubAppAuthorization(GitHubAppAuthorizationEvent<'a>),
    #[serde(borrow)]
    Gollum(GollumEvent<'a>),
    #[serde(borrow)]
    Installation(InstallationEvent<'a>),
    #[serde(borrow)]
    InstallationRepositories(InstallationRepositoriesEvent<'a>),
    IntegrationInstallation(IntegrationInstallationEvent),
    IntegrationInstallationRepositories(IntegrationInstallationRepositoriesEvent),
    #[serde(borrow)]
    IssueComment(IssueCommentEvent<'a>),
    #[serde(borrow)]
    Issues(IssuesEvent<'a>),
    #[serde(borrow)]
    Label(LabelEvent<'a>),
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
    #[serde(borrow)]
    PullRequest(PullRequestEvent<'a>),
    #[serde(borrow)]
    PullRequestReview(PullRequestReviewEvent<'a>),
    #[serde(borrow)]
    PullRequestReviewComment(PullRequestReviewCommentEvent<'a>),
    #[serde(borrow)]
    Push(PushEvent<'a>),
    // Release(ReleaseEvent),
    #[serde(borrow)]
    Repository(RepositoryEvent<'a>),
    // RepositoryImport(RepositoryImportEvent),
    // RepositoryVulnerabilityAlert(RepositoryVulnerabilityAlertEvent),
    // SecurityAdvisory(SecurityAdvisoryEvent),
    // Status(StatusEvent),
    // Team(TeamEvent),
    // TeamAdd(TeamAddEvent),
    #[serde(borrow)]
    Watch(WatchEvent<'a>),
}

impl<'a> AppEvent for Event<'a> {
    fn installation(&self) -> Option<u64> {
        match self {
            Event::Ping(e) => e.installation(),
            Event::CheckRun(e) => e.installation(),
            Event::CheckSuite(e) => e.installation(),
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
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstallationId {
    pub id: u64,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type")]
pub enum Hook<'a> {
    #[serde(borrow)]
    Repository(RepoHook<'a>),
    #[serde(borrow)]
    App(AppHook<'a>),
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RepoHook<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub name: &'a str,
    pub active: bool,
    pub events: Box<[EventType]>,
    #[serde(borrow)]
    pub config: HookConfig<'a>,
    pub updated_at: DateTime,
    pub created_at: DateTime,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub test_url: &'a str,
    #[serde(borrow)]
    pub ping_url: &'a str,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HookConfig<'a> {
    #[serde(borrow)]
    pub content_type: &'a str,
    #[serde(borrow)]
    pub insecure_ssl: &'a str,
    #[serde(borrow)]
    pub secret: Option<&'a str>,
    #[serde(borrow)]
    pub url: &'a str,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AppHook<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub name: &'a str,
    pub active: bool,
    pub events: Box<[EventType]>,
    #[serde(borrow)]
    pub config: HookConfig<'a>,
    pub updated_at: DateTime,
    pub created_at: DateTime,
    pub app_id: u64,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PingEvent<'a> {
    #[serde(borrow)]
    pub zen: &'a str,
    pub hook_id: u64,
    #[serde(borrow)]
    pub hook: Hook<'a>,
    #[serde(borrow)]
    pub repository: Option<Repository<'a>>,
    #[serde(borrow)]
    pub sender: Option<User<'a>>,
}

impl<'a> AppEvent for PingEvent<'a> {}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunEventAction {
    /// A new check run was created.
    Created,

    /// The `status` of the check run is `completed`.
    Completed,

    /// Someone requested to re-run your check run.
    Rerequested,

    /// Someone requested that an action be taken. For example, this `action`
    /// will be sent if someone clicks a "Fix it" button in the UI.
    RequestedAction,
}

/// See: https://developer.github.com/v3/activity/events/types/#checkrunevent
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckRunEvent<'a> {
    /// The action performed.
    pub action: CheckRunEventAction,

    /// The check run.
    #[serde(borrow)]
    pub check_run: CheckRun<'a>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID.
    pub installation: InstallationId,
}

impl<'a> AppEvent for CheckRunEvent<'a> {
    fn installation(&self) -> Option<u64> {
        Some(self.installation.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteEventAction {
    Completed,
    Requested,
    Rerequested,
}

impl CheckSuiteEventAction {
    /// Returns `true` if the action indicates that the check suite is
    /// completed.
    pub fn is_completed(self) -> bool {
        match self {
            CheckSuiteEventAction::Completed => false,
            _ => false,
        }
    }

    /// Returns `true` if the action indicates that the check suite has been
    /// requested or re-requested.
    pub fn is_requested(self) -> bool {
        match self {
            CheckSuiteEventAction::Requested | CheckSuiteEventAction::Rerequested => true,
            _ => false,
        }
    }
}

/// See: https://developer.github.com/v3/activity/events/types/#checkrunevent
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckSuiteEvent<'a> {
    /// The action performed.
    pub action: CheckSuiteEventAction,

    /// The check suite.
    #[serde(borrow)]
    pub check_suite: CheckSuite<'a>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID.
    pub installation: InstallationId,
}

impl<'a> CheckSuiteEvent<'a> {
    /// Returns `true` if this event indicates that a check suite was requested.
    pub fn is_requested(&self) -> bool {
        self.action.is_requested()
    }
}

impl<'a> AppEvent for CheckSuiteEvent<'a> {
    fn installation(&self) -> Option<u64> {
        Some(self.installation.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CommitCommentAction {
    Created,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CommitCommentEvent<'a> {
    pub action: CommitCommentAction,

    /// The comment in question.
    #[serde(borrow)]
    pub comment: Comment<'a>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for CommitCommentEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CreateRefType {
    Repository,
    Branch,
    Tag,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CreateEvent<'a> {
    /// The Git ref type.
    pub ref_type: CreateRefType,

    /// The Git ref string.
    ///
    /// `None` if only a repository was created.
    #[serde(borrow, rename = "ref")]
    pub git_ref: Option<&'a str>,

    /// The name of the repository's default branch (usually `master`).
    #[serde(borrow)]
    pub master_branch: &'a str,

    /// The repository's current description.
    #[serde(borrow)]
    pub description: Option<&'a str>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for CreateEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DeleteRefType {
    Branch,
    Tag,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeleteEvent<'a> {
    /// The Git ref type.
    pub ref_type: DeleteRefType,

    /// The Git ref string.
    #[serde(borrow, rename = "ref")]
    pub git_ref: &'a str,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for DeleteEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GitHubAppAuthorizationAction {
    Revoked,
}

/// Triggered when someone revokes their authorization of a GitHub App. A GitHub
/// App receives this webhook by default and cannot unsubscribe from this event.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GitHubAppAuthorizationEvent<'a> {
    pub action: GitHubAppAuthorizationAction,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID.
    pub installation: InstallationId,
}

impl<'a> AppEvent for GitHubAppAuthorizationEvent<'a> {
    fn installation(&self) -> Option<u64> {
        Some(self.installation.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PageAction {
    Created,
    Edited,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PageEvent<'a> {
    #[serde(borrow)]
    pub page_name: &'a str,
    #[serde(borrow)]
    pub title: &'a str,
    #[serde(borrow)]
    pub summary: Option<&'a str>,
    pub action: PageAction,
    pub sha: Oid,
    #[serde(borrow)]
    pub html_url: &'a str,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GollumEvent<'a> {
    /// The pages that were created or edited.
    #[serde(borrow)]
    pub pages: Box<[PageEvent<'a>]>,

    /// The repository for which the action took place.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for GollumEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum InstallationAction {
    Created,
    Deleted,
    NewPermissionsAccepted,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstallationEvent<'a> {
    pub action: InstallationAction,
    #[serde(borrow)]
    pub installation: Installation<'a>,
    #[serde(borrow)]
    pub sender: User<'a>,
}

impl<'a> AppEvent for InstallationEvent<'a> {
    fn installation(&self) -> Option<u64> {
        Some(self.installation.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum InstallationRepositoriesAction {
    Added,
    Removed,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstallationRepositoriesEvent<'a> {
    pub action: InstallationRepositoriesAction,
    #[serde(borrow)]
    pub installation: Installation<'a>,
    #[serde(borrow)]
    pub repository_selection: &'a str,
    #[serde(borrow)]
    pub repositories_added: Box<[ShortRepo<'a>]>,
    #[serde(borrow)]
    pub repositories_removed: Box<[ShortRepo<'a>]>,
    #[serde(borrow)]
    pub sender: User<'a>,
}

impl<'a> AppEvent for InstallationRepositoriesEvent<'a> {
    fn installation(&self) -> Option<u64> {
        Some(self.installation.id)
    }
}

/// Event deprecated by GitHub. Use `InstallationEvent` instead.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IntegrationInstallationEvent {}

impl AppEvent for IntegrationInstallationEvent {
    fn installation(&self) -> Option<u64> {
        None
    }
}

/// Event deprecated by GitHub. Use `InstallationRepositoriesEvent` instead.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IntegrationInstallationRepositoriesEvent {}

impl AppEvent for IntegrationInstallationRepositoriesEvent {
    fn installation(&self) -> Option<u64> {
        None
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IssueCommentEvent<'a> {
    /// The action that was performed.
    pub action: IssueCommentAction,

    /// The issue associated with the comment.
    #[serde(borrow)]
    pub issue: Issue<'a>,

    /// The comment in question.
    #[serde(borrow)]
    pub comment: Comment<'a>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for IssueCommentEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
pub struct IssuesEvent<'a> {
    /// The action that was performed.
    pub action: IssueAction,

    /// The issue itself.
    #[serde(borrow)]
    pub issue: Issue<'a>,

    /// Changes to the issues (if the action is `Edited`).
    pub changes: Option<IssueChanges>,

    /// The label that was added or removed (if the action is `Labeled` or
    /// `Unlabeled`).
    #[serde(borrow)]
    pub label: Option<Label<'a>>,

    /// The optional user who was assigned or unassigned from the issue (if the
    /// action is `Assigned` or `Unassigned`).
    #[serde(borrow)]
    pub assignee: Option<User<'a>>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for IssuesEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
pub struct LabelEvent<'a> {
    /// The action that was performed.
    pub action: LabelAction,

    /// The label itself.
    #[serde(borrow)]
    pub label: Label<'a>,

    /// Changes to the issues (if the action is `Edited`).
    pub changes: Option<LabelChanges>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for LabelEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
    ReadyForReview,
    Locked,
    Unlocked,
    Reopened,
    Synchronize,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PullRequestEvent<'a> {
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
    #[serde(borrow)]
    pub pull_request: PullRequest<'a>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for PullRequestEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
pub struct PullRequestReviewEvent<'a> {
    /// The action that was performed.
    pub action: PullRequestReviewAction,

    /// The review that was affected.
    #[serde(borrow)]
    pub review: Review<'a>,

    /// Changes to the review if the action is `Edited`.
    pub changes: Option<PullRequestReviewChanges>,

    /// The pull request itself.
    #[serde(borrow)]
    pub pull_request: PullRequest<'a>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for PullRequestReviewEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
pub struct PullRequestReviewCommentEvent<'a> {
    pub action: PullRequestReviewCommentAction,

    /// The changes to the comment if the action was `Edited`.
    pub changes: Option<PullRequestReviewCommentChanges>,

    /// The pull request itself.
    #[serde(borrow)]
    pub pull_request: PullRequest<'a>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The comment in question.
    #[serde(borrow)]
    pub comment: Comment<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for PullRequestReviewCommentEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pusher<'a> {
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub email: Option<&'a str>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PushAuthor<'a> {
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub email: Option<&'a str>,
    #[serde(borrow)]
    pub username: Option<&'a str>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PushCommit<'a> {
    pub id: Oid,
    pub tree_id: Oid,
    pub distinct: bool,
    #[serde(borrow)]
    pub message: &'a str,
    pub timestamp: DateTime,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub author: PushAuthor<'a>,
    #[serde(borrow)]
    pub committer: PushAuthor<'a>,
    #[serde(borrow)]
    pub added: Box<[&'a str]>,
    #[serde(borrow)]
    pub removed: Box<[&'a str]>,
    #[serde(borrow)]
    pub modified: Box<[&'a str]>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PushEvent<'a> {
    /// The Git ref string that was pushed.
    #[serde(borrow, rename = "ref")]
    pub git_ref: &'a str,

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

    #[serde(borrow)]
    pub base_ref: Option<&'a str>,

    /// The URL to compare the changes with.
    #[serde(borrow)]
    pub compare: &'a str,

    /// The list of commits that were pushed.
    #[serde(borrow)]
    pub commits: Box<[PushCommit<'a>]>,

    /// The new head commit.
    #[serde(borrow)]
    pub head_commit: Option<PushCommit<'a>>,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who pushed the branch. This is the same as the sender, except
    /// with less information.
    #[serde(borrow)]
    pub pusher: Pusher<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for PushEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
pub struct RepositoryEvent<'a> {
    /// The action that was performed.
    pub action: RepositoryAction,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for RepositoryEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WatchAction {
    Started,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WatchEvent<'a> {
    /// The action that was performed.
    pub action: WatchAction,

    /// The repository associated with this event.
    #[serde(borrow)]
    pub repository: Repository<'a>,

    /// The user who triggered the event.
    #[serde(borrow)]
    pub sender: User<'a>,

    /// The App installation ID. This is only present for GitHub App events.
    pub installation: Option<InstallationId>,
}

impl<'a> AppEvent for WatchEvent<'a> {
    fn installation(&self) -> Option<u64> {
        self.installation.map(|i| i.id)
    }
}
