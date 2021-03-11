#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_STREAM0();
    fn DMA1_STREAM1();
    fn DMA1_STREAM2();
    fn DMA1_STREAM3();
    fn DMA1_STREAM4();
    fn DMA1_STREAM5();
    fn DMA1_STREAM6();
    fn ADC();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn PWM1_UP();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn DMA1_STREAM7();
    fn TIM5();
    fn TIM6_DAC1();
    fn DMA2_STREAM0();
    fn DMA2_STREAM1();
    fn DMA2_STREAM2();
    fn DMA2_STREAM3();
    fn DMA2_STREAM4();
    fn EXTI19();
    fn DMA2_STREAM5();
    fn DMA2_STREAM6();
    fn DMA2_STREAM7();
    fn USART6();
    fn EXTI20();
    fn RNG();
    fn FPU();
    fn SPI5();
    fn I2C4_EV();
    fn I2C4_ER();
    fn LPTIM1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 98] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_STREAM0,
    },
    Vector {
        _handler: DMA1_STREAM1,
    },
    Vector {
        _handler: DMA1_STREAM2,
    },
    Vector {
        _handler: DMA1_STREAM3,
    },
    Vector {
        _handler: DMA1_STREAM4,
    },
    Vector {
        _handler: DMA1_STREAM5,
    },
    Vector {
        _handler: DMA1_STREAM6,
    },
    Vector { _handler: ADC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM9,
    },
    Vector { _handler: PWM1_UP },
    Vector {
        _handler: TIM1_TRG_COM_TIM11,
    },
    Vector { _handler: TIM1_CC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _reserved: 0 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA1_STREAM7,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: TIM6_DAC1,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA2_STREAM0,
    },
    Vector {
        _handler: DMA2_STREAM1,
    },
    Vector {
        _handler: DMA2_STREAM2,
    },
    Vector {
        _handler: DMA2_STREAM3,
    },
    Vector {
        _handler: DMA2_STREAM4,
    },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI19 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA2_STREAM5,
    },
    Vector {
        _handler: DMA2_STREAM6,
    },
    Vector {
        _handler: DMA2_STREAM7,
    },
    Vector { _handler: USART6 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI20 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RNG },
    Vector { _handler: FPU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: LPTIM1 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - Window watchdog interrupt"]
    WWDG = 0,
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD = 1,
    #[doc = "2 - Tamper and TimeStamp interrupts through the EXTI line"]
    TAMP_STAMP = 2,
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"]
    RTC_WKUP = 3,
    #[doc = "4 - FLASH global interrupt"]
    FLASH = 4,
    #[doc = "5 - RCC global interrupt"]
    RCC = 5,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0 = 6,
    #[doc = "7 - EXTI Line1 interrupt"]
    EXTI1 = 7,
    #[doc = "8 - EXTI Line2 interrupt"]
    EXTI2 = 8,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3 = 9,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4 = 10,
    #[doc = "11 - DMA1 stream0 global interrupt"]
    DMA1_STREAM0 = 11,
    #[doc = "12 - DMA1 stream1 global interrupt"]
    DMA1_STREAM1 = 12,
    #[doc = "13 - DMA1 stream2 global interrupt"]
    DMA1_STREAM2 = 13,
    #[doc = "14 - DMA1 stream3 global interrupt"]
    DMA1_STREAM3 = 14,
    #[doc = "15 - DMA1 stream4 global interrupt"]
    DMA1_STREAM4 = 15,
    #[doc = "16 - DMA1 stream5 global interrupt"]
    DMA1_STREAM5 = 16,
    #[doc = "17 - DMA1 stream6 global interrupt"]
    DMA1_STREAM6 = 17,
    #[doc = "18 - ADC1 global interrupt"]
    ADC = 18,
    #[doc = "23 - EXTI Line\\[9:5\\]
interrupts"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"]
    TIM1_BRK_TIM9 = 24,
    #[doc = "25 - Timer1 Update interrupt"]
    PWM1_UP = 25,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
    TIM1_TRG_COM_TIM11 = 26,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC = 27,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1 = 35,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2 = 36,
    #[doc = "37 - USART1 global interrupt"]
    USART1 = 37,
    #[doc = "38 - USART2 global interrupt"]
    USART2 = 38,
    #[doc = "40 - EXTI Line\\[15:10\\]
interrupts"]
    EXTI15_10 = 40,
    #[doc = "41 - RTC Alarms (A and B) through EXTI line interrupt"]
    RTC_ALARM = 41,
    #[doc = "47 - DMA1 stream7 global interrupt"]
    DMA1_STREAM7 = 47,
    #[doc = "50 - TIM5 global interrupt"]
    TIM5 = 50,
    #[doc = "54 - Timer6 and DAC1 global interrupt"]
    TIM6_DAC1 = 54,
    #[doc = "56 - DMA2 stream0 global interrupt"]
    DMA2_STREAM0 = 56,
    #[doc = "57 - DMA2 stream1 global interrupt"]
    DMA2_STREAM1 = 57,
    #[doc = "58 - DMA2 stream2 global interrupt"]
    DMA2_STREAM2 = 58,
    #[doc = "59 - DMA2 stream3 global interrupt"]
    DMA2_STREAM3 = 59,
    #[doc = "60 - DMA2 stream4 global interrupt"]
    DMA2_STREAM4 = 60,
    #[doc = "62 - EXTI Line 19 interrupt"]
    EXTI19 = 62,
    #[doc = "68 - DMA2 stream5 global interrupt"]
    DMA2_STREAM5 = 68,
    #[doc = "69 - DMA2 stream6 global interrupt"]
    DMA2_STREAM6 = 69,
    #[doc = "70 - DMA2 stream7 global interrupt"]
    DMA2_STREAM7 = 70,
    #[doc = "71 - USART6 global interrupt"]
    USART6 = 71,
    #[doc = "76 - EXTI Line 20 interrupt"]
    EXTI20 = 76,
    #[doc = "80 - RNG global interrupt"]
    RNG = 80,
    #[doc = "81 - Floating point unit"]
    FPU = 81,
    #[doc = "85 - SPI 5 global interrupt"]
    SPI5 = 85,
    #[doc = "95 - I2C4 event interrupt"]
    I2C4_EV = 95,
    #[doc = "96 - I2C2 error interrupt"]
    I2C4_ER = 96,
    #[doc = "97 - LPTimer global interrupt"]
    LPTIM1 = 97,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "ADC common registers"]
