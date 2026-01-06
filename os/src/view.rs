//! # view
//!
//! view 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! View template engine

use rf_errors::Result;
use serde::Serialize;
use tera::{Tera, Context};

/// Template engine wrapper
pub struct View {
    tera: Tera,
}

impl View {
    /// Create a new view engine
    pub fn new(template_dir: &str) -> Result<Self> {
        let tera = Tera::new(template_dir)
            .map_err(|e| rf_errors::RfError::Internal(format!("Failed to initialize Tera: {}", e)))?;
        Ok(Self { tera })
    }

    /// Render a template
    pub fn render<T: Serialize>(&self, template: &str, data: &T) -> Result<String> {
        let context = Context::from_serialize(data)
            .map_err(|e| rf_errors::RfError::Internal(format!("Failed to create context: {}", e)))?;
        self.tera.render(template, &context)
            .map_err(|e| rf_errors::RfError::Internal(format!("Template render failed: {}", e)))
    }

    /// Render a template with context
    pub fn render_with_context(&self, template: &str, context: &Context) -> Result<String> {
        self.tera.render(template, context)
            .map_err(|e| rf_errors::RfError::Internal(format!("Template render failed: {}", e)))
    }
}
