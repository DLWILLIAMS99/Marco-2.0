//! Built-in templates for Marco 2.0

use super::TemplateDefinition;

#[derive(Debug, Clone)]
pub enum BuiltinTemplate {
    Calculator,
    Basic2DGame,
    Blank,
}

pub fn get_template(template: BuiltinTemplate) -> String {
    match template {
        BuiltinTemplate::Calculator => "Calculator: AddNode, ButtonNode, SliderNode".to_string(),
        BuiltinTemplate::Basic2DGame => "2D Game: ButtonNode, SliderNode".to_string(),
        BuiltinTemplate::Blank => "Blank Project".to_string(),
    }
}

pub fn get_builtin_templates() -> Vec<TemplateDefinition> {
    vec![
        TemplateDefinition::new(
            "blank".to_string(),
            "Blank Project".to_string(), 
            "Empty project template".to_string(),
        ),
        TemplateDefinition::new(
            "calculator".to_string(),
            "Calculator".to_string(),
            "Calculator application with AddNode, ButtonNode, SliderNode".to_string(),
        ),
        TemplateDefinition::new(
            "basic_2d_game".to_string(),
            "Basic 2D Game".to_string(),
            "Simple 2D game template with ButtonNode, SliderNode".to_string(),
        ),
    ]
}
