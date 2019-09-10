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

use crate::{DateTime, Oid, User};

/// Short info about a repository.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShortRepo<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub full_name: &'a str,
    pub private: bool,
}

/// A repository.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Repository<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub owner: User<'a>,
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub full_name: &'a str,
    #[serde(borrow)]
    pub description: Option<&'a str>,
    pub private: bool,
    pub fork: bool,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    #[serde(borrow)]
    pub archive_url: &'a str,
    #[serde(borrow)]
    pub assignees_url: &'a str,
    #[serde(borrow)]
    pub blobs_url: &'a str,
    #[serde(borrow)]
    pub branches_url: &'a str,
    #[serde(borrow)]
    pub clone_url: &'a str,
    #[serde(borrow)]
    pub collaborators_url: &'a str,
    #[serde(borrow)]
    pub comments_url: &'a str,
    #[serde(borrow)]
    pub commits_url: &'a str,
    #[serde(borrow)]
    pub compare_url: &'a str,
    #[serde(borrow)]
    pub contents_url: &'a str,
    #[serde(borrow)]
    pub contributors_url: &'a str,
    #[serde(borrow)]
    pub deployments_url: &'a str,
    #[serde(borrow)]
    pub downloads_url: &'a str,
    #[serde(borrow)]
    pub events_url: &'a str,
    #[serde(borrow)]
    pub forks_url: &'a str,
    #[serde(borrow)]
    pub git_commits_url: &'a str,
    #[serde(borrow)]
    pub git_refs_url: &'a str,
    #[serde(borrow)]
    pub git_tags_url: &'a str,
    #[serde(borrow)]
    pub git_url: &'a str,
    #[serde(borrow)]
    pub hooks_url: &'a str,
    #[serde(borrow)]
    pub issue_comment_url: &'a str,
    #[serde(borrow)]
    pub issue_events_url: &'a str,
    #[serde(borrow)]
    pub issues_url: &'a str,
    #[serde(borrow)]
    pub keys_url: &'a str,
    #[serde(borrow)]
    pub labels_url: &'a str,
    #[serde(borrow)]
    pub languages_url: &'a str,
    #[serde(borrow)]
    pub merges_url: &'a str,
    #[serde(borrow)]
    pub milestones_url: &'a str,
    #[serde(borrow)]
    pub mirror_url: Option<&'a str>,
    #[serde(borrow)]
    pub notifications_url: &'a str,
    #[serde(borrow)]
    pub pulls_url: &'a str,
    #[serde(borrow)]
    pub releases_url: &'a str,
    #[serde(borrow)]
    pub ssh_url: &'a str,
    #[serde(borrow)]
    pub stargazers_url: &'a str,
    #[serde(borrow)]
    pub statuses_url: &'a str,
    #[serde(borrow)]
    pub subscribers_url: &'a str,
    #[serde(borrow)]
    pub subscription_url: &'a str,
    #[serde(borrow)]
    pub svn_url: &'a str,
    #[serde(borrow)]
    pub tags_url: &'a str,
    #[serde(borrow)]
    pub teams_url: &'a str,
    #[serde(borrow)]
    pub trees_url: &'a str,
    #[serde(borrow)]
    pub homepage: Option<&'a str>,
    #[serde(borrow)]
    pub language: Option<&'a str>,
    pub forks_count: u64,
    pub stargazers_count: u64,
    pub watchers_count: u64,
    pub size: u64,
    #[serde(borrow)]
    pub default_branch: &'a str,
    pub open_issues_count: u64,
    pub has_issues: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub archived: bool,
    pub pushed_at: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Comment<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    #[serde(borrow)]
    pub body: &'a str,
    #[serde(borrow)]
    pub user: User<'a>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PullRequest<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    #[serde(borrow)]
    pub diff_url: &'a str,
    #[serde(borrow)]
    pub patch_url: &'a str,
    #[serde(borrow)]
    pub issue_url: &'a str,
    #[serde(borrow)]
    pub commits_url: &'a str,
    #[serde(borrow)]
    pub review_comments_url: &'a str,
    #[serde(borrow)]
    pub review_comment_url: &'a str,
    #[serde(borrow)]
    pub comments_url: &'a str,
    #[serde(borrow)]
    pub statuses_url: &'a str,
    pub number: u64,
    #[serde(borrow)]
    pub state: &'a str,
    #[serde(borrow)]
    pub title: &'a str,
    #[serde(borrow)]
    pub body: Option<&'a str>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub closed_at: Option<DateTime>,
    pub merged_at: Option<DateTime>,
    #[serde(borrow)]
    pub head: ShortCommit<'a>,
    #[serde(borrow)]
    pub base: ShortCommit<'a>,
    pub user: User<'a>,
    #[serde(borrow)]
    pub assignee: Option<User<'a>>,
    #[serde(borrow)]
    pub assignees: Box<[User<'a>]>,
    #[serde(borrow)]
    pub merge_commit_sha: Option<&'a str>,
    pub merged: bool,
    pub mergeable: Option<bool>,
    #[serde(borrow)]
    pub merged_by: Option<User<'a>>,
    pub comments: Option<u64>,
    pub commits: Option<u64>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
    pub changed_files: Option<u64>,
    #[serde(borrow)]
    pub labels: Box<[Label<'a>]>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShortCommit<'a> {
    #[serde(borrow)]
    pub label: &'a str,
    #[serde(rename = "ref", borrow)]
    pub git_ref: &'a str,
    pub sha: Oid,
    #[serde(borrow)]
    pub user: User<'a>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Label<'a> {
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub color: &'a str,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Issue<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub labels_url: &'a str,
    #[serde(borrow)]
    pub comments_url: &'a str,
    #[serde(borrow)]
    pub events_url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    pub number: u64,
    #[serde(borrow)]
    pub state: &'a str,
    #[serde(borrow)]
    pub title: &'a str,
    #[serde(borrow)]
    pub body: Option<&'a str>,
    #[serde(borrow)]
    pub user: User<'a>,
    #[serde(borrow)]
    pub labels: Box<[Label<'a>]>,
    #[serde(borrow)]
    pub assignee: Option<User<'a>>,
    pub locked: bool,
    pub comments: u64,
    #[serde(borrow)]
    pub pull_request: Option<PullRef<'a>>,
    pub closed_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(borrow)]
    pub assignees: Box<[User<'a>]>,
}

/// A reference to a pull request.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PullRef<'a> {
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    #[serde(borrow)]
    pub diff_url: &'a str,
    #[serde(borrow)]
    pub patch_url: &'a str,
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    Commented,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Review<'a> {
    pub id: u64,
    pub user: User<'a>,
    #[serde(borrow)]
    pub body: Option<&'a str>,
    pub commit_id: Oid,
    pub submitted_at: DateTime,
    pub state: ReviewState,
    #[serde(borrow)]
    pub html_url: &'a str,
    #[serde(borrow)]
    pub pull_request_url: &'a str,
    #[serde(borrow)]
    pub author_association: &'a str,
}