pub struct ADC_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC_COMMON {}
impl ADC_COMMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc_common::RegisterBlock {
        0x4001_2300 as *const _
    }
}
impl Deref for ADC_COMMON {
    type Target = adc_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC_COMMON::ptr() }
    }
}
#[doc = "ADC common registers"]
pub mod adc_common;
#[doc = "Analog-to-digital converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-digital converter"]
pub mod adc1;
#[doc = "Cryptographic processor"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub mod crc;
#[doc = "Debug support"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbgmcu;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x4001_3c00 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "FLASH"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4002_3c00 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "FLASH"]
pub mod flash;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x4002_3800 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x4001_3800 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "Advanced-timers"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM11 {}
impl TIM11 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim11::RegisterBlock {
        0x4001_4800 as *const _
    }
}
impl Deref for TIM11 {
    type Target = tim11::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM11::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim11;
#[doc = "General-purpose-timers"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim5::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim5;
#[doc = "General purpose timers"]
pub struct TIM9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM9 {}
impl TIM9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim9::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TIM9 {
    type Target = tim9::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM9::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim9;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_1400 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART6::ptr() }
    }
}
#[doc = "Window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "DMA controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_6400 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "DMA controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_6000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma1;
#[doc = "General-purpose I/Os"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioh::RegisterBlock {
        0x4002_1c00 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioh;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioh::RegisterBlock {
        0x4002_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "Inter-integrated circuit"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "Serial peripheral interface"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for SPI5 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI5::ptr() }
    }
}
#[doc = "Basic timers"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "Basic timers"]
pub mod tim6;
#[doc = "Random number generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random number generator"]
pub mod rng;
#[doc = "Digital-to-analog converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4000_7400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub mod dac;
#[doc = "Low power timer"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_2400 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM1::ptr() }
    }
}
#[doc = "Low power timer"]
pub mod lptim1;
#[doc = "Inter-integrated circuit"]
pub struct FMPI2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMPI2C1 {}
impl FMPI2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmpi2c1::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for FMPI2C1 {
    type Target = fmpi2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMPI2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod fmpi2c1;
#[doc = "Floting point unit"]
pub struct FPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU {}
impl FPU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fpu::RegisterBlock {
        0xe000_ef34 as *const _
    }
}
impl Deref for FPU {
    type Target = fpu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPU::ptr() }
    }
}
#[doc = "Floting point unit"]
pub mod fpu;
#[doc = "SysTick timer"]
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stk::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STK::ptr() }
    }
}
#[doc = "SysTick timer"]
pub mod stk;
#[doc = "Nested vectored interrupt controller"]
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVIC_STIR {}
impl NVIC_STIR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvic_stir::RegisterBlock {
        0xe000_ef00 as *const _
    }
}
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
#[doc = "Nested vectored interrupt controller"]
pub mod nvic_stir;
#[doc = "Floating point unit CPACR"]
pub struct FPU_CPACR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_CPACR {}
impl FPU_CPACR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fpu_cpacr::RegisterBlock {
        0xe000_ed88 as *const _
    }
}
impl Deref for FPU_CPACR {
    type Target = fpu_cpacr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPU_CPACR::ptr() }
    }
}
#[doc = "Floating point unit CPACR"]
pub mod fpu_cpacr;
#[doc = "System control block ACTLR"]
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb_actrl::RegisterBlock {
        0xe000_e008 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
#[doc = "System control block ACTLR"]
pub mod scb_actrl;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC_COMMON"]
    pub ADC_COMMON: ADC_COMMON,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM11"]
    pub TIM11: TIM11,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM9"]
    pub TIM9: TIM9,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI5"]
    pub SPI5: SPI5,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "FMPI2C1"]
    pub FMPI2C1: FMPI2C1,
    #[doc = "FPU"]
    pub FPU: FPU,
    #[doc = "STK"]
    pub STK: STK,
    #[doc = "NVIC_STIR"]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "FPU_CPACR"]
    pub FPU_CPACR: FPU_CPACR,
    #[doc = "SCB_ACTRL"]
    pub SCB_ACTRL: SCB_ACTRL,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC_COMMON: ADC_COMMON {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            TIM11: TIM11 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM9: TIM9 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART6: USART6 {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI5: SPI5 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            FMPI2C1: FMPI2C1 {
                _marker: PhantomData,
            },
            FPU: FPU {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            FPU_CPACR: FPU_CPACR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
        }
    }
}
