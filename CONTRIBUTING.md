# Contributing

Thank you for contributing to vesting-cliff-drip-stream!

## Branch Protection on `main`

The `main` branch is protected with the following rules:

| Rule | Setting |
|------|---------|
| Require pull request | ✅ — all changes must go through a PR |
| Required approving reviews | 1 |
| Dismiss stale reviews on new push | ✅ |
| Require CI to pass before merge | ✅ (`test`, `build` checks) |
| Require branch to be up to date | ✅ |
| Allow force push | ❌ |
| Allow branch deletion | ❌ |

### Applying the rules

To (re-)apply the protection rules after a fresh repo clone or rule reset:

```bash
export GITHUB_TOKEN=<your-pat-with-repo-scope>
export REPO=AlienScroll78/vesting-cliff-drip-stream
bash scripts/apply_branch_protection.sh
```

Requires a PAT with **repo** scope or a GitHub App installation token with `administration: write`.

## Admin Override

Admins can merge without a review in exceptional circumstances (incident hotfix, CI outage):

1. The protection rule sets `enforce_admins: false`, so repository admins bypass review requirements.
2. **Document the reason** in the PR description using the `## Emergency Merge` section.
3. Follow up within 24 hours with a normal PR that adds or confirms tests for the change.
4. Post a note in the `#eng-oncall` Slack channel linking the PR.

## Workflow

1. Fork or create a feature branch: `git checkout -b feat/<short-description>`
2. Make changes, add tests, and ensure `make test` passes locally.
3. Open a pull request against `main`.
4. Address review feedback; CI must be green before merging.
5. Squash-merge (preferred) or merge commit — no force pushes.

## Commit Message Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add multi-token support
fix: clamp claimable amount at stream end
docs: update restore runbook
chore: bump soroban-sdk to 22.0
```

Breaking changes: append `!` or add `BREAKING CHANGE:` in the footer.
