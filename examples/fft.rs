//! TODO: Docs

#![no_std]
#![no_main]

use core::convert::TryFrom;
use cortex_m::singleton;
use cortex_m_semihosting::hprintln;
use embedded_graphics::{
    geometry::Point, image::Image, pixelcolor::BinaryColor, pixelcolor::Rgb565, prelude::*,
    primitives::*, style::*,
};
use num_traits::float::{Float, FloatCore};
use panic_semihosting as _;
use rtfm::app;
use ssd1306::{prelude::*, Builder};
use stm32f1xx_hal::{
    adc,
    delay::Delay,
    dma, gpio,
    pac::{self, ADC1, SPI1},
    prelude::*,
    spi::{self, Mode, Phase, Polarity, Spi},
    timer,
    timer::{CountDownTimer, Event, Timer},
};

type Display = ssd1306::mode::graphics::GraphicsMode<
    ssd1306::interface::spi::SpiInterface<
        spi::Spi<
            SPI1,
            spi::Spi1NoRemap,
            (
                gpio::gpioa::PA5<gpio::Alternate<gpio::PushPull>>,
                gpio::gpioa::PA6<gpio::Input<gpio::Floating>>,
                gpio::gpioa::PA7<gpio::Alternate<gpio::PushPull>>,
            ),
        >,
        gpio::gpiob::PB1<gpio::Output<gpio::PushPull>>,
    >,
>;

type DebugLed = gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>;

type Mic = stm32f1xx_hal::gpio::gpioa::PA3<stm32f1xx_hal::gpio::Analog>;

// type AdcChannelThing =
//     dma::RxDma<adc::AdcPayload<gpio::gpioa::PA3<gpio::Analog>, adc::Continuous>, dma::dma1::C1>;

// type Sampler = stm32f1xx_hal::dma::CircBuffer<[u16; NUM_SAMPLES], AdcChannelThing>;
type Sampler = stm32f1xx_hal::adc::Adc<ADC1>;

const NUM_SAMPLES: usize = 32;

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        display: Display,
        timer: CountDownTimer<pac::TIM1>,
        timer2: CountDownTimer<pac::TIM2>,
        led: DebugLed,
        #[init(0)]
        count: u32,
        #[init([0; NUM_SAMPLES])]
        sample_buf: [u16; NUM_SAMPLES],
        sampler: Sampler,
        mic: Mic,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let dp = cx.device;
        let core = cx.core;

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
        let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

        // SPI1
        let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
        let miso = gpioa.pa6;
        let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

        let mut delay = Delay::new(core.SYST, clocks);

        let mut rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
        let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

        let spi = Spi::spi1(
            dp.SPI1,
            (sck, miso, mosi),
            &mut afio.mapr,
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            8.mhz(),
            clocks,
            &mut rcc.apb2,
        );

        let mut display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

        display.reset(&mut rst, &mut delay).unwrap();
        display.init().unwrap();

        Triangle::new(
            Point::new(8, 16 + 16),
            Point::new(8 + 16, 16 + 16),
            Point::new(8 + 8, 16),
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(BinaryColor::On)
                .stroke_width(1)
                .build(),
        )
        .draw(&mut display);

        display.flush().unwrap();

        let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        let mut timer = Timer::tim1(dp.TIM1, &clocks, &mut rcc.apb2).start_count_down(44.khz());
        timer.listen(timer::Event::Update);

        let mut timer2 = Timer::tim2(dp.TIM2, &clocks, &mut rcc.apb1).start_count_down(60.hz());
        timer2.listen(timer::Event::Update);

        let mut sampler = adc::Adc::adc1(dp.ADC1, &mut rcc.apb2, clocks);
        let mic = gpioa.pa3.into_analog(&mut gpioa.crl);

        // Init the static resources to use them later through RTFM
        init::LateResources {
            timer,
            timer2,
            display,
            led,
            sampler,
            mic,
        }
    }

    #[task(binds = TIM1_UP, resources = [count, timer, led, sample_buf, mic, sampler])]
    fn update(cx: update::Context) {
        use core::fmt::Write;

        let update::Resources {
            count,
            timer,
            led,
            sample_buf,
            sampler,
            mic,
            ..
        } = cx.resources;

        sample_buf.rotate_right(1);

        sample_buf[sample_buf.len() - 1] = sampler.read(mic).unwrap();

        // led.toggle();

        // Clears the update flag
        timer.clear_update_interrupt_flag();
    }

    #[task(binds = TIM2, resources = [timer2, led, display, sample_buf])]
    fn timer2(cx: timer2::Context) {
        use core::fmt::Write;

        let timer2::Resources {
            timer2,
            led,
            display,
            sample_buf,
            ..
        } = cx.resources;

        led.toggle();

        display.clear();

        let x_scale = (display.size().width / (NUM_SAMPLES as u32 - 1)) as i32;
        let y_scale = (4096 as u32 / display.size().height) as i32;

        // sample_buf
        //     .windows(2)
        //     .enumerate()
        //     .map(|(idx, parts)| {
        //         let idx = idx as i32;

        //         match parts {
        //             [start, end] => Line::new(
        //                 Point::new(idx * x_scale, *start as i32 / y_scale),
        //                 Point::new((idx + 1) * x_scale, *end as i32 / y_scale),
        //             )
        //             .into_styled(
        //                 PrimitiveStyleBuilder::new()
        //                     .stroke_color(BinaryColor::On)
        //                     .build(),
        //             )
        //             .into_iter(),
        //             _ => unreachable!(),
        //         }
        //     })
        //     .flatten()
        //     .draw(display);

        let mut normalised = normalise_samples(sample_buf.clone());

        // let mut normalised = sample_buf.clone();

        let mut spectrum = microfft::real::rfft_32(&mut normalised);

        let offs = 16;

        spectrum
            .iter()
            // norm()
            .map(|item| item.re.hypot(item.im))
            .enumerate()
            .map(|(idx, item)| {
                Line::new(
                    Point::new(idx as i32 * 2, 16),
                    Point::new(idx as i32 * 2, 16 + (item * 10.0) as i32 + 1),
                )
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_color(BinaryColor::On)
                        .build(),
                )
                .into_iter()
            })
            .flatten()
            .draw(display);

        display.flush();

        // Clears the update flag
        timer2.clear_update_interrupt_flag();
    }
};

// Mic output has a DC bias of 1.25v. Assuming 3.3v supply voltage.
const BIAS: f32 = (1.25 / 3.3);

// 2Vpp (peak to peak) so 1v above bias voltage, or 2.25v
const MAX: f32 = (2.25 / 3.3);
const MIN: f32 = (0.25 / 3.3);

// Normalise samples from -1.0 to 1.0 ready for FFT
fn normalise_samples(buf: [u16; NUM_SAMPLES]) -> [f32; NUM_SAMPLES] {
    // Half the mic input range in ADC value
    let sample_range_half = 4096.0 * (MAX - MIN) / 2.0;
    let adc_bias = BIAS * 4096.0;

    let mut out = [0.0; NUM_SAMPLES];

    for (idx, sample) in buf.iter().enumerate() {
        let sample = *sample as f32;

        // Subtract bias so sample is centered around 0
        let sample = sample as f32 - adc_bias;

        // Scale sample from -1 to 1
        let sample = sample / sample_range_half;

        out[idx] = sample
    }

    out
}
