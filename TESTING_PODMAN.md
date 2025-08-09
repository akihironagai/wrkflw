# Testing Podman Support in WRKFLW

This document provides comprehensive testing steps to verify that Podman support is working correctly in wrkflw.

## Prerequisites

### 1. Install Podman

Choose the installation method for your operating system:

#### macOS (using Homebrew)
```bash
brew install podman
```

#### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install podman
```

#### RHEL/CentOS/Fedora
```bash
# Fedora
sudo dnf install podman

# RHEL/CentOS 8+
sudo dnf install podman
```

#### Windows
```bash
# Using Chocolatey
choco install podman-desktop

# Or download from https://podman.io/getting-started/installation
```

### 2. Initialize Podman (macOS/Windows only)
```bash
podman machine init
podman machine start
```

### 3. Verify Podman Installation
```bash
podman version
podman info
```

Expected output should show Podman version and system information without errors.

### 4. Build WRKFLW with Podman Support
```bash
cd /path/to/wrkflw
cargo build --release
```

## Test Plan

### Test 1: CLI Runtime Selection

#### 1.1 Test Default Runtime (Docker)
```bash
# Should default to Docker
./target/release/wrkflw run --help | grep -A 5 "runtime"
```
Expected: Should show `--runtime` option with default value `docker`.

#### 1.2 Test Podman Runtime Selection
```bash
# Should accept podman as runtime
./target/release/wrkflw run --runtime podman test-workflows/example.yml
```
Expected: Should run without CLI argument errors.

#### 1.3 Test Emulation Runtime Selection
```bash
# Should accept emulation as runtime
./target/release/wrkflw run --runtime emulation test-workflows/example.yml
```
Expected: Should run without CLI argument errors.

#### 1.4 Test Invalid Runtime Selection
```bash
# Should reject invalid runtime
./target/release/wrkflw run --runtime invalid test-workflows/example.yml
```
Expected: Should show error about invalid runtime choice.

### Test 2: Podman Availability Detection

#### 2.1 Test with Podman Available
```bash
# Ensure Podman is running
podman info > /dev/null && echo "Podman is available"

# Test wrkflw detection
./target/release/wrkflw run --runtime podman --verbose test-workflows/example.yml
```
Expected: Should show "Podman is available, using Podman runtime" in logs.

#### 2.2 Test with Podman Unavailable
```bash
# Temporarily make podman unavailable
sudo mv /usr/local/bin/podman /usr/local/bin/podman.bak 2>/dev/null || echo "Podman not in /usr/local/bin"

# Test fallback to emulation
./target/release/wrkflw run --runtime podman --verbose test-workflows/example.yml

# Restore podman
sudo mv /usr/local/bin/podman.bak /usr/local/bin/podman 2>/dev/null || echo "Nothing to restore"
```
Expected: Should show "Podman is not available. Using emulation mode instead."

### Test 3: Container Execution with Podman

#### 3.1 Create a Simple Test Workflow
Create `test-podman-workflow.yml`:

```yaml
name: Test Podman Workflow
on: [workflow_dispatch]

jobs:
  test-podman:
    runs-on: ubuntu-latest
    container: ubuntu:20.04
    steps:
      - name: Test basic commands
        run: |
          echo "Testing Podman container execution"
          whoami
          pwd
          ls -la
          echo "Container test completed successfully"
      
      - name: Test environment variables
        env:
          TEST_VAR: "podman-test"
        run: |
          echo "Testing environment variables"
          echo "TEST_VAR: $TEST_VAR"
          echo "GITHUB_WORKSPACE: $GITHUB_WORKSPACE"
          echo "RUNNER_OS: $RUNNER_OS"
      
      - name: Test volume mounting
        run: |
          echo "Testing volume mounting"
          echo "test-file-content" > test-file.txt
          cat test-file.txt
          ls -la test-file.txt
```

#### 3.2 Test Podman Container Execution
```bash
./target/release/wrkflw run --runtime podman --verbose test-podman-workflow.yml
```
Expected: Should execute all steps successfully using Podman containers.

#### 3.3 Compare with Docker Execution
```bash
# Test same workflow with Docker
./target/release/wrkflw run --runtime docker --verbose test-podman-workflow.yml

