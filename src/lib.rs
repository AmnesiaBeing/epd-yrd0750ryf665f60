//! 电子墨水屏驱动
//!
//! 基于 [`embedded-hal`] traits 构建
//! 通过 [`embedded-graphics`] 提供图形支持
//!
//! [`embedded-graphics`]: https://docs.rs/embedded-graphics/
//! [`embedded-hal`]: https://docs.rs/embedded-hal
#![cfg_attr(not(feature = "simulator"), no_std)]
#![deny(missing_docs)]

#[cfg(feature = "graphics")]
pub mod graphics;

mod traits;

pub mod color;

/// 显示与控制设备之间的物理连接接口
mod interface;

pub mod yrd0750ryf665f60;

/// 包含除选定显示类型外的所有重要内容
pub mod prelude {
    pub use crate::color::QuadColor;
    pub use crate::traits::WaveshareDisplay;

    #[cfg(feature = "graphics")]
    pub use crate::graphics::Display;
}

/// 计算所需的缓冲区长度。处理宽度不能被 8 整除时的向上取整
///
///  未使用
///  位        宽度
/// <----><------------------------>
/// \[XXXXX210\]\[76543210\]...\[76543210\] ^
/// \[XXXXX210\]\[76543210\]...\[76543210\] | 高度
/// \[XXXXX210\]\[76543210\]...\[76543210\] v
pub const fn buffer_len(width: usize, height: usize) -> usize {
    (width + 7) / 8 * height
}
