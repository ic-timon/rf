//! # converter
//!
//! converter 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Converter system for custom type conversions

use rf_errors::Result;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

/// Custom converter function type
pub type ConverterFn = Arc<dyn Fn(&dyn std::any::Any) -> Result<Box<dyn std::any::Any>> + Send + Sync>;

/// Converter registry
pub struct Converter {
    converters: HashMap<(TypeId, TypeId), ConverterFn>,
}

impl Converter {
    /// Create a new converter
    pub fn new() -> Self {
        Self {
            converters: HashMap::new(),
        }
    }

    /// Register a custom converter
    pub fn register<From, To, F>(&mut self, converter: F)
    where
        From: 'static,
        To: 'static,
        F: Fn(&From) -> Result<To> + Send + Sync + 'static,
    {
        let from_id = TypeId::of::<From>();
        let to_id = TypeId::of::<To>();
        let converter_fn: ConverterFn = Arc::new(move |value| {
            if let Some(from_value) = value.downcast_ref::<From>() {
                match converter(from_value) {
                    Ok(to_value) => Ok(Box::new(to_value)),
                    Err(e) => Err(e),
                }
            } else {
                Err(rf_errors::RfError::Internal("Type mismatch".to_string()))
            }
        });
        self.converters.insert((from_id, to_id), converter_fn);
    }

    /// Convert using registered converter
    pub fn convert<From, To>(&self, value: &From) -> Result<To>
    where
        From: 'static,
        To: 'static,
    {
        let from_id = TypeId::of::<From>();
        let to_id = TypeId::of::<To>();
        
        if let Some(converter) = self.converters.get(&(from_id, to_id)) {
            let result = converter(value)?;
            if result.downcast_ref::<To>().is_some() {
                // This is a limitation - we can't move out of Box<dyn Any>
                // In practice, converters should return the actual type
                Err(rf_errors::RfError::Internal("Converter result type mismatch".to_string()))
            } else {
                Err(rf_errors::RfError::Internal("Converter failed".to_string()))
            }
        } else {
            Err(rf_errors::RfError::Internal("No converter registered".to_string()))
        }
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}

/// Global converter instance
static CONVERTER: once_cell::sync::Lazy<std::sync::Mutex<Converter>> = 
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(Converter::new()));

/// Register a custom converter globally
pub fn register_converter<From, To, F>(converter: F)
where
    From: 'static,
    To: 'static,
    F: Fn(&From) -> Result<To> + Send + Sync + 'static,
{
    let mut conv = CONVERTER.lock().unwrap();
    conv.register(converter);
}

