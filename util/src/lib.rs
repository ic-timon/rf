//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF Util Module
//!
//! Provides utility functions.

pub mod conv {
    pub mod basic;
    pub mod int;
    pub mod uint;
    pub mod float;
    pub mod time;
    pub mod slice;
    pub mod map;
    pub mod struct_conv;
    pub mod scan;
    pub mod converter;

    pub use basic::*;
    pub use int::*;
    pub use uint::*;
    pub use float::*;
    pub use time::*;
    pub use slice::*;
    // Use specific imports to avoid conflicts
    pub use map::{map_to_map};
    pub use struct_conv::{struct_to_map, map_to_struct};
    pub use scan::*;
    pub use converter::*;
}
pub mod meta;
pub mod mode;
pub mod page;
pub mod rand;
pub mod tag;
pub mod guid;
pub mod util;
pub mod valid;

pub use conv::*;
pub use meta::*;
pub use mode::*;
pub use page::*;
// Use specific imports to avoid conflicts with rand::string
pub use rand::{int, int_range, float, float_range, string as rand_string};
pub use tag::*;
pub use guid::*;
pub use util::*;
pub use valid::*;

