// frontend/src/editor/config.rs

use serde::{Deserialize, Serialize};

/// Puck editor configuration per D-008
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuckConfig {
    pub components: Vec<ComponentDefinition>,
    pub allowed_nesting_depth: i32,
    pub max_node_count: i32,
    pub field_text_caps: FieldTextCaps,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentDefinition {
    pub name: String,
    pub label: String,
    pub fields: Vec<FieldDefinition>,
    pub allow_in: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub label: String,
    pub field_type: String,
    pub required: bool,
    pub max_length: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldTextCaps {
    pub hero_title: i32,
    pub hero_description: i32,
    pub section_title: i32,
    pub section_description: i32,
}

impl Default for PuckConfig {
    fn default() -> Self {
        Self {
            components: vec![
                ComponentDefinition {
                    name: "Hero".to_string(),
                    label: "بطل الصفحة".to_string(),
                    fields: vec![
                        FieldDefinition {
                            name: "title".to_string(),
                            label: "العنوان".to_string(),
                            field_type: "text".to_string(),
                            required: true,
                            max_length: Some(200),
                        },
                        FieldDefinition {
                            name: "description".to_string(),
                            label: "الوصف".to_string(),
                            field_type: "textarea".to_string(),
                            required: false,
                            max_length: Some(500),
                        },
                    ],
                    allow_in: vec!["root".to_string()],
                },
                ComponentDefinition {
                    name: "Section".to_string(),
                    label: "قسم".to_string(),
                    fields: vec![
                        FieldDefinition {
                            name: "title".to_string(),
                            label: "العنوان".to_string(),
                            field_type: "text".to_string(),
                            required: false,
                            max_length: Some(200),
                        },
                    ],
                    allow_in: vec!["root".to_string()],
                },
                ComponentDefinition {
                    name: "ProductGrid".to_string(),
                    label: "شبكة المنتجات".to_string(),
                    fields: vec![
                        FieldDefinition {
                            name: "title".to_string(),
                            label: "العنوان".to_string(),
                            field_type: "text".to_string(),
                            required: false,
                            max_length: Some(200),
                        },
                        FieldDefinition {
                            name: "category_id".to_string(),
                            label: "الفئة".to_string(),
                            field_type: "select".to_string(),
                            required: false,
                            max_length: None,
                        },
                    ],
                    allow_in: vec!["root".to_string(), "Section".to_string()],
                },
            ],
            allowed_nesting_depth: 4,
            max_node_count: 200,
            field_text_caps: FieldTextCaps {
                hero_title: 200,
                hero_description: 500,
                section_title: 200,
                section_description: 500,
            },
        }
    }
}
