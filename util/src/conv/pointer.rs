//! # pointer
//!
//! pointer 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Pointer type conversion

use rf_errors::Result;

/// Convert value to pointer
pub fn to_ptr<T>(value: T) -> *const T {
    &value as *const T
}

/// Convert value to mutable pointer
pub fn to_mut_ptr<T>(value: &mut T) -> *mut T {
    value as *mut T
}

/// Convert pointer to reference (unsafe)
pub unsafe fn from_ptr<'a, T>(ptr: *const T) -> Option<&'a T> {
    if ptr.is_null() {
        None
    } else {
        Some(&*ptr)
    }
}

/// Convert mutable pointer to mutable reference (unsafe)
pub unsafe fn from_mut_ptr<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    if ptr.is_null() {
        None
    } else {
        Some(&mut *ptr)
    }
}

/// Convert value to boxed pointer
pub fn to_box<T>(value: T) -> Box<T> {
    Box::new(value)
}

/// Convert boxed pointer to value
pub fn from_box<T>(value: Box<T>) -> T {
    *value
}

/// Convert value to reference counted pointer
pub fn to_rc<T>(value: T) -> std::rc::Rc<T> {
    std::rc::Rc::new(value)
}

/// Convert value to atomic reference counted pointer
pub fn to_arc<T>(value: T) -> std::sync::Arc<T> {
    std::sync::Arc::new(value)
}

