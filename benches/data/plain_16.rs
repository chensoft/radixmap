// Test data comes from https://github.com/github/rest-api-description
const URLS_16: &[&str] = &[
    "/repos/octocat-repo/hello-world",
    "/feeds",
    "/repos/octokit/octokit.rb/issues/123",
    "/repos/octocat/Hello-World-Template/subscription",
    "/repos/octocat/Hello-World/branches/master/protection/required_status_checks",
    "/repos/octocat/Hello-World/hooks/1/test",
    "/users/Octocoders/orgs",
    "/repos/repo/a-package/security-advisories/GHSA-1234-5678-9012",
    "/licenses/unlicense",
    "/users/octokitten/repos",
    "/repos/Codertocat/Hello-World",
    "/marketplace_listing/plans/1313/accounts",
    "/repos/octocat/Hello-World/git/commits/6dcb09b5b57875f334f61aebed695e2e4193db5e",
    "/repos/octocat/octo-name-repo/downloads",
    "/repos/github/hello-world/pages/builds/latest",
    "/repos/octocat-repo/hello-world/git/commits/f14d7debf9775f957cf4f1e8176da0786431f72b",
];

#[allow(dead_code)]
const PATH_16: &str = URLS_16[URLS_16.len() - 1];