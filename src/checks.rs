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

use serde::Deserialize;

use crate::{App, DateTime, Oid};

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunStatus {
    Queued,
    InProgress,
    Completed,
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum Conclusion {
    Success,
    Failure,
    Neutral,
    Cancelled,
    TimedOut,
    ActionRequired,
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationLevel {
    Notice,
    Warning,
    Failure,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Annotation {
    /// Required. The path of the file to add an annotation to. For example,
    /// `assets/css/main.css`.
    pub path: String,

    /// Required. The start line of the annotation.
    pub start_line: u32,

    /// Required. The end line of the annotation.
    pub end_line: u32,

    /// The start column of the annotation. Annotations only support
    /// `start_column` and `end_column` on the same line. Omit this parameter
    /// if `start_line` and `end_line` have different values.
    pub start_column: Option<u32>,

    /// The end column of the annotation. Annotations only support
    /// `start_column` and `end_column` on the same line. Omit this parameter
    /// if `start_line` and `end_line` have different values.
    pub end_column: Option<u32>,

    /// Required. The level of annotation.
    pub annotation_level: AnnotationLevel,

    /// Required. A short description of the feedback for these lines of code.
    /// The maximum size is 64 KB.
    pub message: String,

    /// The title that represents the annotation. The maximum size is 255
    /// characters.
    pub title: Option<String>,

    /// Raw details about this annotation. The maximum size is 64 KB.
    pub raw_details: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Image {
    /// Required. The alternative text for the image.
    pub alt: String,

    /// Required. The full URL of the image.
    pub image_url: String,

    /// A short image description.
    pub caption: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Output {
    /// The title of the check run.
    pub title: String,

    /// The summary of the check run. This parameter supports Markdown.
    pub summary: String,

    /// The details of the check run. This parameter supports Markdown.
    pub text: Option<String>,

    /// Adds information from your analysis to specific lines of code.
    /// Annotations are visible on GitHub in the *Checks* and *Files changed*
    /// tab of the pull request. The Checks API limits the number of
    /// annotations to a maximum of 50 per API request. To create more than
    /// 50 annotations, you have to make multiple requests to the [Update a
    /// check run][1] endpoint.  Each time you update the check run,
    /// annotations are appended to the list of annotations that already
    /// exist for the check run. For details about how you can view
    /// annotations on GitHub, see "[About status checks][2]".
    ///
    /// [1]: https://developer.github.com/v3/checks/runs/#update-a-check-run
    /// [2]: https://help.github.com/articles/about-status-checks#checks
    pub annotations: Option<Vec<Annotation>>,

    /// Adds images to the output displayed in the GitHub pull request UI.
    pub images: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckRunAction {
    /// The text to be displayed on a button in the web UI. The maximum size is
    /// 20 characters.
    pub label: String,

    /// A short explanation of what this action would do. The maximum size is
    /// 40 characters.
    pub description: String,

    /// A reference for the action on the integrator's system. The maximum size
    /// is 20 characters.
    pub identifier: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckSuiteId {
    pub id: u64,
}

/// A repo associated with a `CheckRun`.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckRunRepo {
    pub id: u64,
    pub url: String,
    pub name: String,
}

/// A commit associated with a `CheckRun`.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckRunCommit {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub sha: Oid,
    pub repo: CheckRunRepo,
}

/// A pull request associated with a `CheckRun`.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckRunPullRequest {
    pub url: String,
    pub id: u64,
    pub number: u64,
    pub head: CheckRunCommit,
    pub base: CheckRunCommit,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckRun {
    /// The ID of the check run.
    pub id: u64,

    /// The name of the check run.
    pub name: String,

    /// The URL of the integrator's site that has the full details of the
    /// check.
    pub head_sha: Oid,

    /// A reference for the run on the integrator's system.
    pub external_id: String,

    pub url: String,

    pub html_url: String,

    /// The current status.
    pub status: CheckRunStatus,

    /// `None` until the check run has a `Completed` status.
    pub conclusion: Option<Conclusion>,

    /// The time that the check run began.
    pub started_at: Option<DateTime>,

    /// The time the check completed.
    pub completed_at: Option<DateTime>,

    pub output: Option<Output>,

    pub check_suite: CheckSuiteId,

    pub app: App,

    pub pull_requests: Vec<CheckRunPullRequest>,

    /// Possible further actions the integrator can perform, which a user may
    /// trigger. A maximum of three actions are accepted.
    pub actions: Option<Vec<CheckRunAction>>,
}

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteStatus {
    Requested,
    InProgress,
    Completed,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckSuite {
    pub id: u64,

    /// The head branch name of the changes are on.
    ///
    /// This is `None` if head branch is in a forked repository.
    pub head_branch: Option<String>,

    /// The SHA of the most recent commit for this check suite.
    pub head_sha: Oid,

    /// The summary status for all check runs that are part of the check suite.
    pub status: CheckSuiteStatus,

    /// The summary conclusion for all check runs that are part of the check
    /// suite. This will be `None` until the status is `Completed`.
    pub conclusion: Option<Conclusion>,

    /// URL that points to the check suite API resource.
    pub url: String,

    /// The commit SHA of the previous commit. If this is a new branch, this
    /// will be `Oid::zero()`.
    pub before: Oid,

    /// The commit SHA of the new commit.
    pub after: Oid,

    /// An array of pull requests that match this check suite. A pull request
    /// matches a check suite if they have the same `head_sha` and
    /// `head_branch`. When the check suite's `head_branch` is in a forked
    /// repository it will be `None` and the `pull_requests` array will be
    /// empty.
    pub pull_requests: Vec<CheckRunPullRequest>,

    pub app: App,
}
