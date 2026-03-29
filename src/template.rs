use std::collections::HashMap;
use tera::{Context, Tera};

pub fn render(template_str: &str, vars: &HashMap<&str, &str>) -> Result<String, tera::Error> {
    let mut context = Context::new();
    for (k, v) in vars {
        context.insert(*k, v);
    }
    Tera::one_off(template_str, &context, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_contains_font_family() {
        let mut vars = HashMap::new();
        vars.insert("font_family", "JetBrainsMono");
        let result = render("Hello {{ font_family }}!", &vars).unwrap();
        assert_eq!(result, "Hello JetBrainsMono!");
    }

    #[test]
    fn test_render_contains_font_size() {
        let mut vars = HashMap::new();
        vars.insert("font_size", "12");
        let result = render("Size: {{ font_size }}", &vars).unwrap();
        assert_eq!(result, "Size: 12");
    }

    #[test]
    fn test_render_with_multiple_values() {
        let mut vars = HashMap::new();
        vars.insert("font_family", "JetBrainsMono");
        vars.insert("font_size", "14");
        let result = render("Font: {{ font_family }}, Size: {{ font_size }}", &vars).unwrap();
        assert_eq!(result, "Font: JetBrainsMono, Size: 14");
    }

    #[test]
    fn test_render_unknown_variable_errors() {
        let vars = HashMap::new();
        let result = render("{{ undefined_var }}", &vars);
        assert!(result.is_err());
    }
}
