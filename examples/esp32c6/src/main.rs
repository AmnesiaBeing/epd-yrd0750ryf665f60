#![no_std]
#![no_main]

use embedded_graphics::mono_font::*;
use embedded_graphics::{prelude::*, primitives::*, text::*};
use embedded_hal_async::delay::DelayNs;
use epd_yrd0750ryf665f60::{prelude::*, yrd0750ryf665f60::Epd7in5};
use esp_backtrace as _;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::spi::master::Spi;
use esp_sync::RawMutex;
use rtt_target::rprintln;
use rtt_target::rtt_init_print;

esp_bootloader_esp_idf::esp_app_desc!();

extern crate alloc;

#[esp_rtos::main]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    esp_alloc::heap_allocator!(size: 64 * 1024);
    rtt_init_print!();
    esp_println::logger::init_logger_from_env();
    // 获取ESP32C6的外设
    let mut peripherals =
        esp_hal::init(esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()));
    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);

    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    rprintln!("ESP32C6 EPD 7.5英寸测试");

    // 配置SPI引脚
    // 配置 SPI 引脚
    let sck = peripherals.GPIO22.reborrow();
    let sda = peripherals.GPIO23.reborrow();
    let cs: Output<'_> = Output::new(
        peripherals.GPIO21.reborrow(),
        Level::High,
        OutputConfig::default(),
    );

    // 配置 EPD 控制引脚
    let busy = Input::new(peripherals.GPIO18.reborrow(), InputConfig::default());
    let dc = Output::new(
        peripherals.GPIO20.reborrow(),
        Level::High,
        OutputConfig::default(),
    );
    let rst = Output::new(
        peripherals.GPIO19.reborrow(),
        Level::High,
        OutputConfig::default(),
    );

    // 获取 SPI2 实例
    let spi2 = peripherals.SPI2.reborrow();

    // 创建 SPI 总线
    let spi_bus = Spi::new(
        spi2,
        esp_hal::spi::master::Config::default()
            .with_frequency(esp_hal::time::Rate::from_mhz(10))
            .with_mode(esp_hal::spi::Mode::_0),
    )
    .unwrap()
    .with_sck(sck)
    .with_sio0(sda)
    .into_async();

    let mut delay = embassy_time::Delay;
    let spi_bus_mutex = embassy_sync::mutex::Mutex::<RawMutex, _>::new(spi_bus);
    let mut spi_device =
        embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice::new(&spi_bus_mutex, cs);

    rprintln!("SPI初始化完成");

    // 初始化EPD
    rprintln!("正在初始化EPD...");
    let mut epd = Epd7in5::new(&mut spi_device, busy, dc, rst, &mut delay)
        .await
        .expect("EPD初始化失败");

    rprintln!("EPD初始化成功，尺寸：{}x{}", epd.width(), epd.height());

    // 创建显示缓冲区
    let mut display = epd_yrd0750ryf665f60::yrd0750ryf665f60::Display7in5::default();

    // 绘制一些图形
    rprintln!("绘制测试图形...");

    // 绘制矩形
    Rectangle::new(Point::new(10, 10), Size::new(100, 100))
        .into_styled(PrimitiveStyle::with_fill(QuadColor::Black))
        .draw(&mut display)
        .unwrap();

    // 绘制圆
    Circle::new(Point::new(200, 60), 50)
        .into_styled(PrimitiveStyle::with_fill(QuadColor::Red))
        .draw(&mut display)
        .unwrap();

    // 绘制文本
    Text::new(
        "Hello ESP32C6!",
        Point::new(10, 150),
        MonoTextStyle::new(&ascii::FONT_7X13, QuadColor::White),
    )
    .draw(&mut display)
    .unwrap();

    Text::new(
        "7.5 inch EPD Test",
        Point::new(10, 170),
        MonoTextStyle::new(&ascii::FONT_7X13, QuadColor::Black),
    )
    .draw(&mut display)
    .unwrap();

    Text::new(
        "Using esp-hal",
        Point::new(10, 190),
        MonoTextStyle::new(&ascii::FONT_7X13, QuadColor::Black),
    )
    .draw(&mut display)
    .unwrap();
    // 更新并显示帧
    rprintln!("更新显示...");
    let _ = epd.wake_up(&mut spi_device, &mut delay).await;
    epd.update_and_display_frame(&mut spi_device, display.buffer())
        .await
        .expect("更新显示失败");

    rprintln!("显示更新完成");

    // 进入低功耗模式
    rprintln!("测试完成，进入睡眠模式");
    epd.sleep(&mut spi_device).await.expect("进入睡眠模式失败");

    // 无限循环
    loop {
        delay.delay_ms(1000u32).await;
    }
}
