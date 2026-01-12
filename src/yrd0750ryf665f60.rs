//! YRD0750RYF665F60 电子墨水屏驱动

use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::{digital::Wait, spi::SpiDevice};

#[cfg(feature = "simulator")]
use crate::simulator::Window;

use crate::color::QuadColor;
use crate::interface::DisplayInterface;
use crate::traits::{InternalWiAdditions, RefreshLut, WaveshareDisplay};

use crate::buffer_len;
use crate::traits;

#[cfg(feature = "simulator")]
use embedded_graphics_core::prelude::*;

#[cfg(feature = "simulator")]
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

/// 7.5 英寸 EPD 完整缓冲区
#[cfg(feature = "graphics")]
pub type Display7in5 = crate::graphics::Display<
    WIDTH,
    HEIGHT,
    { buffer_len(WIDTH as usize, HEIGHT as usize * 2) },
    QuadColor,
>;

/// 显示宽度
pub const WIDTH: u32 = 800;
/// 显示高度
pub const HEIGHT: u32 = 480;
/// 默认背景颜色
pub const DEFAULT_BACKGROUND_COLOR: QuadColor = QuadColor::White;

/// 缓冲区字节数
const NUM_DISPLAY_BITS: usize = WIDTH as usize / 4 * HEIGHT as usize;
const IS_BUSY_LOW: bool = true;
const SINGLE_BYTE_WRITE: bool = false;

/// EPD 命令
#[derive(Copy, Clone)]
#[allow(unused)]
pub(crate) enum Command {
    /// 面板设置
    PanelSetting = 0x00,
    /// 电源设置
    PowerSetting = 0x01,
    /// 关闭电源
    PowerOff = 0x02,
    /// 开启电源
    PowerOn = 0x04,
    /// 启动数据传输
    BoosterSoftStart = 0x06,
    /// 深度睡眠
    DeepSleep = 0x07,
    /// 开始传输数据（黑白/旧数据）
    DataStartTransmission1 = 0x10,
    /// 停止数据传输
    DataStop = 0x11,
    /// 显示刷新
    DisplayRefresh = 0x12,
    /// PLL 控制
    PllControl = 0x30,
    /// 温度传感器
    TemperatureSensor = 0x40,
    /// 温度校准
    TemperatureCalibration = 0x41,
    /// 温度传感器写入
    TemperatureSensorWrite = 0x42,
    /// 温度传感器读取
    TemperatureSensorRead = 0x43,
    /// 神秘命令 1
    MisteryCommand1 = 0x4D,
    /// VCOM 和数据间隔设置
    VcomAndDataIntervalSetting = 0x50,
    /// 低功耗检测
    LowPowerDetection = 0x51,
    /// TCON 分辨率
    TconResolution = 0x61,
    /// SPI Flash 控制
    SpiFlashControl = 0x65,
    /// 版本
    Revision = 0x70,
    /// 自动测量 VCOM
    AutoMeasurementVcom = 0x80,
    /// 读取 VCOM 值
    ReadVcomValue = 0x81,
    /// VCOM DC 设置
    VcmDcSetting = 0x82,
    /// 部分窗口
    PartialWindow = 0x83,
    /// 编程模式
    ProgramMode = 0x90,
    /// 激活编程
    ActiveProgram = 0x91,
    /// 读取 MTP 数据
    ReadMTPData = 0x92,
    /// MTP 编程配置
    MtpProgramConfig = 0xA2,
    /// 级联设置
    CascadeSetting = 0xE0,
    /// 省电设置
    PowerSavingSetting = 0xE3,
    /// LVD 电压选择
    LvdVoltageSelect = 0xE4,
    /// 神秘命令 2
    MisteryCommand2 = 0xE9,
}

impl traits::Command for Command {
    fn address(self) -> u8 {
        self as u8
    }
}

/// Epd7in5 (yrd0750ryf665f60) 驱动
pub struct Epd7in5<SPI, BUSY, DC, RST, DELAY> {
    interface: DisplayInterface<SPI, BUSY, DC, RST, SINGLE_BYTE_WRITE>,
    color: QuadColor,
    _delay: PhantomData<DELAY>,
    #[cfg(feature = "simulator")]
    simulator_window: Option<core::cell::RefCell<Window>>,
    #[cfg(feature = "simulator")]
    simulator_display: SimulatorDisplay<QuadColor>,
}

impl<SPI, BUSY, DC, RST, DELAY> InternalWiAdditions<SPI, BUSY, DC, RST, DELAY>
    for Epd7in5<SPI, BUSY, DC, RST, DELAY>
where
    SPI: SpiDevice,
    BUSY: InputPin + Wait,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    async fn init(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.interface.reset(delay, 20_000, 20_000).await;
        self.wait_until_idle(spi).await?;
        self.interface
            .cmd_with_data(spi, Command::MisteryCommand1, &[0x78])
            .await?;
        self.interface
            .cmd_with_data(spi, Command::PanelSetting, &[0x2F, 0x29])
            .await?;
        self.interface
            .cmd_with_data(spi, Command::VcomAndDataIntervalSetting, &[0x37])
            .await?;
        self.interface
            .cmd_with_data(spi, Command::SpiFlashControl, &[0x00, 0x00, 0x00, 0x00])
            .await?;
        self.interface
            .cmd_with_data(spi, Command::PowerSavingSetting, &[0x88])
            .await?;
        self.interface
            .cmd_with_data(spi, Command::MisteryCommand2, &[0x01])
            .await?;
        self.interface
            .cmd_with_data(spi, Command::PllControl, &[0x08])
            .await?;
        self.interface.cmd(spi, Command::PowerOn).await?;
        self.wait_until_idle(spi).await?;

        Ok(())
    }
}

