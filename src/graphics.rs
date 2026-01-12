//! EPD 显示图形支持

use crate::color::{ColorType, QuadColor};
use core::marker::PhantomData;
use embedded_graphics_core::prelude::*;

/// 计算每行字节数（考虑填充位）
const fn line_bytes(width: u32, bits_per_pixel: usize) -> usize {
    (width as usize * bits_per_pixel + 7) / 8
}

/// 用于 embedded graphics 的显示缓冲区
///
/// - WIDTH: 显示宽度（像素）
/// - HEIGHT: 显示高度（像素）
/// - COLOR: 目标显示使用的颜色类型
/// - BYTECOUNT: 缓冲区字节数
pub struct Display<
    const WIDTH: u32,
    const HEIGHT: u32,
    const BYTECOUNT: usize,
    COLOR: ColorType + PixelColor,
> {
    buffer: [u8; BYTECOUNT],
    _color: PhantomData<COLOR>,
}

impl<
        const WIDTH: u32,
        const HEIGHT: u32,
        const BYTECOUNT: usize,
        COLOR: ColorType + PixelColor,
    > Default for Display<WIDTH, HEIGHT, BYTECOUNT, COLOR>
{
    /// 初始化显示缓冲区，默认为白色
    #[inline(always)]
    fn default() -> Self {
        Self {
            buffer: [QuadColor::default_color_byte(); BYTECOUNT],
            _color: PhantomData,
        }
    }
}

/// 用于 embedded graphics 绘图
impl<
        const WIDTH: u32,
        const HEIGHT: u32,
        const BYTECOUNT: usize,
        COLOR: ColorType + PixelColor,
    > DrawTarget for Display<WIDTH, HEIGHT, BYTECOUNT, COLOR>
{
    type Color = COLOR;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.set_pixel(pixel);
        }
        Ok(())
    }
}

/// 用于 embedded graphics 获取尺寸
impl<
        const WIDTH: u32,
        const HEIGHT: u32,
        const BYTECOUNT: usize,
        COLOR: ColorType + PixelColor,
    > OriginDimensions for Display<WIDTH, HEIGHT, BYTECOUNT, COLOR>
{
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}

impl<
        const WIDTH: u32,
        const HEIGHT: u32,
        const BYTECOUNT: usize,
        COLOR: ColorType + PixelColor,
    > Display<WIDTH, HEIGHT, BYTECOUNT, COLOR>
{
    /// 获取内部缓冲区引用
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// 设置指定像素颜色
    pub fn set_pixel(&mut self, pixel: Pixel<COLOR>) {
        set_pixel(&mut self.buffer, WIDTH, HEIGHT, pixel);
    }
}

/// 设置缓冲区中指定像素的颜色
fn set_pixel<COLOR: ColorType + PixelColor>(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    pixel: Pixel<COLOR>,
) {
    let Pixel(point, color) = pixel;
    let x = point.x;
    let y = point.y;

    // 越界检查
    if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
        return;
    }

    let index = x as usize * COLOR::BITS_PER_PIXEL / 8
        + y as usize * line_bytes(width, COLOR::BITS_PER_PIXEL);
    let (mask, bits) = color.bitmask(x as u32);

    buffer[index] = buffer[index] & mask | bits as u8;
}
