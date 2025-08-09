# Testing Strategy

This directory contains all tests and test-related files for the `wrkflw` project. We follow the Rust testing best practices by organizing tests as follows:

## Test Organization

- **Unit Tests**: Located alongside the source files in `src/` using `#[cfg(test)]` modules
- **Integration Tests**: Located directly in this `tests/` directory
  - `matrix_test.rs` - Tests for matrix expansion functionality
  - `reusable_workflow_test.rs` - Tests for reusable workflow validation
- **End-to-End Tests**: Also located in this `tests/` directory
  - `cleanup_test.rs` - Tests for cleanup functionality with Docker resources

## Test Directory Structure

- **`fixtures/`**: Test data and configuration files
  - `gitlab-ci/` - GitLab CI configuration files for testing
- **`workflows/`**: GitHub Actions workflow files for testing
  - Various YAML files for testing workflow validation and execution
- **`scripts/`**: Test automation scripts
  - `test-podman-basic.sh` - Basic Podman integration test script
  - `test-preserve-containers.sh` - Container preservation testing script
- **`TESTING_PODMAN.md`**: Comprehensive Podman testing documentation

## Running Tests

To run all tests:
```bash
cargo test
```

To run only unit tests:
```bash
cargo test --lib
```

To run only integration tests:
```bash
cargo test --test matrix_test --test reusable_workflow_test
```

To run only end-to-end tests:
```bash
cargo test --test cleanup_test
```

To run a specific test:
```bash
cargo test test_name
```

## Writing Tests

Please follow these guidelines when writing tests:

1. Use meaningful test names that describe what is being tested
2. Group related tests together in modules
3. Use helper functions to reduce duplication
4. Test both success and failure cases
5. Use `#[should_panic]` for tests that expect a panic
6. Avoid test interdependencies 