impl<SPI, BUSY, DC, RST, DELAY> WaveshareDisplay<SPI, BUSY, DC, RST, DELAY>
    for Epd7in5<SPI, BUSY, DC, RST, DELAY>
where
    SPI: SpiDevice,
    BUSY: InputPin + Wait,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    type DisplayColor = QuadColor;

    async fn new(
        _spi: &mut SPI,
        busy: BUSY,
        dc: DC,
        rst: RST,
        _delay: &mut DELAY,
        _delay_us: Option<u32>,
    ) -> Result<Self, SPI::Error> {
        let interface = DisplayInterface::new(busy, dc, rst);
        let color = DEFAULT_BACKGROUND_COLOR;

        let epd = Epd7in5 {
            interface,
            color,
            _delay: PhantomData,
            #[cfg(feature = "simulator")]
            simulator_window: None,
            #[cfg(feature = "simulator")]
            simulator_display: SimulatorDisplay::with_default_color(
                Size::new(WIDTH, HEIGHT),
                QuadColor::default(),
            ),
        };

        Ok(epd)
    }

    async fn wake_up(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.init(spi, delay).await
    }

    async fn sleep(&mut self, spi: &mut SPI, _delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.wait_until_idle(spi).await?;
        self.interface
            .cmd_with_data(spi, Command::PowerOff, &[0x00])
            .await?;
        self.wait_until_idle(spi).await?;
        self.interface
            .cmd_with_data(spi, Command::DeepSleep, &[0xA5])
            .await?;
        Ok(())
    }

    #[cfg(feature = "simulator")]
    fn update_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        _delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
        debug_assert_eq!(
            buffer.len(),
            NUM_DISPLAY_BITS,
            "EPD buffer length mismatch: expected {}, got {}",
            NUM_DISPLAY_BITS,
            buffer.len()
        );

        let color_iter = buffer.iter().flat_map(|byte| {
            [0, 2, 4, 6].iter().map(move |&shift| {
                let pixel_bits = (*byte >> shift) & 0x03;
                QuadColor::from_bits(pixel_bits)
            })
        });

        let pixels = color_iter.enumerate().map(|(i, color)| {
            let x = (i % WIDTH as usize) as i32;
            let y = (i / WIDTH as usize) as i32;
            Pixel(Point::new(x, y), color)
        });

        self.simulator_display
            .draw_iter(pixels)
            .expect("Failed to draw frame to EPD simulator");

        Ok(())
    }

    #[cfg(not(feature = "simulator"))]
    async fn update_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        _delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
        self.wait_until_idle(spi).await?;
        self.interface
            .cmd_with_data(
                spi,
                Command::DataStartTransmission1,
                &buffer[..NUM_DISPLAY_BITS],
            )
            .await?;
        Ok(())
    }

    async fn display_frame(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        if cfg!(feature = "simulator") {
            let _ = self.init(spi, delay).await;
            #[cfg(feature = "simulator")]
            {
                if self.simulator_window.is_none() {
                    self.simulator_window = Some(core::cell::RefCell::new(Window::new(
                        &format!("EPD Simulator {}x{}", WIDTH, HEIGHT),
                        &OutputSettingsBuilder::new().scale(1).build(),
                    )));
                }
                if let Some(window) = &self.simulator_window {
                    window.borrow_mut().update(&self.simulator_display);
                }
            }
            Ok(())
        } else {
            self.interface
                .cmd_with_data(spi, Command::DisplayRefresh, &[0x00])
                .await?;
            delay.delay_us(500);
            self.wait_until_idle(spi).await?;
            Ok(())
        }
    }

    async fn update_and_display_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
        self.update_frame(spi, buffer, delay).await?;
        self.interface.cmd(spi, Command::PowerOn).await?;
        self.display_frame(spi, delay).await?;
        Ok(())
    }

    async fn clear_frame(&mut self, spi: &mut SPI, _delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.wait_until_idle(spi).await?;

        self.interface
            .cmd(spi, Command::DataStartTransmission1)
            .await?;
        self.interface
            .data_x_times(spi, QuadColor::default_color_byte(), WIDTH * HEIGHT / 4)
            .await?;

        self.interface.cmd(spi, Command::DataStop).await?;

        self.interface.cmd(spi, Command::DisplayRefresh).await?;

        Ok(())
    }

    fn set_background_color(&mut self, color: Self::DisplayColor) {
        self.color = color;
    }

    fn background_color(&self) -> &Self::DisplayColor {
        &self.color
    }

    fn width(&self) -> u32 {
        WIDTH
    }

    fn height(&self) -> u32 {
        HEIGHT
    }

    fn set_lut(
        &mut self,
        _spi: &mut SPI,
        _delay: &mut DELAY,
        _refresh_rate: Option<RefreshLut>,
    ) -> Result<(), SPI::Error> {
        unimplemented!();
    }

    async fn wait_until_idle(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        let _ = spi;
        self.interface.wait_until_idle(IS_BUSY_LOW).await;
        Ok(())
    }
}
