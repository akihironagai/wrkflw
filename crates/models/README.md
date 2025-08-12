## wrkflw-models

Common data structures shared across crates.

- `ValidationResult` for structural/semantic checks
- GitLab pipeline models (serde types)

### Example

```rust
use wrkflw_models::ValidationResult;

let mut res = ValidationResult::new();
res.add_issue("missing jobs".into());
assert!(!res.is_valid);
```
