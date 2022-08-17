![ci status](https://github.com/move-z/git-issue/actions/workflows/ci.yml/badge.svg?branch=master)

# Git Issue

Git command to set commit message templates and branch names with info from 
issue tracking systems. So far only GitHub and local Jira installations are 
supported.

## Installation

Grab a binary and put it in your path, for Linux and MacOS rename it to 
`git-issue`.

## Configuration

Git Issue requires some config options, either global or per project.

Sample config:

```sh
git config --global issue.github.user xxx
git config --global issue.jira.host https://jira.mycompany.com
git config --global issue.jira.user xxx
# sample github project configuration
git config issue.personality github
git config issue.github.repo r
# if the repo is public don't require auth
git config issue.github.public true
```

## Use

Set the commit template to **<issue id> - <issue title>** for jira issue 
JIR-44:

```sh
git issue JIR-44
```

Set the commit template *and* switch to branch **<issue-id>-<issue-title>** 
for github issue 234:

```sh
git issue -b 234
```

Reset the commit template and eventually switch back to master:

```sh
git issue -c
```

## Building from source

Requires rust🦀:

`cargo build --release`


## Appendix: Table of configuration options

Git Issue uses the following configuration parameters, all set with `git 
config`, either globally or per project.

| Name                | Description                                                                                 |
|---------------------|---------------------------------------------------------------------------------------------|
| issue.personality   | Either `github` or `jira`                                                                   |
| issue.defaultbranch | (Optional) The branch to switch to on clear, if `--to`  is not present. Default is `master` |
| issue.github.repo   | (Mandatory for github) Repository for github                                                |
| issue.github.user   | (Mandatory for github is repo is not public) Username for github                            |
| issue.github.public | (Optional) If true the repo is public and requires no auth                                  |
| issue.jira.host     | (Mandatory for jira) Jira base url                                                          |
| issue.jira.user     | (Mandatory for jira) Username for jira                                                      |

