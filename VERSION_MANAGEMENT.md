# Version Management Guide

This guide explains how to manage versions in the wrkflw workspace, both for the entire workspace and for individual crates.

## Overview

The wrkflw project uses a Cargo workspace with flexible version management that supports:
- **Workspace-wide versioning**: All crates share the same version
- **Individual crate versioning**: Specific crates can have their own versions
- **Automatic dependency management**: Internal dependencies are managed through workspace inheritance

## Current Setup

### Workspace Dependencies
All internal crate dependencies are defined in the root `Cargo.toml` under `[workspace.dependencies]`:

```toml
[workspace.dependencies]
# Internal crate dependencies
wrkflw-models = { path = "crates/models", version = "0.7.2" }
wrkflw-evaluator = { path = "crates/evaluator", version = "0.7.2" }
# ... other crates
```

### Crate Dependencies
Individual crates reference internal dependencies using workspace inheritance:

```toml
[dependencies]
# Internal crates
wrkflw-models.workspace = true
wrkflw-validators.workspace = true
```

This approach means:
- ✅ No hard-coded versions in individual crates
- ✅ Single source of truth for internal crate versions
- ✅ Easy individual crate versioning without manual updates everywhere

## Version Management Strategies

### Strategy 1: Workspace-Wide Versioning (Recommended for most cases)

Use this when changes affect multiple crates or for major releases.

```bash
# Bump all crates to the same version
cargo ws version patch  # 0.7.2 → 0.7.3
cargo ws version minor  # 0.7.2 → 0.8.0
cargo ws version major  # 0.7.2 → 1.0.0

# Or specify exact version
cargo ws version 1.0.0

# Commit and tag
git add .
git commit -m "chore: bump workspace version to $(grep '^version' Cargo.toml | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/')"
git tag v$(grep '^version' Cargo.toml | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/')
git push origin main --tags
```

### Strategy 2: Individual Crate Versioning

Use this when changes are isolated to specific crates.

#### Using the Helper Script

```bash
# Bump a specific crate
./scripts/bump-crate.sh wrkflw-models patch    # 0.7.2 → 0.7.3
./scripts/bump-crate.sh wrkflw-models minor    # 0.7.2 → 0.8.0
./scripts/bump-crate.sh wrkflw-models 0.8.5    # Specific version

# The script will:
# 1. Update the crate's Cargo.toml to use explicit version
# 2. Update workspace dependencies
# 3. Show you next steps
```

#### Manual Individual Versioning

If you prefer manual control:

1. **Update the crate's Cargo.toml**:
   ```toml
   # Change from:
   version.workspace = true
   # To:
   version = "0.7.3"
   ```

2. **Update workspace dependencies**:
   ```toml
   [workspace.dependencies]
   wrkflw-models = { path = "crates/models", version = "0.7.3" }
   ```

3. **Test and commit**:
   ```bash
   cargo check
   git add .
   git commit -m "bump: wrkflw-models to 0.7.3"
   git tag v0.7.3-wrkflw-models
   git push origin main --tags
   ```

## Release Workflows

### Full Workspace Release

```bash
# 1. Make your changes
# 2. Bump version
cargo ws version patch --no-git-commit

# 3. Commit and tag
git add .
git commit -m "chore: release version $(grep '^version' Cargo.toml | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/')"
git tag v$(grep '^version' Cargo.toml | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/')

# 4. Push (this triggers GitHub Actions)
git push origin main --tags
```

### Individual Crate Release

```bash
# 1. Use helper script or manual method above
./scripts/bump-crate.sh wrkflw-models patch

# 2. Follow the script's suggestions
git add .
git commit -m "bump: wrkflw-models to X.Y.Z"
git tag vX.Y.Z-wrkflw-models
git push origin main --tags

# 3. Optionally publish to crates.io
cd crates/models
cargo publish
```

## Publishing to crates.io

### Publishing Individual Crates

```bash
# Navigate to the crate
cd crates/models

# Ensure all dependencies are published first
# (or available on crates.io)
cargo publish --dry-run

# Publish
cargo publish
```

### Publishing All Crates

```bash
# Use cargo-workspaces
cargo ws publish --from-git
```

## Integration with GitHub Actions

The existing `.github/workflows/release.yml` works with both strategies:

- **Tag format `v1.2.3`**: Triggers full workspace release
- **Tag format `v1.2.3-crate-name`**: Could be used for individual crate releases (needs workflow modification)

### Modifying for Individual Crate Releases

To support individual crate releases, you could modify the workflow to:

```yaml
on:
  push:
    tags:
      - 'v*'           # Full releases: v1.2.3
      - 'v*-wrkflw-*'  # Individual releases: v1.2.3-wrkflw-models
```

## Best Practices

### When to Use Each Strategy

**Use Workspace-Wide Versioning when:**
- Making breaking changes across multiple crates
- Major feature releases
- Initial development phases
- Simpler release management is preferred

**Use Individual Crate Versioning when:**
- Changes are isolated to specific functionality
- Different crates have different stability levels
- You want to minimize dependency updates for users
- Publishing to crates.io with different release cadences

### Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **Patch (0.7.2 → 0.7.3)**: Bug fixes, internal improvements
- **Minor (0.7.2 → 0.8.0)**: New features, backward compatible
- **Major (0.7.2 → 1.0.0)**: Breaking changes

### Dependency Management

- Keep internal dependencies using workspace inheritance
- Only specify explicit versions when a crate diverges from workspace version
- Always test with `cargo check` and `cargo test` before releasing
- Use `cargo tree` to verify dependency resolution

## Troubleshooting

### Common Issues

**Issue**: Cargo complains about version mismatches
```bash
# Solution: Check workspace dependencies match crate versions
grep -r "version.*=" crates/*/Cargo.toml
grep "wrkflw-.*version" Cargo.toml
```

**Issue**: Published crate can't find dependencies
```bash
# Solution: Ensure all dependencies are published to crates.io first
# Or use path dependencies only for local development
```

**Issue**: GitHub Actions fails on tag
```bash
# Solution: Ensure tag format matches workflow trigger
git tag -d v1.2.3  # Delete local tag
git push origin :refs/tags/v1.2.3  # Delete remote tag
git tag v1.2.3  # Recreate with correct format
git push origin v1.2.3
```

## Tools and Commands

### Useful Commands

```bash
# List all workspace members with versions
cargo ws list

# Check all crates
cargo check --workspace

# Test all crates
cargo test --workspace

# Show dependency tree
cargo tree

# Show outdated dependencies
cargo outdated

# Verify publishability
cargo publish --dry-run --manifest-path crates/models/Cargo.toml
```

### Recommended Tools

- `cargo-workspaces`: Workspace management
- `cargo-outdated`: Check for outdated dependencies  
- `cargo-audit`: Security audit
- `cargo-machete`: Find unused dependencies

## Migration Notes

If you're migrating from the old hard-coded version system:

1. All internal crate versions are now managed in workspace `Cargo.toml`
2. Individual crates use `crate-name.workspace = true` for internal dependencies
3. Use the helper script or manual process above for individual versioning
4. The system is fully backward compatible with existing workflows