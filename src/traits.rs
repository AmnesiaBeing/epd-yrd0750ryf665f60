#![allow(async_fn_in_trait)]

use core::marker::Sized;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::{delay::DelayNs, spi::SpiDevice};

/// 命令 trait，提供 SPI 命令地址
pub(crate) trait Command: Copy {
    fn address(self) -> u8;
}

pub(crate) trait InternalWiAdditions<SPI, BUSY, DC, RST, DELAY>
where
    SPI: SpiDevice,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    /// 初始化 EPD 并上电
    ///
    /// 此函数已在以下函数中调用：
    /// - [new()](WaveshareDisplay::new())
    /// - [`wake_up`]
    ///
    /// 此函数会调用 [reset](WaveshareDisplay::reset)，唤醒设备时无需手动调用 reset
    async fn init(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error>;
}

/// EPD 交互函数
///
/// 包含使用 EPD 的所有公共函数
pub trait WaveshareDisplay<SPI, BUSY, DC, RST, DELAY>
where
    SPI: SpiDevice,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    /// 显示使用的颜色类型
    type DisplayColor;

    /// 从 SPI 外设创建驱动
    /// 此函数会初始化设备
    async fn new(
        spi: &mut SPI,
        busy: BUSY,
        dc: DC,
        rst: RST,
        delay: &mut DELAY,
    ) -> Result<Self, SPI::Error>
    where
        Self: Sized;

    /// 进入深度睡眠模式以节省功耗
    ///
    /// 深度睡眠模式需要硬件复位才能恢复
    async fn sleep(&mut self, spi: &mut SPI) -> Result<(), SPI::Error>;

    /// 从睡眠模式唤醒设备
    ///
    /// 必要时重新初始化设备
    async fn wake_up(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error>;

    /// 获取显示宽度
    fn width(&self) -> u32;

    /// 获取显示高度
    fn height(&self) -> u32;

    /// 传输完整帧到 EPD 的 SRAM
    async fn update_frame(&mut self, spi: &mut SPI, buffer: &[u8]) -> Result<(), SPI::Error>;

    /// 显示 SRAM 中的帧数据
    ///
    /// 此函数会等待设备空闲
    async fn display_frame(&mut self, spi: &mut SPI) -> Result<(), SPI::Error>;

    /// 合并更新和显示操作（跳过中间的忙检查）
    async fn update_and_display_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
    ) -> Result<(), SPI::Error>;

    /// 用背景颜色清空 EPD 帧缓冲区
    ///
    /// 背景颜色为COLOR::default()
    async fn clear_frame(&mut self, spi: &mut SPI) -> Result<(), SPI::Error>;

    /// 等待显示停止处理数据
    ///
    /// 调用此函数可确保帧显示完成
    async fn wait_until_idle(&mut self) -> Result<(), SPI::Error>;
}
