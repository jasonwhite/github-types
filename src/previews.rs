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

use std::fmt;

/// API previews.
///
/// See: https://developer.github.com/v3/previews/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Preview {
    /// Allows you to download repositories from your GitHub user or
    /// organization account to review, backup, and migrate data to GitHub
    /// Enterprise Server.
    Wyandotte,

    /// Import source repositories to GitHub with the API version of the GitHub
    /// Importer.
    BarredRock,

    /// Exercise greater control over deployments with more information and
    /// finer granularity.
    AntMan,

    /// Manage reactions for commits, issues, and comments.
    SquirrelGirl,

    /// Get a list of events for an issue or pull request.
    Mockingbird,

    /// Get more information about your GitHub Pages site.
    MisterFantastic,

    /// Manage integrations through the API.
    MachineMan,

    /// Manage projects.
    Inertia,

    /// Search commits.
    Cloak,

    /// Retrieve community profile metrics (also known as community health) for
    /// any public repository.
    BlackPanther,

    /// Users can block other users. Organizations can block users, too.
    GiantSentryFist,

    /// View a list of repository topics in calls that return repository
    /// results.
    Mercy,

    /// View all codes of conduct or get which code of conduct a repository has
    /// currently.
    ScarletWitch,

    /// Include nested team content in team payloads.
    Hellcat,

    /// Transfer a repository to an organization or user.
    Nightshade,

    /// You can now add a reason when you lock an issue.
    SailorV,

    /// You can now use the API to invite new users to an organization by
    /// creating an organization invitation.
    Dazzler,

    /// You can now use the API to manage team discussions and team discussion
    /// comments.
    Echo,

    /// You can now use emoji in label names, add descriptions to labels, and
    /// search for labels in a repository.
    Symmetra,

    /// You can now use the API to manage the setting for requiring signed
    /// commits on protected branches.
    Zzzax,

    /// You can now require multiple approving reviews for a pull request using
    /// the API.
    LukeCage,

    /// Retrieve information from someone's hovercard.
    Hagar,

    /// Allows a GitHub App to run external checks on a repository's code. See
    /// the Check runs and Check suites APIs for more details.
    Antiope,

    /// The REST API v3 responses for issue events and issue timeline events
    /// now return the project_card field for project-related events.
    Starfox,

    /// GitHub App Manifests allow people to create preconfigured GitHub Apps.
    /// See "Creating GitHub Apps from a manifest" for more details.
    Fury,

    /// You can now update the `environment` of a deployment status and use the
    /// `in_progress` and `queued` states. When you create deployment statuses,
    /// you can now use the `auto_inactive` parameter to mark old `production`
    /// deployments as `inactive`.
    Flash,

    /// You can now configure whether organization members can create
    /// repositories and which types of repositories they can create. See "Edit
    /// an organization" for more details.
    Surtur,

    /// You can now provide more information in GitHub for URLs that link to
    /// registered domains by using the Content Attachments API. See "Using
    /// content attachments" for more details.
    Corsair,

    /// Allows you to temporarily restrict interactions, such as commenting,
    /// opening issues, and creating pull requests, for GitHub repositories or
    /// organizations. When enabled, only the specified group of GitHub users
    /// will be able to participate in these interactions. See the Repository
    /// interactions and Organization interactions APIs for more details.
    Sombra,

    /// You can use the Draft Pull Requests API and its pull request endpoints
    /// to see whether a pull request is in draft state. To learn more about
    /// draft pull requests, see "About pull requests" in the GitHub Help
    /// documentation.
    ShadowCat,

    /// You can use the new endpoints in the Pages API to enable or disable
    /// Pages. To learn more about Pages, see "GitHub Pages Basics" in the
    /// GitHub Help documentation.
    Switcheroo,

    /// You can use the new endpoints in the Commits API to list branches or
    /// pull requests for a commit.
    Groot,

    /// Owners of GitHub Apps can now uninstall an app using the Apps API.
    Gambit,
}

impl Preview {
    /// Returns the kebab-case name of the preview.
    pub fn name(self) -> &'static str {
        match self {
            Preview::Wyandotte => "wyandotte",
            Preview::BarredRock => "barred-rock",
            Preview::AntMan => "ant-man",
            Preview::SquirrelGirl => "squirrel-girl",
            Preview::Mockingbird => "mocking-bird",
            Preview::MisterFantastic => "mister-fantastic",
            Preview::MachineMan => "machine-man",
            Preview::Inertia => "inertia",
            Preview::Cloak => "clock",
            Preview::BlackPanther => "black-panther",
            Preview::GiantSentryFist => "giant-sentry-fist",
            Preview::Mercy => "mercy",
            Preview::ScarletWitch => "scarlet-witch",
            Preview::Hellcat => "hellcat",
            Preview::Nightshade => "nightshade",
            Preview::SailorV => "sailor-v",
            Preview::Dazzler => "dazzler",
            Preview::Echo => "echo",
            Preview::Symmetra => "symmetra",
            Preview::Zzzax => "zzzax",
            Preview::LukeCage => "luke-cage",
            Preview::Hagar => "hagar",
            Preview::Antiope => "antiope",
            Preview::Starfox => "starfox",
            Preview::Fury => "fury",
            Preview::Flash => "flash",
            Preview::Surtur => "surtur",
            Preview::Corsair => "corsair",
            Preview::Sombra => "sombra",
            Preview::ShadowCat => "shadow-cat",
            Preview::Switcheroo => "switcheroo",
            Preview::Groot => "groot",
            Preview::Gambit => "gambit",
        }
    }

    /// Returns the media type for the preview. This can be used for the
    /// `Accept` header in requests.
    pub fn media_type(self) -> String {
        format!("application/vnd.github.{}-preview+json", self.name())
    }
}

impl fmt::Display for Preview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}
