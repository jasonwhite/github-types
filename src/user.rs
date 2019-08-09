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

#[test]
fn test_user_parsing() {
    // value from: https://developer.github.com/v3/pulls/#list-pull-requests
    test_generator! {
        TypeName: User;
        TestData: r#"{
        "login": "hubot",
        "id": 1,
        "node_id": "MDQ6VXNlcjE=",
        "avatar_url": "https://github.com/images/error/hubot_happy.gif",
        "gravatar_id": "",
        "url": "https://api.github.com/users/hubot",
        "html_url": "https://github.com/hubot",
        "followers_url": "https://api.github.com/users/hubot/followers",
        "following_url": "https://api.github.com/users/hubot/following{/other_user}",
        "gists_url": "https://api.github.com/users/hubot/gists{/gist_id}",
        "starred_url": "https://api.github.com/users/hubot/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/hubot/subscriptions",
        "organizations_url": "https://api.github.com/users/hubot/orgs",
        "repos_url": "https://api.github.com/users/hubot/repos",
        "events_url": "https://api.github.com/users/hubot/events{/privacy}",
        "received_events_url": "https://api.github.com/users/hubot/received_events",
        "type": "User",
        "site_admin": true
      }"#;
    assert!(login == "hubot");
    assert!(id == 1u64);
    assert!(avatar_url == "https://github.com/images/error/hubot_happy.gif");
    assert!(gravatar_id == "");
        assert!(url == "https://api.github.com/users/hubot");
        assert!(html_url == "https://github.com/hubot");
        assert!(followers_url == "https://api.github.com/users/hubot/followers");
        assert!(following_url == "https://api.github.com/users/hubot/following{/other_user}");
        assert!(gists_url == "https://api.github.com/users/hubot/gists{/gist_id}");
        assert!(starred_url == "https://api.github.com/users/hubot/starred{/owner}{/repo}");
        assert!(subscriptions_url == "https://api.github.com/users/hubot/subscriptions");
        assert!(organizations_url == "https://api.github.com/users/hubot/orgs");
        assert!(repos_url == "https://api.github.com/users/hubot/repos");
        assert!(events_url == "https://api.github.com/users/hubot/events{/privacy}");
        assert!(received_events_url == "https://api.github.com/users/hubot/received_events");
        assert!(site_admin == true);
    }
}
