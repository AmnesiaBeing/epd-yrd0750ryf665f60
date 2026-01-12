# YRD0750RYF665F60 电子墨水屏驱动

一个基于 Rust 的嵌入式电子墨水屏驱动库，专为 YRD0750RYF665F60 型号的 7.5 英寸电子墨水屏设计。

## 特性

- ✅ 基于 `embedded-hal(-async)` 构建，支持异步操作
- ✅ 支持 `embedded-graphics` 图形库
- ✅ 四色显示（黑白红黄）
- ✅ 800x480 分辨率
- ✅ 支持模拟器模式进行开发和测试

## 硬件规格

- **屏幕尺寸**: 7.5 英寸
- **分辨率**: 800x480 像素
- **颜色**: 黑白红黄四色
- **接口**: half-duplex SPI + DC + RST

## 可选特性

- `graphics`: 启用 `embedded-graphics` 支持（默认启用）
- `simulator`: 启用模拟器模式

```toml
[dependencies]
epd-yrd0750ryf665f60 = {
    version = "0.1.0",
    features = ["graphics", "simulator"]
}
```

## 快速开始

### 基本用法

```rust
use epd_yrd0750ryf665f60::yrd0750ryf665f60::EPD7in5;
use embedded_hal_async::spi::SpiDevice;

// 初始化 SPI 设备和 GPIO 引脚
let spi = SpiDevice::new(...);
let cs = ...;
let dc = ...;
let rst = ...;
let busy = ...;

// 创建显示驱动
let mut epd = EPD7in5::new(spi, cs, dc, rst, busy).await?;

// 初始化屏幕
epd.init().await?;

// 绘制内容
// ...

// 刷新显示
epd.display_frame().await?;

// 进入睡眠模式
epd.sleep().await?;
```

### 图形模式

```rust
use epd_yrd0750ryf665f60::prelude::*;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;

// 创建显示缓冲区
let mut display = Display7in5::default();

// 绘制矩形
Rectangle::new(Point::new(10, 10), Size::new(100, 100))
    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
    .draw(&mut display)?;

// 将缓冲区发送到屏幕
epd.update_frame(&display.buffer()).await?;
epd.display_frame().await?;
```

### 模拟器模式

```rust
use epd_yrd0750ryf665f60::yrd0750ryf665f60::EPD7in5;
use epd_yrd0750ryf665f60::simulator::Simulator;

// 创建模拟器
let mut epd = EPD7in5::new(Simulator::new(), ...).await?;

// 初始化并绘制
// ...

// 显示模拟器窗口
Simulator::show_window();
```

## 示例

项目包含以下示例：

- `simulator`: 模拟器模式下的基本绘制示例
- `esp32c6`: ESP32-C6 开发板的实际硬件示例

运行模拟器示例：

```bash
cd examples/simulator cargo run
cd examples/esp32c6 cargo run
```

## 项目结构

```
src/
├── color.rs          # 颜色定义
├── graphics.rs       # 图形支持
├── interface.rs      # 接口定义
├── lib.rs            # 库入口
├── traits.rs         # 特性定义
└── yrd0750ryf665f60.rs # 具体驱动实现

examples/
├── esp32c6/          # ESP32-C6 示例
└── simulator/        # 模拟器示例
```