# Test same workflow with emulation  
./target/release/wrkflw run --runtime emulation --verbose test-podman-workflow.yml
```
Expected: All three runtimes should produce similar results (emulation may have limitations).

### Test 4: TUI Interface Testing

#### 4.1 Test TUI Runtime Selection
```bash
./target/release/wrkflw tui test-workflows/
```

**Test Steps:**
1. Launch TUI
2. Press `e` key to cycle through runtimes
3. Verify status bar shows: Docker → Podman → Emulation → Docker
4. Check that Podman status shows "Connected" or "Not Available"
5. Select a workflow and run it with Podman runtime

#### 4.2 Test TUI with Specific Runtime
```bash
# Start TUI with Podman runtime
./target/release/wrkflw tui --runtime podman test-workflows/

# Start TUI with emulation runtime
./target/release/wrkflw tui --runtime emulation test-workflows/
```
Expected: TUI should start with the specified runtime active.

### Test 5: Container Preservation Testing

✅ **Note**: Container preservation is fully supported with Podman and works correctly.

#### 5.1 Test Container Cleanup (Default)
```bash
# Run a workflow that will fail
echo 'name: Failing Test
on: [workflow_dispatch]
jobs:
  fail:
    runs-on: ubuntu-latest
    container: ubuntu:20.04
    steps:
      - run: exit 1' > test-fail-workflow.yml

./target/release/wrkflw run --runtime podman test-fail-workflow.yml

# Check if containers were cleaned up
podman ps -a --filter "name=wrkflw-"
```
Expected: No wrkflw containers should remain.

#### 5.2 Test Container Preservation on Failure
```bash
./target/release/wrkflw run --runtime podman --preserve-containers-on-failure test-fail-workflow.yml

# Check if failed container was preserved
podman ps -a --filter "name=wrkflw-"
```
Expected: Should show preserved container. Note the container ID for inspection.

#### 5.3 Test Container Inspection
```bash
# Get container ID from previous step
CONTAINER_ID=$(podman ps -a --filter "name=wrkflw-" --format "{{.ID}}" | head -1)

# Inspect the preserved container
podman exec -it $CONTAINER_ID bash
# Inside container: explore the environment, check files, etc.
# Exit with: exit

# Clean up manually
podman rm $CONTAINER_ID
```

### Test 6: Image Operations Testing

#### 6.1 Test Image Pulling
```bash
# Create workflow that uses a specific image
echo 'name: Image Pull Test
on: [workflow_dispatch]
jobs:
  test:
    runs-on: ubuntu-latest
    container: node:18-alpine
    steps:
      - run: node --version' > test-image-pull.yml

./target/release/wrkflw run --runtime podman --verbose test-image-pull.yml
```
Expected: Should pull node:18-alpine image and execute successfully.

#### 6.2 Test Custom Image Building
```bash
# Create a workflow that builds a custom image (if supported)
# This tests the build_image functionality
mkdir -p test-build
echo 'FROM ubuntu:20.04
RUN apt-get update && apt-get install -y curl
CMD ["echo", "Custom image test"]' > test-build/Dockerfile

echo 'name: Image Build Test
on: [workflow_dispatch]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Build and test custom image
        run: |
          echo "Testing custom image scenarios"
          curl --version' > test-custom-image.yml

# Note: This test depends on language environment preparation
./target/release/wrkflw run --runtime podman --verbose test-custom-image.yml
```

### Test 7: Error Handling and Edge Cases

#### 7.1 Test Invalid Container Image
```bash
echo 'name: Invalid Image Test
on: [workflow_dispatch]
jobs:
  test:
    runs-on: ubuntu-latest
    container: nonexistent-image:invalid-tag
    steps:
      - run: echo "This should fail"' > test-invalid-image.yml

./target/release/wrkflw run --runtime podman test-invalid-image.yml
```
Expected: Should handle image pull failure gracefully with clear error message.

#### 7.2 Test Network Connectivity
```bash
echo 'name: Network Test
on: [workflow_dispatch]
jobs:
  test:
    runs-on: ubuntu-latest
    container: ubuntu:20.04
    steps:
      - name: Test network access
        run: |
          apt-get update
          apt-get install -y curl
          curl -s https://httpbin.org/get
      - name: Test DNS resolution
        run: nslookup google.com' > test-network.yml

./target/release/wrkflw run --runtime podman --verbose test-network.yml
```
Expected: Should have network access and complete successfully.

#### 7.3 Test Resource Intensive Workflow
```bash
echo 'name: Resource Test
on: [workflow_dispatch]
jobs:
  test:
    runs-on: ubuntu-latest
    container: ubuntu:20.04
    steps:
      - name: Memory test
        run: |
          echo "Testing memory usage"
          free -h
          dd if=/dev/zero of=/tmp/test bs=1M count=100
          ls -lh /tmp/test
          rm /tmp/test
      - name: CPU test
        run: |
          echo "Testing CPU usage"
          yes > /dev/null &
          PID=$!
          sleep 2
          kill $PID
          echo "CPU test completed"' > test-resources.yml

