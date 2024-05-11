// Test data comes from https://github.com/github/rest-api-description
const PLAIN_URLS_16: &[&[u8]] = &[
    b"/repos/octocat-repo/hello-world",
    b"/feeds",
    b"/repos/octokit/octokit.rb/issues/123",
    b"/repos/octocat/Hello-World-Template/subscription",
    b"/repos/octocat/Hello-World/branches/master/protection/required_status_checks",
    b"/repos/octocat/Hello-World/hooks/1/test",
    b"/users/Octocoders/orgs",
    b"/repos/repo/a-package/security-advisories/GHSA-1234-5678-9012",
    b"/licenses/unlicense",
    b"/users/octokitten/repos",
    b"/repos/Codertocat/Hello-World",
    b"/marketplace_listing/plans/1313/accounts",
    b"/repos/octocat/Hello-World/git/commits/6dcb09b5b57875f334f61aebed695e2e4193db5e",
    b"/repos/octocat/octo-name-repo/downloads",
    b"/repos/github/hello-world/pages/builds/latest",
    b"/repos/octocat-repo/hello-world/git/commits/f14d7debf9775f957cf4f1e8176da0786431f72b",
];

#[allow(dead_code)]
const PLAIN_PATH_16: &[u8] = PLAIN_URLS_16[PLAIN_URLS_16.len() - 1];