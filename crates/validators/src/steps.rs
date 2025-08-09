use crate::validate_action_reference;
use models::ValidationResult;
use serde_yaml::Value;
use std::collections::HashSet;

pub fn validate_steps(steps: &[Value], job_name: &str, result: &mut ValidationResult) {
    let mut step_ids: HashSet<String> = HashSet::new();

    for (i, step) in steps.iter().enumerate() {
        if let Some(step_map) = step.as_mapping() {
            if !step_map.contains_key(Value::String("name".to_string()))
                && !step_map.contains_key(Value::String("uses".to_string()))
                && !step_map.contains_key(Value::String("run".to_string()))
            {
                result.add_issue(format!(
                    "Job '{}', step {}: Missing 'name', 'uses', or 'run' field",
                    job_name,
                    i + 1
                ));
            }

            // Check for both 'uses' and 'run' in the same step
            if step_map.contains_key(Value::String("uses".to_string()))
                && step_map.contains_key(Value::String("run".to_string()))
            {
                result.add_issue(format!(
                    "Job '{}', step {}: Contains both 'uses' and 'run' (should only use one)",
                    job_name,
                    i + 1
                ));
            }

            // Check for duplicate step IDs
            if let Some(Value::String(id)) = step_map.get(Value::String("id".to_string())) {
                if !step_ids.insert(id.clone()) {
                    result.add_issue(format!(
                        "Job '{}', step {}: The identifier '{}' may not be used more than once within the same scope",
                        job_name,
                        i + 1,
                        id
                    ));
                }
            }

            // Validate action reference if 'uses' is present
            if let Some(Value::String(uses)) = step_map.get(Value::String("uses".to_string())) {
                validate_action_reference(uses, job_name, i, result);
            }
        } else {
            result.add_issue(format!(
                "Job '{}', step {}: Not a valid mapping",
                job_name,
                i + 1
            ));
        }
    }
}