./target/release/wrkflw run --runtime podman --verbose test-resources.yml
```

### Test 8: Comparison Testing

#### 8.1 Create Comprehensive Test Workflow
```bash
echo 'name: Comprehensive Runtime Comparison
on: [workflow_dispatch]

env:
  GLOBAL_VAR: "global-value"

jobs:
  test-all-features:
    runs-on: ubuntu-latest
    container: ubuntu:20.04
    env:
      JOB_VAR: "job-value"
    steps:
      - name: Environment test
        env:
          STEP_VAR: "step-value"
        run: |
          echo "=== Environment Variables ==="
          echo "GLOBAL_VAR: $GLOBAL_VAR"
          echo "JOB_VAR: $JOB_VAR"
          echo "STEP_VAR: $STEP_VAR"
          echo "GITHUB_WORKSPACE: $GITHUB_WORKSPACE"
          echo "GITHUB_REPOSITORY: $GITHUB_REPOSITORY"
          echo "RUNNER_OS: $RUNNER_OS"
          
      - name: File system test
        run: |
          echo "=== File System Test ==="
          pwd
          ls -la
          whoami
          id
          df -h
          
      - name: Network test
        run: |
          echo "=== Network Test ==="
          apt-get update -q
          apt-get install -y curl iputils-ping
          ping -c 3 8.8.8.8
          curl -s https://httpbin.org/ip
          
      - name: Process test
        run: |
          echo "=== Process Test ==="
          ps aux
          top -b -n 1 | head -10
          
      - name: Package installation test
        run: |
          echo "=== Package Test ==="
          apt-get install -y python3 python3-pip
          python3 --version
          pip3 --version' > comprehensive-test.yml
```

#### 8.2 Run Comprehensive Test with All Runtimes
```bash
echo "Testing with Docker:"
./target/release/wrkflw run --runtime docker --verbose comprehensive-test.yml > docker-test.log 2>&1

echo "Testing with Podman:"
./target/release/wrkflw run --runtime podman --verbose comprehensive-test.yml > podman-test.log 2>&1

echo "Testing with Emulation:"
./target/release/wrkflw run --runtime emulation --verbose comprehensive-test.yml > emulation-test.log 2>&1

# Compare results
echo "=== Comparing Results ==="
echo "Docker exit code: $?"
echo "Podman exit code: $?"
echo "Emulation exit code: $?"

# Optional: Compare log outputs
diff docker-test.log podman-test.log | head -20
```

## Expected Results Summary

### ✅ **Should Work:**
- CLI accepts `--runtime podman` without errors
- TUI cycles through Docker → Podman → Emulation with 'e' key
- Status bar shows Podman availability correctly
- Container execution works identically to Docker
- Container cleanup respects preservation settings
- Image pulling and basic image operations work
- Environment variables are passed correctly
- Volume mounting works for workspace access
- Network connectivity is available in containers
- Error handling is graceful and informative

### ⚠️ **Limitations to Expect:**
- Some advanced Docker-specific features may not work identically
- Performance characteristics may differ from Docker
- Podman-specific configuration might be needed for complex scenarios
- Error messages may differ between Docker and Podman

### 🚨 **Should Fail Gracefully:**
- Invalid runtime selection should show clear error
- Missing Podman should fall back to emulation with warning
- Invalid container images should show helpful error messages
- Network issues should be reported clearly

## Cleanup

After testing, clean up test files:
```bash
rm -f test-podman-workflow.yml test-fail-workflow.yml test-image-pull.yml
rm -f test-custom-image.yml test-invalid-image.yml test-network.yml
rm -f test-resources.yml comprehensive-test.yml
rm -f docker-test.log podman-test.log emulation-test.log
rm -rf test-build/
podman system prune -f  # Clean up unused containers and images
```

## Troubleshooting

### Common Issues:

1. **"Podman not available"**
   - Verify Podman installation: `podman version`
   - Check Podman service: `podman machine list` (macOS/Windows)

2. **Permission errors**
   - Podman should work rootless by default
   - Check user namespaces: `podman unshare cat /proc/self/uid_map`

3. **Network issues**
   - Test basic connectivity: `podman run --rm ubuntu:20.04 ping -c 1 8.8.8.8`

4. **Container startup failures**
   - Check Podman logs: `podman logs <container-id>`
   - Verify image availability: `podman images`

This comprehensive testing plan should verify that Podman support is working correctly and help identify any issues that need to be addressed.
