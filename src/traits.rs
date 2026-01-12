#![allow(async_fn_in_trait)]

use core::marker::Sized;
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use embedded_hal_async::spi::SpiDevice;

/// 命令 trait，提供 SPI 命令地址
pub(crate) trait Command: Copy {
    fn address(self) -> u8;
}

/// 显示刷新查找表类型
#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum RefreshLut {
    /// 完整刷新查找表
    #[default]
    Full,
    /// 快速刷新查找表（可能导致重影）
    Quick,
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
    ///
    /// `delay_us`: 空闲循环休眠的微秒数，0 表示忙等待，None 表示使用默认值
    ///
    /// 此函数会初始化设备
    async fn new(
        spi: &mut SPI,
        busy: BUSY,
        dc: DC,
        rst: RST,
        delay: &mut DELAY,
        delay_us: Option<u32>,
    ) -> Result<Self, SPI::Error>
    where
        Self: Sized;

    /// 进入深度睡眠模式以节省功耗
    ///
    /// 深度睡眠模式需要硬件复位才能恢复
    async fn sleep(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error>;

    /// 从睡眠模式唤醒设备
    ///
    /// 必要时重新初始化设备
    async fn wake_up(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error>;

    /// 设置背景颜色
    fn set_background_color(&mut self, color: Self::DisplayColor);

    /// 获取当前背景颜色
    fn background_color(&self) -> &Self::DisplayColor;

    /// 获取显示宽度
    fn width(&self) -> u32;

    /// 获取显示高度
    fn height(&self) -> u32;

    /// 传输完整帧到 EPD 的 SRAM
    async fn update_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY,
    ) -> Result<(), SPI::Error>;

    /// 显示 SRAM 中的帧数据
    ///
    /// 此函数会等待设备空闲
    async fn display_frame(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error>;

    /// 合并更新和显示操作（跳过中间的忙检查）
    async fn update_and_display_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY,
    ) -> Result<(), SPI::Error>;

    /// 用背景颜色清空 EPD 帧缓冲区
    ///
    /// 背景颜色可通过 [`WaveshareDisplay::set_background_color`] 修改
    async fn clear_frame(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error>;

    /// 设置刷新查找表
    ///
    /// 用于部分刷新等场景
    ///
    /// 一定次数的快速刷新后需要完整刷新！
    ///
    /// 警告：快速刷新可能导致重影问题，尤其是 4.2 英寸显示屏
    ///
    /// None 表示重新加载当前 LUT
    fn set_lut(
        &mut self,
        spi: &mut SPI,
        delay: &mut DELAY,
        refresh_rate: Option<RefreshLut>,
    ) -> Result<(), SPI::Error>;

    /// 等待显示停止处理数据
    ///
    /// 调用此函数可确保帧显示完成
    async fn wait_until_idle(&mut self, spi: &mut SPI) -> Result<(), SPI::Error>;
}

/// 快速刷新支持
///
/// 使用快速刷新查找表时，显示需要分别接收旧帧和新帧数据
///
/// 快速刷新不需要分别发送旧帧和新帧数据
pub trait QuickRefresh<SPI, BUSY, DC, RST, DELAY>
where
    SPI: SpiDevice,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    /// 更新旧帧
    fn update_old_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY,
    ) -> Result<(), SPI::Error>;

    /// 更新旧帧的部分区域
    #[allow(clippy::too_many_arguments)]
    fn update_partial_old_frame(
        &mut self,
        spi: &mut SPI,
        delay: &mut DELAY,
        buffer: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), SPI::Error>;

    /// 更新部分帧缓冲区
    fn clear_partial_frame(
        &mut self,
        spi: &mut SPI,
        delay: &mut DELAY,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), SPI::Error>;
}
