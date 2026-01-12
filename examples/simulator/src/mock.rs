use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::digital::Wait;
use embedded_hal_async::spi::SpiDevice;

// 模拟的SPI设备
pub(crate) struct MockSpi;

impl SpiDevice for MockSpi {
    async fn transaction<'a>(
        &mut self,
        _operations: &mut [embedded_hal_async::spi::Operation<'a, u8>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl embedded_hal::spi::ErrorType for MockSpi {
    type Error = core::convert::Infallible;
}

// 模拟的输入引脚
pub(crate) struct MockInputPin;

impl embedded_hal::digital::ErrorType for MockInputPin {
    type Error = core::convert::Infallible;
}

impl InputPin for MockInputPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl Wait for MockInputPin {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// 模拟的输出引脚
pub(crate) struct MockOutputPin;

impl embedded_hal::digital::ErrorType for MockOutputPin {
    type Error = core::convert::Infallible;
}

impl OutputPin for MockOutputPin {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// 模拟的延时
pub(crate) struct MockDelay;

impl DelayNs for MockDelay {
    async fn delay_ns(&mut self, _ns: u32) {
        // 模拟延时
        Timer::after_micros(1).await;
    }

    async fn delay_us(&mut self, _us: u32) {
        // 模拟延时
        Timer::after_micros(1).await;
    }

    async fn delay_ms(&mut self, _ms: u32) {
        // 模拟延时
        Timer::after_millis(1).await;
    }
}
