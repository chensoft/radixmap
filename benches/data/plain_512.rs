// Test data comes from https://github.com/github/rest-api-description
const PLAIN_URLS_512: &[&[u8]] = &[
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
    b"/repos/octocat/Hello-World/tarball/v1.0.0",
    b"/repos/octocat/Hello-World/git/commits/7d1b31e74ee336d15cbd21741bc88a537ed063a0",
    b"/repos/octocat/Hello-World/git/trees/827efc6d56897b048c772eb4087f854f46256132",
    b"/repos/jquery/jquery/subscription",
    b"/repos/octocat/octo-name-repo/teams",
    b"/repos/octocat/hello-world/branches/main/protection/required_signatures",
    b"/rate_limit",
    b"/repos/octocat/Hello-World/git/commits/612077ae6dffb4d2fbd8ce0cccaa58893b07b5ac",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/tags",
    b"/repos/octocat/octo-name-repo/subscribers",
    b"/repos/octo-org/octo-repo/tags",
    b"/repos/github/hello-world/actions/runs/5/jobs",
    b"/repos/octocat/Hello-World/forks",
    b"/users/hubot/orgs",
    b"/repos/octocat/example/git/blobs/3a0f86fb8db8eea7ccbb9a95f325ddbedfb25e15",
    b"/repos/octo-org/octo-repo/actions/runs/29679449",
    b"/projects/1002603/columns",
    b"/repos/octocat-repo/hello-world/git/blobs/af5626b4a114abcb82d63db7c8082c3c4756e51b",
    b"/organizations/1/team/2343027/discussions/1",
    b"/users/monauser",
    b"/repos/octocat/Hello-World/pulls/1347",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/forks",
    b"/repos/github/developer.github.com/pages/builds/latest",
    b"/user/followers",
    b"/gists/2decf6c462d9b4418f2/comments",
    b"/user",
    b"/users/github-product-roadmap",
    b"/repos/octocat/Hello-World-Template/merges",
    b"/users/monalisa/repos",
    b"/repos/octocat/Hello-World/statuses/6dcb09b5b57875f334f61aebed695e2e4193db5e",
    b"/repos/octocat/Spoon-Knife/languages",
    b"/projects/1002605/columns",
    b"/users/github/subscriptions",
    b"/repos/octo-org/hello-world/teams",
    b"/repos/jquery/jquery/events",
    b"/repos/octo-org/octo-repo/check-suites/414944374",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/hooks",
    b"/repos/octocat/example/subscription",
    b"/users/octocat/orgs",
    b"/repos/octocat/Hello-World/git/blobs/f484d249c660418515fb01c2b9662073663c242e",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/teams",
    b"/users/mojombo/repos",
    b"/repos/Octocoders/Hello-World/forks",
    b"/repos/github/roadmap/issues/events/6430295168",
    b"/repos/octocat/Hello-World/downloads",
    b"/repos/octo-org/octo-repo/actions/workflows/269289",
    b"/users/octokitten/subscriptions",
    b"/users/github/followers",
    b"/repos/octocat/hello-world/code-scanning/alerts/3/instances",
    b"/users/octo-org/orgs",
    b"/repos/octokit/octokit.rb/contents/bin/some-symlink",
    b"/organizations/org/secrets/my_secret/repositories",
    b"/repos/octocat/hello-world/branches/main/protection/restrictions/teams",
    b"/repos/octocat/octo-name-repo/hooks",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/merges",
    b"/repos/github/roadmap/issues/events/6635165802",
    b"/organizations/org/dependabot/secrets/my_secret/repositories",
    b"/repos/the-org/an-org-repo/branches/master/protection/dismissal_restrictions/teams",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/subscription",
    b"/repos/dtrupenn/Tetris/forks",
    b"/users/hubot/repos",
    b"/projects/1002605",
    b"/repos/jquery/jquery/downloads",
    b"/repos/octocat/Hello-World/pulls/1",
    b"/users/jquery",
    b"/repos/octocat/Hello-World/git/commits/7fd1a60b01f91b314f59955a4e4d4e80d8edf11d",
    b"/user/codespaces/monalisa-octocat-hello-world-f8adfad99a/stop",
    b"/user/emails",
    b"/users/octo-org/repos",
    b"/gists/2decf6c462d9b4418f2",
    b"/gists/public",
    b"/users/Octocoders",
    b"/repos/octocat-repo/hello-world/subscribers",
    b"/repos/octocat-repo/hello-world/languages",
    b"/repos/octo-org/octo-repo/merges",
    b"/repos/Codertocat/Hello-World/forks",
    b"/users/octokitten/received_events",
    b"/repos/octocat/Hello-World/releases/1/assets",
    b"/repos/octocat/Spoon-Knife/git/trees/a639e96f9038797fba6e0469f94a4b0cc459fa68",
    b"/repos/octokit/octokit.rb/issues/comments/123",
    b"/users/Codertocat/followers",
    b"/repos/batterseapower/pinyin-toolkit/issues/132/comments",
    b"/repos/octocat/Hello-World/branches/master/protection/dismissal_restrictions/users",
    b"/repos/octocat/example",
    b"/repos/Octocoders/Hello-World/downloads",
    b"/repos/octocat/Hello-World/comments/1",
    b"/gists/a6db0bec360bb87e9418/comments/1",
    b"/repos/octocat/hello-world/git/commits/f14d7debf9775f957cf4f1e8176da0786431f72b",
    b"/repos/octocat-repo/hello-world/merges",
    b"/repos/octocat/Hello-World/releases/1",
    b"/repos/octocat/hello-world/code-scanning/alerts/4",
    b"/users/Nick3C/orgs",
    b"/repos/octo-org/hello-world/subscription",
    b"/repos/Octocoders/Hello-World/contributors",
    b"/organizations/2/invitations/1/teams",
    b"/repos/owner-482f3203ecf01f67e9deb18e/BBB_Private_Repo/git/blobs/23f6827669e43831def8a7ad935069c8bd418261",
    b"/repos/octocat/Hello-World/git/blobs/95b966ae1c166bd92f8ae7d1c313e738c731dfc3",
    b"/repos/octocat/Hello-World/branches/master",
    b"/teams/2",
    b"/users/Octocoders/received_events",
    b"/repos/octocat/Hello-World/compare/master...topic",
    b"/repos/octocat/Hello-World/pulls/1347/comments",
    b"/repos/Codertocat/Hello-World/contributors",
    b"/teams/1/repos",
    b"/repos/octo-org/hello-world/stargazers",
    b"/repos/octocat-repo/hello-world/events",
    b"/user/codespaces/secrets/CODESPACE_GH_SECRET/repositories",
    b"/repos/github/developer.github.com/pages",
    b"/users/github/repos",
    b"/repos/octocat/Hello-World/check-suites/5/check-runs",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/downloads",
    b"/organizations/1/team/1/repos",
    b"/repos/octocat/hello-world/dependabot/alerts/2",
    b"/repos/octocat/Hello-World/issues/comments/1",
    b"/repos/octocat-repo/hello-world/tags",
    b"/licenses/apache-2.0",
    b"/repos/octocat/Hello-World/deployments",
    b"/repos/Codertocat/Hello-World/deployments/326191728/statuses",
    b"/repos/octocat/octo-name-repo/subscription",
    b"/repos/github/hello-world/actions/runs/5/rerun/artifacts",
    b"/app/installations/1/access_tokens",
    b"/repos/dtrupenn/Tetris",
    b"/repos/octocat/Hello-World/labels/bug%20bug",
    b"/repos/octocat-repo/hello-world/deployments",
    b"/repos/octocat/Hello-World/pulls/12",
    b"/orgs/github/packages/container/super-linter",
    b"/repos/octo-org/octo-repo/check-runs/399444496",
    b"/repos/batterseapower/pinyin-toolkit/issues/132/events",
    b"/repos/spraints/socm/import",
    b"/repos/Octocoders/Hello-World/events",
    b"/users/github-product-roadmap/repos",
    b"/repos/octocat/octo-name-repo/merges",
    b"/orgs/github/events",
    b"/users/other_user",
    b"/repos/octocat/Hello-World/commits/6dcb09b5b57875f334f61aebed695e2e4193db5e/comments",
    b"/orgs/github/hooks",
    b"/organizations/16/invitations/1/teams",
    b"/repos/octo-org/octo-repo/actions/runs/30433642",
    b"/repos/octocat/Hello-World-Template/teams",
    b"/repos/Codertocat/Hello-World/check-runs/128620228/annotations",
    b"/repos/octocat/Hello-World/pulls/1347/commits",
    b"/repos/octocat/hello-world/code-scanning/analyses/200",
    b"/users/octo-org/received_events",
    b"/repos/dtrupenn/Tetris/tags",
    b"/users/monalisa/orgs",
    b"/users/monalisa/received_events",
    b"/orgs/github/packages/container/goodbye_docker",
    b"/users/hubot/received_events",
    b"/repos/octo-org/octo-repo/hooks",
    b"/users/Nick3C/repos",
    b"/repos/octocat/Hello-World/git/tags/940bd336248efae0f9ee5bc7b2d5c985887b16ac",
    b"/repos/jquery/jquery/teams",
    b"/repos/Octocoders/Hello-World/languages",
    b"/repos/octo-org/hello-world/deployments",
    b"/orgs/octocat/hooks/1/pings",
    b"/repos/octocat/Hello-World/contents/CONTRIBUTING",
    b"/repos/github/hello-world/actions/runs/5/attempts/3",
    b"/repos/octocat/Hello-World/tree/6dcb09b5b57875f334f61aebed695e2e4193db5e",
    b"/repos/octo-org/hello-world/tags",
    b"/orgs/my-org/rulesets/432",
    b"/orgs/octo-org/dependabot/secrets/NPM_TOKEN/repositories",
    b"/repos/octocat/Hello-World/git/commits/c3d0be41ecbe669545ee3e94d31ed9a4bc91ee3c",
    b"/repos/octocat/Hello-World/branches/main/protection",
    b"/repos/actions/setup-ruby/workflows/5",
    b"/user/octocat/packages/container/goodbye_docker",
    b"/emojis",
    b"/repos/octocat/Hello-World/branches/master/protection/restrictions",
    b"/repos/Octocoders/Hello-World/subscription",
    b"/repos/octocat/Hello-World/git/commits/7638417db6d59f3c431d3e1f261cc637155684cd",
    b"/repos/octocat/octorepo/git/blobs/fff6fe3a23bf1c8ea0692b4a883af99bee26fd3b",
    b"/repos/octocat/Hello-World/issues/1347/events",
    b"/repos/Codertocat/Hello-World/check-runs/128620228",
    b"/repos/octocat/Hello-World/subscription",
    b"/teams/2343027/discussions/1/comments",
    b"/user/keys/2",
    b"/repos/octocat/Hello-World/trees/fc6274d15fa3ae2ab983129fb037999f264ba9a7",
    b"/repos/octo-org/octo-repo/dependabot/alerts/2",
    b"/teams/2/repos",
    b"/repos/dtrupenn/Tetris/subscribers",
    b"/repos/octocat/Hello-World/git/7c258a9869f33c1e1e1f74fbb32f07c86cb5a75b",
    b"/repos/github/hello-world/check-runs/4",
    b"/users/mojombo/orgs",
    b"/repos/project/a-package/security-advisories/GHSA-abcd-1234-efgh",
    b"/repos/octocat/socm/import/authors/2268558",
    b"/repos/octocat/hello-world/branches/main/protection/restrictions/apps",
    b"/users/octocat/packages/container/hello_docker/versions/45763",
    b"/repos/octocat/octo-name-repo/deployments",
    b"/user/codespaces/monalisa-octocat-hello-world-g4wpq6h95q/start",
    b"/organizations/652551/personal-access-token-requests/25381/repositories",
    b"/repos/octo-org/octo-repo/languages",
    b"/repos/octocat/socm",
    b"/repositories/42/actions/permissions/selected-actions",
    b"/repos/octo-org/oct-repo",
    b"/repos/octocat/Hello-World-Template/events",
    b"/repos/octocat/Hello-World/issues/1347/comments",
    b"/repositories/42/git/tags/940bd336248efae0f9ee5bc7b2d5c985887b16ac",
    b"/orgs/github/packages/container/super-linter/versions/786068",
    b"/repos/Codertocat/Hello-World/subscribers",
    b"/repos/octo-org/hello-world/subscribers",
    b"/users/other_user/subscriptions",
    b"/users/github-product-roadmap/followers",
    b"/repos/octo-org/oct-repo/commits/7a8f3ac80e2ad2f6842cb86f576d4bfe2c03e300",
    b"/repos/Codertocat/Hello-World/downloads",
    b"/repos/Codertocat/Hello-World/check-suites/118578147",
    b"/orgs/octo-org/migrations/79",
    b"/repos/octocat/Spoon-Knife/commits/bb4cc8d3b2e14b3af5df699876dd4ff3acd00b7f/comments",
    b"/user/codespaces/monalisa-octocat-hello-world-f8adfad99a",
    b"/repos/Codertocat/Hello-World/pulls/2",
    b"/repos/octokit/octokit.rb/git/trees/a84d88e7554fc1fa21bcbc4efae3c782a70d2b9d",
    b"/repos/octocat/Hello-World-Template/tags",
    b"/orgs/octocat/hooks/1",
    b"/repos/github/hello-world/pages/deployments/4fd754f7e594640989b406850d0bc8f06a121251",
    b"/repos/octocat/Hello-World/hooks/12345678/test",
    b"/users/octocat/followers",
    b"/user/codespaces/monalisa-octocat-hello-world-f8adfad99a/start",
    b"/gists/aa5a315d61ae9438b18d",
    b"/users/octocat/packages/rubygems/octo-name/versions/387039",
    b"/notifications/threads/1/subscription",
    b"/repos/octocat/Spoon-Knife/teams",
    b"/users/rrubenich",
    b"/repos/octocat/hello-world/code-scanning/analyses/41",
    b"/repos/octo-org/hello-world/events",
    b"/users/testorg-ea8ec76d71c3af4b/orgs",
    b"/users/jquery/repos",
    b"/users/monauser/subscriptions",
    b"/repos/Octocoders/Hello-World/merges",
    b"/repos/api-playground/projects-test/issues/3",
    b"/repos/dtrupenn/Tetris/stargazers",
    b"/repos/the-org/an-org-repo/branches/master/protection/dismissal_restrictions/users",
    b"/organizations/1/team/2403582/discussions/1",
    b"/apps/another-custom-app",
    b"/user/codespaces/monalisa-octocat-hello-world-f8adfad99a/machines",
    b"/repos/jquery/jquery/merges",
    b"/organizations/1/team/2403582/discussions/1/comments/1",
    b"/teams/1/memberships/octocat",
    b"/repos/octocat/Hello-World/commits/7fd1a60b01f91b314f59955a4e4d4e80d8edf11d/comments",
    b"/users/octo-org/followers",
    b"/repos/octocat/Hello-World/issues/comments/1825855898",
    b"/repos/octocat/Hello-World/trees/cd8274d15fa3ae2ab983129fb037999f264ba9a7",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/stargazers",
    b"/repos/octocat/octorepo/contents/src",
    b"/teams/2403582/discussions/1/comments/1",
    b"/user/codespaces/monalisa-octocat-hello-world-3f89ada1j3",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/languages",
    b"/users/Octocoders/followers",
    b"/repos/octocat/hello-world/branches/main/protection/dismissal_restrictions/teams",
    b"/repos/github/hello-world/actions/workflows/main.yaml",
    b"/repos/octocat/hello-world/branches/main/protection/required_status_checks/contexts",
    b"/repos/octo-org/hello-world/dependabot/alerts/1",
    b"/repos/octocat/Hello-World-Template/hooks",
    b"/repos/octocat/example/deployments/42",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/contributors",
    b"/repos/github/hello-world/actions/artifacts/5",
    b"/users/Octocoders/subscriptions",
    b"/users/Octocoders/repos",
    b"/repos/owner/private-repo/secret-scanning/alerts/2/locations",
    b"/repositories/167174/git/blobs/d7212f9dee2dcc18f084d7df8f417b80846ded5a",
    b"/users/github-product-roadmap/orgs",
    b"/repos/octocat/Hello-World/branches/master/protection/required_signatures",
    b"/teams/2343027/discussions/1/reactions",
    b"/users/monauser/followers",
    b"/repos/octocat/Hello-World/commits/553c2077f0edc3d5dc5d17262f6aa498e69d6f8e",
    b"/gists/8481a81af6b7a2d418f2/468aac8caed5f0c3b859b8286968",
    b"/orgs/octo-org/packages/npm/hello-world-npm/versions/245301",
    b"/repos/octo-org/octo-repo/events",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/downloads",
    b"/repos/octo-org/octo-repo/actions/runs/30433642/cancel",
    b"/users/octocat/repos",
    b"/repos/octocat/example/deployments/42/statuses/1",
    b"/repos/octocat/Spoon-Knife/stargazers",
    b"/organizations/1/team/1",
    b"/repos/octocat/hello-world/branches/main/protection/restrictions",
    b"/orgs/octocat/hooks/1/deliveries",
    b"/users/other_user/received_events",
    b"/repos/octokit/octokit.rb/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
    b"/repos/octocat/Hello-World/commits/c5b97d5ae6c19d5c5df71a34c7fbeeda2479ccbc",
    b"/repos/the-org/an-org-repo/branches/master/protection/dismissal_restrictions",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/tags",
    b"/users/octocat",
    b"/orgs/octocat",
    b"/marketplace_listing/plans/1313",
    b"/repos/octocat/Hello-World/git/refs/heads/feature-a",
    b"/repos/octocat/octo-name-repo/forks",
    b"/repos/octocat/Hello-World/code-scanning/codeql/databases/java",
    b"/repos/octocat/Hello-World/contents/PULL_REQUEST_TEMPLATE",
    b"/repos/dtrupenn/Tetris/events",
    b"/repos/octocat/Hello-World/hooks/12345678",
    b"/orgs/github/packages/container/hello_docker",
    b"/repos/octokit/octokit.rb/contents/README.md",
    b"/repos/octocat/Hello-World/branches/master/protection/dismissal_restrictions",
    b"/repos/octokit/octokit.rb/git/blobs/452a98979c88e093d682cab404a3ec82babebb48",
    b"/users/testorg-ea8ec76d71c3af4b/subscriptions",
    b"/repositories/42/labels/bug",
    b"/repos/github/hello-world",
    b"/users/Nick3C/followers",
    b"/repos/octocat/Hello-World/labels/bug",
    b"/marketplace_listing/plans/1111",
    b"/licenses/lgpl-3.0",
    b"/repos/octocat/hello-world/branches/main/protection/enforce_admins",
    b"/repos/octocat/octo-name-repo/contributors",
    b"/teams/2343027/discussions/1",
    b"/user/codespaces/monalisa-octocat-hello-world-3f89ada1j3/machines",
    b"/projects/columns/cards/1478",
    b"/repos/octo-org/octo-docs/actions/artifacts/13/zip",
    b"/repos/github/hello-world/check-suites/12",
    b"/repos/octocat/Hello-World/hooks/1/deliveries",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/languages",
    b"/user/keys/3",
    b"/repos/octocat/Hello-World/issues/events/1",
    b"/repos/Octocoders/Hello-World/tags",
    b"/repos/dtrupenn/Tetris/deployments",
    b"/gists/aa5a315d61ae9438b18d/comments/",
    b"/users/other_user/orgs",
    b"/repos/octocat/Hello-World-Template/stargazers",
    b"/users/octocat/subscriptions",
    b"/repos/Octocoders/Hello-World/stargazers",
    b"/licenses/mpl-2.0",
    b"/user/codespaces/monalisa-octocat-hello-world-g4wpq6h95q/machines",
    b"/users/octo-org",
    b"/repos/octocat/Hello-World/git/trees/691272480426f78a0138979dd3ce63b77f706feb",
    b"/notifications/threads/2/subscription",
    b"/repos/Octocoders/Hello-World/teams",
    b"/users/dtrupenn/received_events",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/events",
    b"/repos/octocat/octorepo/contents/src/app.js",
    b"/projects/1002604/columns",
    b"/repos/octo-org/octo-repo/actions/workflows/161335",
    b"/repos/github/rest-api-description/git/blobs/abc",
    b"/repos/octocat/hello-world/code-scanning/alerts/4/instances",
    b"/repos/github/roadmap/issues/493",
    b"/repos/octocat/Hello-World/pull/2846",
    b"/repos/octocat/Hello-World/git/trees/9a21f8e2018f42ffcf369b24d2cd20bc25c9e66f",
    b"/apps/a-custom-app",
    b"/repos/jquery/jquery/languages",
    b"/orgs/invitocat/memberships/defunkt",
    b"/projects/1002604",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/contributors",
    b"/repos/octo-org/hello-world/hooks",
    b"/repos/octokit/octokit.rb/git/blobs/fff6fe3a23bf1c8ea0692b4a883af99bee26fd3b",
    b"/repos/octocat/Hello-World/merges",
    b"/orgs/invitocat",
    b"/repos/octocat-repo/hello-world/forks",
    b"/repos/jquery/jquery/tags",
    b"/repos/octocat/Hello-World/releases/assets/1",
    b"/repos/octocat/Hello-World/git/blobs/44b4fc6d56897b048c772eb4087f854f46256132",
    b"/users/monalisa/followers",
    b"/users/octocat/packages/rubygems/octo-name/versions/169770",
    b"/codes_of_conduct/contributor_covenant",
    b"/users/monauser/orgs",
    b"/apps/custom-app-slug",
    b"/marketplace_listing/plans/1111/accounts",
    b"/repos/octocat/Hello-World-Template/contributors",
    b"/repos/github/roadmap/issues/comments/1130876857",
    b"/repos/octocat/Hello-World/git/refs/heads/featureA",
    b"/repos/octocat/Hello-World/branches/master/protection/dismissal_restrictions/teams",
    b"/repos/octocat/linguist/labels/bug",
    b"/organizations/1/team/2343027",
    b"/users/octocat/packages/rubygems/octo-name/versions/3497268",
    b"/repos/octocat-repo/hello-world/contributors",
    b"/orgs/github",
    b"/repos/octocat/hello-world/dependabot/alerts/1",
    b"/repos/1",
    b"/repos/octo-org/octo-repo",
    b"/repos/octocat/octo-name-repo/events",
    b"/repos/github/hello-world/pages",
    b"/notifications/threads/1",
    b"/repos/octocat/hello-world/code-scanning/alerts/42",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/merges",
    b"/repos/octocat/Hello-World-Template",
    b"/repos/octocat/hello-world/branches/main/protection/required_status_checks",
    b"/orgs/octo-org/packages/npm/hello-world-npm/versions/209672",
    b"/users/jquery/received_events",
    b"/repos/octocat/Spoon-Knife/commits/a30c19e3f13765a3b48829788bc1cb8b4e95cee4",
    b"/repos/github/hello-world/environments/staging",
    b"/repos/octocat/hello-world/code-scanning/analyses/201",
    b"/repos/octocat/octorepo/contents/src/images",
    b"/repos/octocat-repo/hello-world/teams",
    b"/orgs/ORGANIZATION/codespaces/secrets/SECRET_NAME/repositories",
    b"/repos/octocat/Hello-World/git/blobs/a56507ed892d05a37c6d6128c260937ea4d287bd",
    b"/repos/octocat/Hello-World/branches/master/protection/required_pull_request_reviews",
    b"/repos/octocat/Hello-World/commits/7638417db6d59f3c431d3e1f261cc637155684cd",
    b"/events",
    b"/repos/octocat/Hello-World/pulls/2846",
    b"/repos/octocat/Hello-World/stargazers",
    b"/repos/octocat/hello-world/code-scanning/sarifs/47177e22-5596-11eb-80a1-c1e54ef945c6",
    b"/orgs/my-org/rulesets/21",
    b"/repos/octocat/Hello-World/branches/master/protection",
    b"/repos/octocat/Spoon-Knife/merges",
    b"/licenses/mit",
    b"/repos/octocat/Spoon-Knife/contributors",
    b"/repos/octocat/socm/import",
    b"/repos/Octocoders/Hello-World",
    b"/repos/Octocoders/Hello-World/hooks/109948940/pings",
    b"/repos/octocat/Hello-World/branches/master/protection/restrictions/teams",
    b"/repos/octocat/Spoon-Knife",
    b"/repos/github/docs/community/code_of_conduct",
    b"/repos/octocat/socm/import/authors/2268557",
    b"/repos/octokit/octokit.rb/contents/lib/octokit",
    b"/repos/dtrupenn/Tetris/downloads",
    b"/repos/Codertocat/Hello-World/tags",
    b"/repos/jquery/qunit/git/trees/6ca3721222109997540bd6d9ccd396902e0ad2f9",
    b"/repos/octo-org/hello-world/contributors",
    b"/repos/octocat/Hello-World/git/trees/b4eecafa9be2f2006b709d6857b07069b4608",
    b"/repos/octocat/Hello-World/git/trees/b4eecafa9be2f2006ce1b709d6857b07069b4608",
    b"/user/codespaces/monalisa-octocat-hello-world-g4wpq6h95q",
    b"/users/jquery/followers",
    b"/repos/Codertocat/Hello-World/events",
    b"/authorizations",
    b"/repos/Octocoders/Hello-World/subscribers",
    b"/users/other_user/followers",
    b"/repos/octocat/Hello-World-Template/subscribers",
    b"/repos/octocat/Hello-World/git/refs/heads/feature-b",
    b"/repos/octocat/Hello-World/6dcb09b5b57875f334f61aebed695e2e4193db5e",
    b"/repos/Codertocat/Hello-World/teams",
    b"/repos/octocat/Hello-World/commits/7a8f3ac80e2ad2f6842cb86f576d4bfe2c03e300",
    b"/repos/octocat/Hello-World/hooks/1",
    b"/repos/octocat/Hello-World/git/commits/da5a433788da5c255edad7979b328b67d79f53f6",
    b"/users/octokitten",
    b"/user/codespaces/name/exports/latest",
    b"/repos/octo-org/octo-repo/actions/runs/30433642/rerun",
    b"/repos/github/hello-world/actions/runs/5/logs",
    b"/users/github/received_events",
    b"/users/mojombo/followers",
    b"/repos/octocat/Hello-World/contents/README.md",
    b"/users/Codertocat/orgs",
    b"/repos/octo-org/octo-repo/deployments",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/stargazers",
    b"/user/keys",
    b"/repos/octocat/Hello-World/pulls/comments/12",
    b"/hub",
    b"/repos/octocat/Spoon-Knife/hooks",
    b"/repos/owner/private-repo/secret-scanning/alerts/42/locations",
    b"/repos/api-playground/projects-test",
    b"/repos/Codertocat/Hello-World/deployments",
    b"/repos/octocat/Spoon-Knife/downloads",
    b"/users/octo-org/subscriptions",
    b"/orgs/octocat/memberships/defunkt",
    b"/gists/2decf6c462d9b4418f2/commits",
    b"/repos/octocat/Hello-World/subscribers",
    b"/repos/octocat/linguist/labels/enhancement",
    b"/repos/octo-org/octo-repo/teams",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh/events",
    b"/organizations/652551/personal-access-tokens/25381/repositories",
    b"/users/github",
    b"/teams/2343027",
    b"/orgs/github/repos",
    b"/repos/dtrupenn/Tetris/hooks",
    b"/repos/octocat/Hello-World/6dcb09b5b57875f334f61aebed695e2e4193db5e/status",
    b"/repos/octocat/Spoon-Knife/subscription",
    b"/licenses/agpl-3.0",
    b"/repos/octocat/Hello-World/tags",
    b"/repos/octo-org/octo-repo/actions/runs/30433642/logs",
    b"/repos/octocat/hello-world/branches/main/protection",
    b"/users/mojombo",
    b"/repos/octocat/Hello-World/milestones/1/labels",
    b"/repos/octocat/Hello-World/languages",
    b"/repos/dtrupenn/Tetris/subscription",
    b"/repos/owner-79e94e2d36b3fd06a32bb213/AAA_Public_Repo/branches/branch/with/protection/protection",
    b"/teams/2403582/discussions/1",
    b"/repos/Octocoders/Hello-World/hooks/109948940/test",
    b"/repos/octocat/Hello-World/git/commits/18a43cd8e1e3a79c786e3d808a73d23b6d212b16",
    b"/repos/octocat/octo-name-repo/tags",
    b"/repos/github/hello-world/actions/artifacts/5/zip",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/hooks",
    b"/repos/Octocoders/Hello-World/hooks/109948940",
    b"/repos/octo-org/octo-repo/actions/runs/30433642/artifacts",
    b"/users/octocat/received_events",
    b"/repos/owner/private-repo/secret-scanning/alerts/2",
    b"/repos/octo-org/octo-repo-ghsa-1234-5678-9012/subscription",
    b"/users/octokitten/orgs",
    b"/licenses/gpl-3.0",
    b"/users/octokitten/followers",
    b"/repos/monalisa/monalisa/code-scanning/alerts/2",
    b"/repos/octo-org/octo-repo-ghsa-abcd-1234-efgh",
    b"/repos/octoorg/octocat/actions/runs/42",
    b"/repos/Codertocat/Hello-World/subscription",
    b"/user/codespaces/monalisa-octocat-hello-world-g4wpq6h95q/stop",
    b"/users/Nick3C",
    b"/repos/Codertocat/Hello-World/merges",
    b"/repos/github/hello-world/actions/runs/5/rerun",
    b"/projects/120",
    b"/repositories/42/issues/comments/1",
    b"/advisories/GHSA-abcd-1234-efgh",
    b"/users/jquery/orgs",
    b"/repos/octo-org/octo-repo/downloads",
    b"/orgs/octo-org/actions/secrets/SUPER_SECRET/repositories",
    b"/issues",
    b"/repos/octocat/Spoon-Knife/git/commits/bb4cc8d3b2e14b3af5df699876dd4ff3acd00b7f",
    b"/repos/octocat/Hello-World/issues/comments/1081119451",
    b"/repos/octocat/Spoon-Knife/subscribers",
    b"/repos/octocat/octo-name-repo/languages",
    b"/users/jquery/subscriptions",
    b"/users/Nick3C/received_events",
    b"/repos/github/hello-world/pulls/1",
    b"/repos/octo-org/octo-repo/subscription",
    b"/repos/octo-org/hello-world/forks",
    b"/repos/octocat-repo/hello-world/git/commits/f14d7debf9775f957cf4f1e8176da0786431f72b",
];

#[allow(dead_code)]
const PLAIN_PATH_512: &[u8] = PLAIN_URLS_512[PLAIN_URLS_512.len() - 1];