use std::collections::HashMap;
use std::fs;

use anyhow::{Context, Result};
use serde_json::Value;
use tera::Tera;

use crate::models::{TemplateJob, Theme};

/// Builds a nested `serde_json::Value` from a flat HashMap of dot-notation keys.
///
/// For example:
/// ```text
/// { "colors.core.background" => "#1e1e2e" }
/// ```
/// becomes:
/// ```json
/// { "colors": { "core": { "background": "#1e1e2e" } } }
/// ```
fn build_nested_context(variables: &HashMap<String, String>) -> Value {
    let mut root = serde_json::Map::new();

    for (key, value) in variables {
        let parts: Vec<&str> = key.split('.').collect();
        insert_nested(&mut root, &parts, value.clone());
    }

    Value::Object(root)
}

/// Recursively inserts a value into a nested JSON object structure.
fn insert_nested(obj: &mut serde_json::Map<String, Value>, parts: &[&str], value: String) {
    if parts.is_empty() {
        return;
    }

    if parts.len() == 1 {
        obj.insert(parts[0].to_string(), Value::String(value));
        return;
    }

    let key = parts[0];
    let rest = &parts[1..];

    let child = obj
        .entry(key.to_string())
        .or_insert_with(|| Value::Object(serde_json::Map::new()));

    if let Value::Object(child_map) = child {
        insert_nested(child_map, rest, value);
    }
}

/// Renders all template jobs using the provided theme.
///
/// For each [`TemplateJob`]:
/// 1. Reads the raw template from `job.source`
/// 2. Renders it via Tera with the theme's variables as context
/// 3. Ensures the parent directory of `job.destination` exists
/// 4. Writes the rendered output to `job.destination`
pub fn render_templates(jobs: &[TemplateJob], theme: &Theme) -> Result<()> {
    // Build the nested context from flat dot-notation variables
    let nested = build_nested_context(&theme.variables);

    // Build Tera context
    let mut context = tera::Context::new();
    if let Value::Object(map) = &nested {
        for (key, val) in map {
            context.insert(key, val);
        }
    }

    for job in jobs {
        // Read the raw template content
        let content = fs::read_to_string(&job.source)
            .with_context(|| format!("failed to read template: {}", job.source.display()))?;

        // Render via Tera
        let rendered = Tera::one_off(&content, &context, false)
            .with_context(|| format!("failed to render template: {}", job.source.display()))?;

        // Ensure parent directory exists
        if let Some(parent) = job.destination.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create parent directories for: {}",
                    job.destination.display()
                )
            })?;
        }

        // Write rendered output
        fs::write(&job.destination, &rendered).with_context(|| {
            format!(
                "failed to write rendered file: {}",
                job.destination.display()
            )
        })?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // -----------------------------------------------------------------------
    // basic rendering with variable substitution
    // -----------------------------------------------------------------------
    #[test]
    fn test_render_template_basic() {
        let tmp = TempDir::new().unwrap();

        // Create a .tera template file
        let source = tmp.path().join("test.conf.tera");
        fs::write(
            &source,
            "foreground {{ colors.core.foreground }}\nbackground {{ colors.core.background }}",
        )
        .unwrap();

        let destination = tmp.path().join("test.conf");

        let mut variables = HashMap::new();
        variables.insert("colors.core.foreground".to_string(), "#ffffff".to_string());
        variables.insert("colors.core.background".to_string(), "#000000".to_string());

        let theme = Theme {
            name: "test-theme".to_string(),
            path: tmp.path().join("test-theme.toml"),
            variables,
        };

        let jobs = vec![TemplateJob {
            source: source.clone(),
            destination: destination.clone(),
        }];

        render_templates(&jobs, &theme).unwrap();

        let output = fs::read_to_string(&destination).unwrap();
        assert_eq!(output, "foreground #ffffff\nbackground #000000");
    }

    // -----------------------------------------------------------------------
    // creates parent directories if they don't exist
    // -----------------------------------------------------------------------
    #[test]
    fn test_render_template_creates_parent_dirs() {
        let tmp = TempDir::new().unwrap();

        // Create a .tera template file
        let source = tmp.path().join("test.conf.tera");
        fs::write(&source, "hello world").unwrap();

        // Destination with non-existent nested parent dirs
        let destination = tmp.path().join("deep").join("nested").join("output.conf");

        let theme = Theme {
            name: "test-theme".to_string(),
            path: tmp.path().join("test-theme.toml"),
            variables: HashMap::new(),
        };

        let jobs = vec![TemplateJob {
            source: source.clone(),
            destination: destination.clone(),
        }];

        render_templates(&jobs, &theme).unwrap();

        assert!(destination.exists(), "output file should exist");
        assert!(
            destination.parent().unwrap().exists(),
            "parent dirs should have been created"
        );
    }

    // -----------------------------------------------------------------------
    // returns an error when a template variable is missing
    // -----------------------------------------------------------------------
    #[test]
    fn test_render_template_missing_variable() {
        let tmp = TempDir::new().unwrap();

        // Template references a variable that won't be in the context
        let source = tmp.path().join("test.conf.tera");
        fs::write(&source, "value {{ colors.nonexistent }}").unwrap();

        let destination = tmp.path().join("output.conf");

        let theme = Theme {
            name: "test-theme".to_string(),
            path: tmp.path().join("test-theme.toml"),
            variables: HashMap::new(), // empty — no variables provided
        };

        let jobs = vec![TemplateJob {
            source: source.clone(),
            destination: destination.clone(),
        }];

        let result = render_templates(&jobs, &theme);

        assert!(
            result.is_err(),
            "render_templates should return an error for missing variables"
        );
    }
}
