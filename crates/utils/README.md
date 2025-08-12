## wrkflw-utils

Shared helpers used across crates.

- Workflow file detection (`.github/workflows/*.yml`, `.gitlab-ci.yml`)
- File-descriptor redirection utilities for silencing noisy subprocess output

### Example

```rust
use std::path::Path;
use wrkflw_utils::{is_workflow_file, fd::with_stderr_to_null};

assert!(is_workflow_file(Path::new(".github/workflows/ci.yml")));

let value = with_stderr_to_null(|| {
    eprintln!("this is hidden");
    42
}).unwrap();
assert_eq!(value, 42);
```
