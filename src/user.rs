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

/// Information about a user.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct User<'a> {
    #[serde(borrow)]
    pub login: &'a str,
    pub id: u64,
    #[serde(borrow)]
    pub avatar_url: &'a str,
    #[serde(borrow)]
    pub gravatar_id: &'a str,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    #[serde(borrow)]
    pub followers_url: &'a str,
    #[serde(borrow)]
    pub following_url: &'a str,
    #[serde(borrow)]
    pub gists_url: &'a str,
    #[serde(borrow)]
    pub starred_url: &'a str,
    #[serde(borrow)]
    pub subscriptions_url: &'a str,
    #[serde(borrow)]
    pub organizations_url: &'a str,
    #[serde(borrow)]
    pub repos_url: &'a str,
    #[serde(borrow)]
    pub events_url: &'a str,
    #[serde(borrow)]
    pub received_events_url: &'a str,
    pub site_admin: bool,
}

/// Information about the current authenticated user.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthenticatedUser<'a> {
    #[serde(borrow)]
    pub login: &'a str,
    pub id: u64,
    #[serde(borrow)]
    pub avatar_url: &'a str,
    #[serde(borrow)]
    pub gravatar_id: &'a str,
    #[serde(borrow)]
    pub url: &'a str,
    #[serde(borrow)]
    pub html_url: &'a str,
    #[serde(borrow)]
    pub followers_url: &'a str,
    #[serde(borrow)]
    pub following_url: &'a str,
    #[serde(borrow)]
    pub gists_url: &'a str,
    #[serde(borrow)]
    pub starred_url: &'a str,
    #[serde(borrow)]
    pub subscriptions_url: &'a str,
    #[serde(borrow)]
    pub organizations_url: &'a str,
    #[serde(borrow)]
    pub repos_url: &'a str,
    #[serde(borrow)]
    pub events_url: &'a str,
    #[serde(borrow)]
    pub received_events_url: &'a str,
    pub site_admin: bool,

    // Extended over `User`:
    #[serde(borrow)]
    pub name: Option<&'a str>,
    #[serde(borrow)]
    pub company: Option<&'a str>,
    #[serde(borrow)]
    pub blog: &'a str,
    #[serde(borrow)]
    pub location: Option<&'a str>,
    #[serde(borrow)]
    pub email: Option<&'a str>,
    pub hireable: Option<bool>,
    #[serde(borrow)]
    pub bio: Option<&'a str>,
}
