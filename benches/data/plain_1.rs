// Test data comes from https://github.com/github/rest-api-description
const URLS_1: &[&[u8]] = &[
    b"/repos/octocat-repo/hello-world/git/commits/f14d7debf9775f957cf4f1e8176da0786431f72b",
];

#[allow(dead_code)]
const PATH_1: &[u8] = URLS_1[URLS_1.len() - 1];