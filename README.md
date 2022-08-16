# Git Issue

Git command to manage branch names and commit messages


# Compilation

Requires rust🦀:

`cargo build --release`


# Configuration

Git Issue uses the following configuration parameters, all set with `git 
config`, either globally or per project.

| Name                | Description                                                                                 |
|=====================|=============================================================================================|
| issue.personality   | Either `github` or `jira`                                                                   |
| issue.defaultbranch | (Optional) The branch to switch to on clear, if `--to`  is not present. Default is `master` |
| issue.github.repo   | (Mandatory for github) Repository for github                                                |
| issue.github.user   | (Mandatory for github is repo is not public) Username for github                            |
| issue.github.public | (Optional) If true the repo is public and requires no auth                                  |
| issue.jira.host     | (Mandatory for jira) Jira base url                                                          |
| issue.jira.user     | (Mandatory for jira) Username for jira                                                      |

