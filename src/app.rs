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

use crate::{DateTime, EventType, User};

pub trait AppEvent {
    /// Returns the installation ID for the event.
    fn installation(&self) -> Option<u64> {
        None
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Read,
    Write,
}

/// Permissions given to the app installation.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstallationPermissions {
    pub issues: Option<Permission>,
    pub contents: Option<Permission>,
    pub pull_requests: Option<Permission>,
    pub metadata: Option<Permission>,
}

/// Information about an app installation.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Installation<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub account: User<'a>,
    #[serde(borrow)]
    pub repository_selection: &'a str,
    #[serde(borrow)]
    pub access_tokens_url: &'a str,
    #[serde(borrow)]
    pub repositories_url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    pub app_id: u64,
    pub target_id: u64,
    #[serde(borrow)]
    pub target_type: &'a str,
    pub permissions: InstallationPermissions,
    pub events: Box<[EventType]>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(borrow)]
    pub single_file_name: Option<&'a str>,
}

/// Information about an app.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct App<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub owner: User<'a>,
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub description: &'a str,
    #[serde(borrow)]
    pub external_url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
