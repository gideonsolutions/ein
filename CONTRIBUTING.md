# Contributing

Thank you for your interest in contributing to this project!

## Semantic Releases

This project uses [Release Please](https://github.com/googleapis/release-please) to automate releases based on [Conventional Commits](https://www.conventionalcommits.org/).

### How It Works

1. When PRs are merged to `main`, Release Please analyzes the commit messages
2. It automatically creates/updates a release PR with version bumps and changelog entries
3. When the release PR is merged, a GitHub release is created and the package is published to crates.io

### Commit Message Format

Your commit messages determine the version bump:

| Commit Type | Description | Version Bump |
|-------------|-------------|--------------|
| `fix:` | Bug fixes | Patch (0.0.X) |
| `feat:` | New features | Minor (0.X.0) |
| `feat!:` or `fix!:` | Breaking changes | Major (X.0.0) |

You can also indicate a breaking change by adding a `BREAKING CHANGE:` footer in the commit body:

```
feat: change EIN parsing rules

BREAKING CHANGE: existing callers may see different validation behavior
```

Other common types (no version bump):
- `docs:` - Documentation changes
- `chore:` - Maintenance tasks
- `ci:` - CI/CD changes
- `refactor:` - Code refactoring
- `test:` - Test changes
- `perf:` - Performance improvements

### Examples

```
fix: handle edge case in EIN prefix validation
feat: add campus code lookup
feat!: rename ParseError variants
docs: improve API documentation
```

### Pull Request Guidelines

1. Use conventional commit format for your PR title (it becomes the merge commit message)
2. Keep commits focused and atomic
3. Ensure all CI checks pass before requesting review
