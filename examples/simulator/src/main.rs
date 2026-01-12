use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_graphics::mono_font::{MonoTextStyle, ascii::FONT_7X13};
use embedded_graphics::{prelude::*, primitives::*, text::*};

use epd_yrd0750ryf665f60::{
    prelude::*,
    yrd0750ryf665f60::{Display7in5, Epd7in5},
};

mod mock;
use mock::*;

#[embassy_executor::main]
async fn main_task(_spawner: Spawner) {
    println!("EPD 7.5英寸模拟器测试");

    // 创建模拟的硬件引脚
    let mut spi = MockSpi;
    let busy = MockInputPin;
    let dc = MockOutputPin;
    let rst = MockOutputPin;
    let mut delay = MockDelay;

    // 初始化EPD
    println!("正在初始化EPD...");
    let mut epd = Epd7in5::new(&mut spi, busy, dc, rst, &mut delay)
        .await
        .expect("初始化EPD失败");

    println!("EPD初始化成功，尺寸：{}x{}", epd.width(), epd.height());

    // 创建显示缓冲区
    let mut display = Display7in5::default();

    // 绘制一些图形
    println!("绘制测试图形...");

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
        "Hello EPD!",
        Point::new(10, 150),
        MonoTextStyle::new(&FONT_7X13, QuadColor::White),
    )
    .draw(&mut display)
    .unwrap();

    Text::new(
        "7.5 inch Display Test",
        Point::new(10, 170),
        MonoTextStyle::new(&FONT_7X13, QuadColor::Yellow),
    )
    .draw(&mut display)
    .unwrap();

    // 更新并显示帧（这将打开模拟器窗口）
    println!("更新并显示帧...");
    epd.update_and_display_frame(&mut spi, display.buffer())
        .await
        .expect("更新显示失败");

    println!("测试完成！模拟器窗口已打开");

    // 保持程序运行
    loop {
        Timer::after_millis(1000).await;
    }
}
