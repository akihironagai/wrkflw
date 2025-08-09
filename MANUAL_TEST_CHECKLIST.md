# Manual Testing Checklist for Podman Support

## Quick Manual Verification Steps

### ✅ **Step 1: CLI Help and Options**
```bash
./target/release/wrkflw run --help
```
**Verify:**
- [ ] `--runtime` option is present
- [ ] Shows `docker`, `podman`, `emulation` as possible values
- [ ] Default is `docker`
- [ ] Help text explains each option

### ✅ **Step 2: CLI Runtime Selection**
```bash
# Test each runtime option
./target/release/wrkflw run --runtime docker test-workflows/example.yml --verbose
./target/release/wrkflw run --runtime podman test-workflows/example.yml --verbose  
./target/release/wrkflw run --runtime emulation test-workflows/example.yml --verbose

# Test invalid runtime (should fail)
./target/release/wrkflw run --runtime invalid test-workflows/example.yml
```
**Verify:**
- [ ] All valid runtimes are accepted
- [ ] Invalid runtime shows clear error message
- [ ] Podman mode shows "Podman: Running container" in verbose logs
- [ ] Emulation mode works without containers

### ✅ **Step 3: TUI Runtime Support**
```bash
./target/release/wrkflw tui test-workflows/
```
**Verify:**
- [ ] TUI starts successfully
- [ ] Status bar shows current runtime (bottom of screen)
- [ ] Press `e` key to cycle through runtimes: Docker → Podman → Emulation → Docker
- [ ] Runtime changes are reflected in status bar
- [ ] Podman shows "Connected" or "Not Available" status

### ✅ **Step 4: TUI Runtime Parameter**
```bash
./target/release/wrkflw tui --runtime podman test-workflows/
./target/release/wrkflw tui --runtime emulation test-workflows/
```
**Verify:**
- [ ] TUI starts with specified runtime
- [ ] Status bar reflects the specified runtime

### ✅ **Step 5: Container Execution Test**
Create a simple test workflow:
```yaml
name: Runtime Test
on: [workflow_dispatch]
jobs:
  test:
    runs-on: ubuntu-latest
    container: ubuntu:20.04
    steps:
      - run: |
          echo "Runtime test execution"
          whoami
          pwd
          echo "Test completed"
```

Test with different runtimes:
```bash
./target/release/wrkflw run --runtime podman test-runtime.yml --verbose
./target/release/wrkflw run --runtime docker test-runtime.yml --verbose
./target/release/wrkflw run --runtime emulation test-runtime.yml --verbose
```
**Verify:**
- [ ] Podman mode runs in containers (shows container logs)
- [ ] Docker mode runs in containers (shows container logs)
- [ ] Emulation mode runs on host system
- [ ] All modes produce similar output

### ✅ **Step 6: Error Handling**
```bash
# Test with Podman unavailable (temporarily rename podman binary)
sudo mv /usr/local/bin/podman /usr/local/bin/podman.tmp 2>/dev/null || echo "podman not in /usr/local/bin"
./target/release/wrkflw run --runtime podman test-runtime.yml
sudo mv /usr/local/bin/podman.tmp /usr/local/bin/podman 2>/dev/null || echo "nothing to restore"
```
**Verify:**
- [ ] Shows "Podman is not available. Using emulation mode instead."
- [ ] Falls back to emulation gracefully
- [ ] Workflow still executes successfully

### ✅ **Step 7: Container Preservation**
```bash
# Create a failing workflow
echo 'name: Fail Test
on: [workflow_dispatch]
jobs:
  fail:
    runs-on: ubuntu-latest
    container: ubuntu:20.04
    steps:
      - run: exit 1' > test-fail.yml

# Test with preservation
./target/release/wrkflw run --runtime podman --preserve-containers-on-failure test-fail.yml

# Check for preserved containers
podman ps -a --filter "name=wrkflw-"
```
**Verify:**
- [ ] Failed container is preserved when flag is used
- [ ] Container can be inspected with `podman exec -it <container> bash`
- [ ] Without flag, containers are cleaned up

### ✅ **Step 8: Documentation**
**Verify:**
- [ ] README.md mentions Podman support
- [ ] Examples show `--runtime podman` usage
- [ ] TUI keybind documentation mentions runtime cycling
- [ ] Installation instructions for Podman are present

## Platform-Specific Tests

### **Linux:**
- [ ] Podman works rootless
- [ ] No sudo required for container operations
- [ ] Network connectivity works in containers

### **macOS:**
- [ ] Podman machine is initialized and running
- [ ] Container execution works correctly
- [ ] Volume mounting works for workspace

### **Windows:**
- [ ] Podman Desktop or CLI is installed
- [ ] Basic container operations work
- [ ] Workspace mounting functions correctly

## Performance and Resource Tests

### **Memory Usage:**
```bash
# Monitor memory during execution
./target/release/wrkflw run --runtime podman test-runtime.yml &
PID=$!
while kill -0 $PID 2>/dev/null; do
    ps -p $PID -o pid,ppid,pgid,sess,cmd,%mem,%cpu
    sleep 2
done
```
**Verify:**
- [ ] Memory usage is reasonable
- [ ] No memory leaks during execution

### **Container Cleanup:**
```bash
# Run multiple workflows and check cleanup
for i in {1..3}; do
    ./target/release/wrkflw run --runtime podman test-runtime.yml
done
podman ps -a --filter "name=wrkflw-"
```
**Verify:**
- [ ] No containers remain after execution
- [ ] Cleanup is thorough and automatic

## Integration Tests

### **Complex Workflow:**
Test with a workflow that has:
- [ ] Multiple jobs
- [ ] Environment variables
- [ ] File operations
- [ ] Network access
- [ ] Package installation

### **Edge Cases:**
- [ ] Very long-running containers
- [ ] Large output logs
- [ ] Network-intensive operations
- [ ] File system intensive operations

## Final Verification

**Overall System Check:**
- [ ] All runtimes work as expected
- [ ] Error messages are clear and helpful
- [ ] Performance is acceptable
- [ ] User experience is smooth
- [ ] Documentation is accurate and complete

**Sign-off:**
- [ ] Basic functionality: ✅ PASS / ❌ FAIL
- [ ] CLI integration: ✅ PASS / ❌ FAIL  
- [ ] TUI integration: ✅ PASS / ❌ FAIL
- [ ] Error handling: ✅ PASS / ❌ FAIL
- [ ] Documentation: ✅ PASS / ❌ FAIL

**Notes:**
_Add any specific issues, observations, or platform-specific notes here._

---

**Testing completed by:** ________________  
**Date:** ________________  
**Platform:** ________________  
**Podman version:** ________________
