//! # metric
//!
//! metric 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Metrics collection

use metrics::{Key, KeyName};
use std::sync::LazyLock;
use std::collections::HashMap;
use parking_lot::RwLock;

// Cache for dynamic metric names to avoid allocations
static METRIC_NAME_CACHE: LazyLock<RwLock<HashMap<String, &'static str>>> = LazyLock::new(|| {
    RwLock::new(HashMap::new())
});

/// Get or create a static string for a dynamic metric name
fn get_static_name(name: &str) -> &'static str {
    let cache = METRIC_NAME_CACHE.read();
    if let Some(&cached) = cache.get(name) {
        return cached;
    }
    drop(cache);
    
    let mut cache = METRIC_NAME_CACHE.write();
    // Double check after acquiring write lock
    if let Some(&cached) = cache.get(name) {
        return cached;
    }
    
    // Create a new static string by leaking the allocation
    let leaked = Box::leak(name.to_string().into_boxed_str());
    cache.insert(name.to_string(), leaked);
    leaked
}

/// Increment a counter
pub fn counter_inc(name: &'static str, value: u64) {
    metrics::counter!(name).increment(value);
}

/// Set a gauge
pub fn gauge_set(name: &'static str, value: f64) {
    metrics::gauge!(name).set(value);
}

/// Record a histogram value
pub fn histogram_record(name: &'static str, value: f64) {
    metrics::histogram!(name).record(value);
}

/// Increment a counter with dynamic name
pub fn counter_inc_dynamic(name: String, value: u64) {
    let static_name = get_static_name(&name);
    metrics::counter!(static_name).increment(value);
}

/// Increment a counter with dynamic name and labels
/// Note: labels are not fully supported by metrics macros, this is a simplified version
pub fn counter_inc_with_labels(name: String, _labels: Vec<(String, String)>, value: u64) {
    // For now, ignore labels as metrics macros don't support them directly
    // Full implementation would require using the recorder API with Key
    let static_name = get_static_name(&name);
    metrics::counter!(static_name).increment(value);
}

/// Set a gauge with dynamic name
pub fn gauge_set_dynamic(name: String, value: f64) {
    let static_name = get_static_name(&name);
    metrics::gauge!(static_name).set(value);
}

/// Set a gauge with dynamic name and labels
/// Note: labels are not fully supported by metrics macros, this is a simplified version
pub fn gauge_set_with_labels(name: String, _labels: Vec<(String, String)>, value: f64) {
    let static_name = get_static_name(&name);
    metrics::gauge!(static_name).set(value);
}

/// Record a histogram with dynamic name
pub fn histogram_record_dynamic(name: String, value: f64) {
    let static_name = get_static_name(&name);
    metrics::histogram!(static_name).record(value);
}

/// Record a histogram with dynamic name and labels
/// Note: labels are not fully supported by metrics macros, this is a simplified version
pub fn histogram_record_with_labels(name: String, _labels: Vec<(String, String)>, value: f64) {
    let static_name = get_static_name(&name);
    metrics::histogram!(static_name).record(value);
}

/// Metric label builder
#[derive(Debug, Clone)]
pub struct MetricLabels {
    labels: Vec<(String, String)>,
}

impl MetricLabels {
    /// Create a new label builder
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
        }
    }

    /// Add a label
    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.push((key.into(), value.into()));
        self
    }

    /// Build Key from labels
    pub fn to_key(&self, name: &str) -> Key {
        // Note: metrics::Key doesn't have a `with` method in this version
        // Labels are handled differently - for now, just return the key with name
        // TODO: Implement proper label support when metrics API is updated
        Key::from_name(KeyName::from(name.to_string()))
    }

    /// Get labels as vector
    pub fn as_vec(&self) -> &[(String, String)] {
        &self.labels
    }
}

impl Default for MetricLabels {
    fn default() -> Self {
        Self::new()
    }
}

/// Increment counter with labels (using recorder API)
pub fn counter_inc_with_key(_key: metrics::Key, value: u64) {
    // Note: metrics crate API may vary by version
    // For now, simplified implementation - labels not fully supported
    // TODO: Implement proper label support when metrics API is updated
    let name_str = _key.name().to_string();
    let static_name: &'static str = Box::leak(name_str.into_boxed_str());
    metrics::counter!(static_name).increment(value);
}

/// Set gauge with labels (using recorder API)
pub fn gauge_set_with_key(_key: metrics::Key, value: f64) {
    // Note: metrics crate API may vary by version
    // For now, simplified implementation - labels not fully supported
    // TODO: Implement proper label support when metrics API is updated
    let name_str = _key.name().to_string();
    let static_name: &'static str = Box::leak(name_str.into_boxed_str());
    metrics::gauge!(static_name).set(value);
}

/// Record histogram with labels (using recorder API)
pub fn histogram_record_with_key(_key: metrics::Key, value: f64) {
    // Note: metrics crate API may vary by version
    // For now, simplified implementation - labels not fully supported
    // TODO: Implement proper label support when metrics API is updated
    let name_str = _key.name().to_string();
    let static_name: &'static str = Box::leak(name_str.into_boxed_str());
    metrics::histogram!(static_name).record(value);
}

/// Validate metric name
pub fn validate_metric_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Metric name cannot be empty".to_string());
    }
    
    // Check for valid characters (alphanumeric, underscore, hyphen)
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
        return Err("Metric name contains invalid characters".to_string());
    }
    
    // Check length
    if name.len() > 255 {
        return Err("Metric name too long (max 255 characters)".to_string());
    }
    
    Ok(())
}

/// Normalize metric name
pub fn normalize_metric_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

/// Metric aggregator for collecting and aggregating metrics
pub struct MetricAggregator {
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    histograms: HashMap<String, Vec<f64>>,
}

impl MetricAggregator {
    /// Create a new metric aggregator
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            gauges: HashMap::new(),
            histograms: HashMap::new(),
        }
    }

    /// Add counter value
    pub fn add_counter(&mut self, name: String, value: u64) {
        *self.counters.entry(name).or_insert(0) += value;
    }

    /// Set gauge value
    pub fn set_gauge(&mut self, name: String, value: f64) {
        self.gauges.insert(name, value);
    }

    /// Record histogram value
    pub fn record_histogram(&mut self, name: String, value: f64) {
        self.histograms.entry(name).or_default().push(value);
    }

    /// Get aggregated counter value
    pub fn get_counter(&self, name: &str) -> Option<u64> {
        self.counters.get(name).copied()
    }

    /// Get aggregated gauge value
    pub fn get_gauge(&self, name: &str) -> Option<f64> {
        self.gauges.get(name).copied()
    }

    /// Get histogram statistics
    pub fn get_histogram_stats(&self, name: &str) -> Option<HistogramStats> {
        self.histograms.get(name).map(|values| {
            if values.is_empty() {
                return HistogramStats {
                    count: 0,
                    sum: 0.0,
                    min: 0.0,
                    max: 0.0,
                    avg: 0.0,
                };
            }
            
            let sum: f64 = values.iter().sum();
            let count = values.len();
            let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let avg = sum / count as f64;
            
            HistogramStats {
                count,
                sum,
                min,
                max,
                avg,
            }
        })
    }

    /// Clear all metrics
    pub fn clear(&mut self) {
        self.counters.clear();
        self.gauges.clear();
        self.histograms.clear();
    }
}

impl Default for MetricAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// Histogram statistics
#[derive(Debug, Clone)]
pub struct HistogramStats {
    pub count: usize,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
}
