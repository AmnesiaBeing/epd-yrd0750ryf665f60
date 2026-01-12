//! EPD 颜色定义

#[cfg(feature = "graphics")]
use embedded_graphics_core::pixelcolor::BinaryColor;
#[cfg(feature = "graphics")]
use embedded_graphics_core::pixelcolor::PixelColor;

/// 黑/白/红/黄显示颜色
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum QuadColor {
    /// 黑色
    Black,
    /// 白色
    #[default]
    White,
    /// 红色
    Red,
    /// 黄色
    Yellow,
}

/// 颜色类型 trait，用于 `Display`
pub trait ColorType {
    /// 每个像素占用的位数
    const BITS_PER_PIXEL: usize;

    /// 返回设置像素颜色所需的数据
    ///
    /// * pos: 像素在行中的位置，用于确定需要设置哪些像素
    ///
    /// 返回值：
    /// * .0: 用于从字节中排除此像素的掩码（如 BiColor 中的 0x7F）
    /// * .1: 用于在字节中设置颜色的位（如 BiColor 中的 0x80）
    fn bitmask(&self, pos: u32) -> (u8, u16);

    /// 从位转换为颜色
    /// 二进制颜色：0 -> 黑色，1 -> 白色
    /// 三色：00 -> 黑色，01 -> 白色，11 -> 红色
    /// 以此类推
    fn from_bits(bits: u8) -> Self;
}

impl QuadColor {
    /// 计算默认颜色字节值
    /// 
    /// 对于四色显示，白色值0b01填充一个字节生成0x55（二进制0b01010101）
    /// 这个值适用于单缓冲区墨水屏的默认初始化
    pub const fn default_color_byte() -> u8 {
        0x55 // 0b01010101
    }
}

impl ColorType for QuadColor {
    const BITS_PER_PIXEL: usize = 2;

    fn bitmask(&self, pos: u32) -> (u8, u16) {
        let shift = if cfg!(feature = "simulator") {
            (pos % 4) * 2
        } else {
            6 - (pos % 4) * 2
        };
        let mask = !(0x03 << shift);
        let color_bits = match self {
            QuadColor::Black => 0b00,
            QuadColor::White => 0b01,
            QuadColor::Yellow => 0b10,
            QuadColor::Red => 0b11,
        };
        let value = (color_bits << shift) as u16;

        (mask, value)
    }

    fn from_bits(bits: u8) -> Self {
        match bits {
            0b01 => QuadColor::White,
            0b10 => QuadColor::Yellow,
            0b11 => QuadColor::Red,
            _ => QuadColor::Black,
        }
    }
}

#[cfg(feature = "graphics")]
impl PixelColor for QuadColor {
    type Raw = embedded_graphics_core::pixelcolor::raw::RawU2;
}

#[cfg(feature = "graphics")]
impl From<embedded_graphics_core::pixelcolor::raw::RawU2> for QuadColor {
    fn from(raw: embedded_graphics_core::pixelcolor::raw::RawU2) -> Self {
        match embedded_graphics_core::prelude::RawData::into_inner(raw) {
            0b00 => QuadColor::Black,
            0b01 => QuadColor::White,
            0b10 => QuadColor::Yellow,
            _ => QuadColor::Red, // 0b11
        }
    }
}

#[cfg(feature = "graphics")]
impl From<BinaryColor> for QuadColor {
    fn from(b: BinaryColor) -> QuadColor {
        match b {
            BinaryColor::On => QuadColor::Black,
            BinaryColor::Off => QuadColor::White,
        }
    }
}

#[cfg(feature = "graphics")]
impl From<embedded_graphics_core::pixelcolor::Rgb888> for QuadColor {
    fn from(rgb: embedded_graphics_core::pixelcolor::Rgb888) -> Self {
        use embedded_graphics_core::pixelcolor::RgbColor;
        if rgb == RgbColor::BLACK {
            QuadColor::Black
        } else if rgb == RgbColor::WHITE {
            QuadColor::White
        } else if rgb == RgbColor::YELLOW {
            QuadColor::Yellow
        } else {
            QuadColor::Red
        }
    }
}

#[cfg(feature = "graphics")]
impl From<QuadColor> for embedded_graphics_core::pixelcolor::Rgb888 {
    fn from(quad_color: QuadColor) -> Self {
        match quad_color {
            QuadColor::Black => embedded_graphics_core::pixelcolor::Rgb888::new(10, 10, 10),
            QuadColor::White => embedded_graphics_core::pixelcolor::Rgb888::new(240, 240, 240),
            QuadColor::Yellow => embedded_graphics_core::pixelcolor::Rgb888::new(240, 240, 100),
            QuadColor::Red => embedded_graphics_core::pixelcolor::Rgb888::new(200, 50, 50),
        }
    }
}
