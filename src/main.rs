//! simple program to blink an LED at a rate that changes based on ADC
//! readings from a moisture sensor.


#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use stm32f1xx_hal::{
    adc,
    gpio,
    prelude::*,
    timer::{Event, Timer, CountDownTimer},
    device::Interrupt,
};
use stm32f1::stm32f103::ADC1;
use stm32f1::stm32f103::TIM2;
use stm32f1::stm32f103::TIM3;
use panic_semihosting as _;


const LED_BASE_RATE: u32 = 2;  // 2hz
const ADC_RATE: u32 = 2;  // 2hz
const MOISTURE_SCALING_FACTOR: u32 = 200;  // scale by factor of 200


#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        #[init(20)]
        led_blink_rate: u32,
        moisture_sensor: gpio::gpioa::PA0<gpio::Analog>,
        internal_led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
        adc1: adc::Adc<ADC1>,
        led_rate_timer: CountDownTimer<TIM2>,
        sensor_rate_timer: CountDownTimer<TIM3>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;
        // Device specific peripherals
        let device: stm32f1xx_hal::pac::Peripherals = cx.device;

        rtic::pend(Interrupt::TIM2);
        rtic::pend(Interrupt::TIM3);

        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
 
        // Configure ADC
        let clocks = rcc
            .cfgr
            .adcclk(2.mhz())
            .sysclk(8.mhz())
            .pclk1(8.mhz())
            .freeze(&mut flash.acr);
        let adc1 = adc::Adc::adc1(device.ADC1, &mut rcc.apb2, clocks);

        // Setup GPIOs
        let mut gpioa = device.GPIOA.split(&mut rcc.apb2);
        let moisture_sensor = gpioa.pa0.into_analog(&mut gpioa.crl);
        let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
        let internal_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        
        let mut led_rate_timer = Timer::tim2(device.TIM2, &clocks, &mut rcc.apb1)
            .start_count_down(LED_BASE_RATE.hz());
        led_rate_timer.listen(Event::Update);
        let mut sensor_rate_timer = Timer::tim3(device.TIM3, &clocks, &mut rcc.apb1)
            .start_count_down(ADC_RATE.hz());
        sensor_rate_timer.listen(Event::Update);

        init::LateResources { moisture_sensor,
                              internal_led,
                              adc1,
                              led_rate_timer,
                              sensor_rate_timer,
                            }
    }

    #[task(binds = TIM2, priority = 1, resources = [internal_led,
                                                    led_rate_timer,
                                                    led_blink_rate,
                                                   ])]
    fn toggle_led(mut cx: toggle_led::Context) {

        // toggle
        cx.resources.internal_led.toggle().unwrap();

        // store new rate to avoid a double-borrow
        let mut new_rate: u32 = LED_BASE_RATE;
        cx.resources.led_blink_rate.lock(|led_blink_rate| {
          new_rate = *led_blink_rate;
        });

        cx.resources.led_rate_timer.clear_update_interrupt_flag();
        cx.resources.led_rate_timer.start(new_rate.hz());
    }
    
    #[task(binds = TIM3, priority = 2, resources = [moisture_sensor,
                                                    adc1,
                                                    sensor_rate_timer,
                                                    led_blink_rate,
                                                   ])]
    fn read_moisture(cx: read_moisture::Context) {
       
        // read new moisture level 
        let moisture: u16 = cx.resources.adc1.read(
                                &mut *cx.resources.moisture_sensor
                            ).unwrap();

        // compute new rate
        let new_rate: u32 = LED_BASE_RATE + (moisture as u32 / MOISTURE_SCALING_FACTOR);
        *cx.resources.led_blink_rate = new_rate;

        cx.resources.sensor_rate_timer.clear_update_interrupt_flag();
        cx.resources.sensor_rate_timer.start(ADC_RATE.hz());
    }

};
