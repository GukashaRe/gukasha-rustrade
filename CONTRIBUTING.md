# Contributing to gukasha-rustrade

Thanks for your interest in contributing!

## Branch Strategy

- `main` – Stable branch, always in a releasable state
- `beta` – Development branch, submit all PRs here
- `hotfix-*` – Emergency fixes

## Development Workflow

1. **Fork the repo** (if you don't have direct write access)

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/gukasha-rustrade.git
   cd gukasha-rustrade
   ```

3. **Create a feature branch from `beta`**
   ```bash
   git checkout beta
   git pull origin beta
   git checkout -b feature/your-feature-name
   ```

4. **Make your changes and commit**
   ```bash
   git add .
   git commit -m "feat: add your feature"
   git push origin feature/your-feature-name
   ```

5. **Open a Pull Request** to the `beta` branch on GitHub

## Commit Convention

| Prefix | Usage |
|--------|-------|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `docs:` | Documentation changes |
| `refactor:` | Code refactoring |
| `test:` | Test related |
| `chore:` | Build / tooling |

## Local Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run the project
cargo run
```

## Merge Rules

- PR requires at least 1 approval (for team repos)
- CI checks must pass
- After merging to `beta`, changes are periodically synced to `main` for releases

## Reporting Issues

Please include:
- Environment: `rustc --version`
- Steps to reproduce
- Expected vs actual behavior
- Logs or screenshots (if applicable)

---

Thanks for contributing! 🦀