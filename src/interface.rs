use crate::traits::Command;

use core::marker::PhantomData;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiDevice};

/// EPD 设备连接接口
pub(crate) struct DisplayInterface<SPI, BUSY, DC, RST> {
    _spi: PhantomData<SPI>,
    /// 低电平表示忙，等待显示就绪
    busy: BUSY,
    /// 数据/命令控制引脚（高电平为数据，低电平为命令）
    dc: DC,
    /// 复位引脚
    rst: RST,
}

impl<SPI, BUSY, DC, RST> DisplayInterface<SPI, BUSY, DC, RST>
where
    SPI: SpiDevice,
    BUSY: InputPin + Wait,
    DC: OutputPin,
    RST: OutputPin,
{
    /// 创建新的 DisplayInterface
    pub fn new(busy: BUSY, dc: DC, rst: RST) -> Self {
        DisplayInterface {
            _spi: PhantomData,
            busy,
            dc,
            rst,
        }
    }

    /// 发送命令
    pub(crate) async fn cmd<T: Command>(
        &mut self,
        spi: &mut SPI,
        command: T,
    ) -> Result<(), SPI::Error> {
        let _ = self.dc.set_low();
        self.write(spi, &[command.address()]).await
    }

    /// 发送数据数组
    pub(crate) async fn data(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        let _ = self.dc.set_high();

        self.write(spi, data).await?;

        Ok(())
    }

    /// 发送命令及对应数据
    pub(crate) async fn cmd_with_data<T: Command>(
        &mut self,
        spi: &mut SPI,
        command: T,
        data: &[u8],
    ) -> Result<(), SPI::Error> {
        self.cmd(spi, command).await?;
        self.data(spi, data).await
    }

    /// 发送重复字节
    pub(crate) async fn data_x_times(
        &mut self,
        spi: &mut SPI,
        val: u8,
        repetitions: u32,
    ) -> Result<(), SPI::Error> {
        let _ = self.dc.set_high();
        for _ in 0..repetitions {
            self.write(spi, &[val]).await?;
        }
        Ok(())
    }

    /// SPI 写入辅助函数
    async fn write(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        if cfg!(target_os = "linux") {
            for data_chunk in data.chunks(4096) {
                spi.write(data_chunk).await?;
            }
            Ok(())
        } else if cfg!(feature = "simulator") {
            Ok(())
        } else {
            spi.write(data).await
        }
    }

    /// 等待设备空闲（busy == HIGH）
    pub(crate) async fn wait_until_idle(&mut self, is_busy_low: bool) {
        match is_busy_low {
            true => {
                let _ = self.busy.wait_for_high().await;
            }
            false => {
                let _ = self.busy.wait_for_low().await;
            }
        }
    }

    /// 复位设备
    ///
    /// 复位引脚保持低电平的时间对不同设备很重要
    pub(crate) async fn reset<D: DelayNs>(
        &mut self,
        delay: &mut D,
        initial_delay: u32,
        duration: u32,
    ) {
        let _ = self.rst.set_high();
        delay.delay_us(initial_delay).await;
        let _ = self.rst.set_low();
        delay.delay_us(duration).await;
        let _ = self.rst.set_high();
        delay.delay_us(200_000).await;
    }
}
