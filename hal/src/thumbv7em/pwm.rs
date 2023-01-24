#![allow(non_snake_case)]

use crate::clock;
use crate::ehal::{Pwm, PwmPin};
use crate::gpio::*;
use crate::gpio::{AlternateE, AnyPin, Pin};
use crate::time::Hertz;
use crate::timer_params::TimerParams;

mod flags;
pub use flags::*;

use crate::pac::{MCLK, TC0, TC1, TC2, TC3, TCC0, TCC1, TCC2};
#[cfg(feature = "min-samd51j")]
use crate::pac::{TC4, TC5, TCC3, TCC4};
#[cfg(feature = "min-samd51n")]
use crate::pac::{TC6, TC7};

// Timer/Counter (TCx)

/// This is a major syntax hack.
///
/// The previous Pinout types were enums that took specific v1::Pin types. As a
/// result, there was no way to make that implementation simultaneously
/// compatible with both v1::Pin and Pin.
///
/// BUT, the enum variant syntax is the same as the namespaced function syntax.
/// I converted the enums to structs, and I created constructor methods with the
/// same names as the previous enum variants. By constructing Pinout types with
/// functions rather than enum variants, you can make it generic over v1::Pin
/// and Pin types.
///
/// This is (mostly) backwards compatible with the current syntax, and all the
/// existing calls compile. The only incompatible change is the requirement of
/// type parameters on the Pwm types. Most of the type, the type parameters can
/// be inferred, so this is mostly backwards compatible as well. But there were
/// one or two instances where I had to add explicit type parameters to existing
/// BSP code.
macro_rules! impl_tc_pinout {
    (
        $Type:ident: [ $(
            $( #[$attr:meta] )?
            ($func: ident, $Id: ident)
        ),+ ]
    ) => {
        pub struct $Type<I: PinId> {
            _pin: Pin<I, AlternateE>,
        }

        $(
            $( #[$attr] )?
            impl $Type<$Id> {
                #[inline]
                pub fn $func(pin: impl AnyPin<Id = $Id>) -> Self {
                    let _pin = pin.into().into_alternate();
                    Self { _pin }
                }
            }
        )+
    };
}

impl_tc_pinout!(TC0Pinout: [
    (Pa4, PA04),
    (Pa5, PA05),
    (Pa9, PA09),
    #[cfg(feature = "min-samd51j")]
    (Pb31, PB31)
]);
impl_tc_pinout!(TC1Pinout: [(Pa7, PA07), (Pa11, PA11)]);
impl_tc_pinout!(TC2Pinout: [(Pa1, PA01), (Pa13, PA13), (Pa17, PA17)]);
impl_tc_pinout!(TC3Pinout: [(Pa15, PA15), (Pa19, PA19)]);
#[cfg(feature = "min-samd51j")]
impl_tc_pinout!(TC4Pinout: [(Pa23, PA23), (Pb09, PB09), (Pb13, PB13)]);
#[cfg(feature = "min-samd51j")]
impl_tc_pinout!(TC5Pinout: [(Pa25, PA25), (Pb11, PB11), (Pb15, PB15)]);
#[cfg(feature = "min-samd51n")]
impl_tc_pinout!(TC6Pinout: [(Pb3, PB03), (Pb17, PB17), (Pa31, PA31)]);
#[cfg(feature = "min-samd51n")]
impl_tc_pinout!(TC7Pinout: [(Pa21, PA21), (Pb23, PB23), (Pb1, PB01)]);

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE<I: PinId> {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: $TC,
    #[allow(dead_code)]
    pinout: $pinout<I>,
}

impl<I: PinId> $TYPE<I> {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tc: $TC,
        pinout: $pinout<I>,
        mclk: &mut MCLK,
    ) -> Self {
        let freq = freq.into();
        {
            let count = tc.count16();
            let params = TimerParams::new(freq, clock.freq().0);
            mclk.$apmask.modify(|_, w| w.$apbits().set_bit());
            count.ctrla.write(|w| w.swrst().set_bit());
            while count.ctrla.read().bits() & 1 != 0 {}
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            count.ctrla.modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
            count.wave.write(|w| w.wavegen().mpwm());
            count.cc[0].write(|w| unsafe { w.cc().bits(params.cycles as u16) });
            while count.syncbusy.read().cc0().bit_is_set() {}
            count.cc[1].write(|w| unsafe { w.cc().bits(0) });
            while count.syncbusy.read().cc1().bit_is_set() {}
            count.ctrla.modify(|_, w| w.enable().set_bit());
        }

        Self {
            clock_freq: clock.freq(),
            tc,
            pinout,
        }
    }

    pub fn get_period(&self) -> Hertz {
        let count = self.tc.count16();
        let divisor = count.ctrla.read().prescaler().bits();
        let top = count.cc[0].read().cc().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top as u32 + 1))
    }

    pub fn set_period<P>(&mut self, period: P)
    where
        P: Into<Hertz>
    {
        let period = period.into();
        let params = TimerParams::new(period, self.clock_freq.0);
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().clear_bit());
        count.ctrla.modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
        count.ctrla.modify(|_, w| w.enable().set_bit());
        count.cc[0].write(|w| unsafe { w.cc().bits(params.cycles as u16) });
        while count.syncbusy.read().cc0().bit_is_set() {}
    }
}

impl<I: PinId> PwmPin for $TYPE<I> {
    type Duty = u16;

    fn disable(&mut self) {
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().clear_bit());
    }

    fn enable(&mut self) {
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().set_bit());
    }


    fn get_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let duty: u16 = count.ccbuf[1].read().ccbuf().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let top = count.cc[0].read().cc().bits();
        top
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        let count = self.tc.count16();
        count.ccbuf[1].write(|w| unsafe {w.ccbuf().bits(duty)});
    }
}

)+}}

pwm! {
    Pwm0: (TC0, TC0Pinout, Tc0Tc1Clock, apbamask, tc0_, Pwm0Wrapper),
    Pwm1: (TC1, TC1Pinout, Tc0Tc1Clock, apbamask, tc1_, Pwm1Wrapper),
    Pwm2: (TC2, TC2Pinout, Tc2Tc3Clock, apbbmask, tc2_, Pwm2Wrapper),
    Pwm3: (TC3, TC3Pinout, Tc2Tc3Clock, apbbmask, tc3_, Pwm3Wrapper),
}

#[cfg(feature = "min-samd51j")]
pwm! {
    Pwm4: (TC4, TC4Pinout, Tc4Tc5Clock, apbcmask, tc4_, Pwm4Wrapper),
    Pwm5: (TC5, TC5Pinout, Tc4Tc5Clock, apbcmask, tc5_, Pwm5Wrapper),
}

#[cfg(feature = "min-samd51n")]
pwm! {
    Pwm6: (TC6, TC6Pinout, Tc6Tc7Clock, apbdmask, tc6_, Pwm6Wrapper),
    Pwm7: (TC7, TC7Pinout, Tc6Tc7Clock, apbdmask, tc7_, Pwm7Wrapper),
}

// Alternate PWM mode for TCs that let you use both waveform outputs with TOP being used to control period
impl_tc_pinout!(TC0PinoutAlt: [
    (Pa4, PA04),
    (Pa5, PA05),
    (Pa8, PA08),
    (Pa9, PA09),
    #[cfg(feature = "min-samd51j")]
    (Pb30, PB30),
    #[cfg(feature = "min-samd51j")]
    (Pb31, PB31)
]);
impl_tc_pinout!(TC1PinoutAlt: [(Pa6, PA06), (Pa7, PA07), (Pa10, PA10), (Pa11, PA11)]);
impl_tc_pinout!(TC2PinoutAlt: [(Pa0, PA00), (Pa1, PA01), (Pa12, PA12), (Pa13, PA13), (Pa16, PA16), (Pa17, PA17)]);
impl_tc_pinout!(TC3PinoutAlt: [(Pa14, PA14), (Pa15, PA15), (Pa18, PA18), (Pa19, PA19)]);
#[cfg(feature = "min-samd51j")]
impl_tc_pinout!(TC4PinoutAlt: [(Pa22, PA22), (Pa23, PA23), (Pb08, PB08), (Pb09, PB09), (Pb12, PB12), (Pb13, PB13)]);
#[cfg(feature = "min-samd51j")]
impl_tc_pinout!(TC5PinoutAlt: [(Pa24, PA24), (Pa25, PA25), (Pb10, PB10), (Pb11, PB11), (Pb14, PB14), (Pb15, PB15)]);
#[cfg(feature = "min-samd51n")]
impl_tc_pinout!(TC6PinoutAlt: [(Pa30, PA30), (Pa31, PA31), (Pb2, PB02), (Pb3, PB03), (Pb16, PB16), (Pb17, PB17)]);
#[cfg(feature = "min-samd51n")]
impl_tc_pinout!(TC7PinoutAlt: [(Pa20, PA20), (Pa21, PA21), (Pb0, PB00), (Pb1, PB01), (Pb22, PB22), (Pb23, PB23)]);

pub enum TcClockPrescaler {
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
    Div64,
    Div256,
    Div1024,
}

macro_rules! pwm_tc {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE<I: PinId> {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: $TC,
    #[allow(dead_code)]
    pinout: $pinout<I>,
}

impl<I: PinId> $TYPE<I> {
    pub fn new (
        clock: &clock::$clock,
        clock_divider: TcClockPrescaler,
        tc: $TC,
        pinout: $pinout<I>,
        mclk: &mut MCLK,
    ) -> Self {
        let count = tc.count16();
        mclk.$apmask.modify(|_, w| w.$apbits().set_bit());
        count.ctrla.write(|w| w.swrst().set_bit());
        while count.ctrla.read().bits() & 1 != 0 {}
        count.ctrla.modify(|_, w| w.enable().clear_bit());
        count.ctrla.modify(|_, w| {
            match clock_divider {
                TcClockPrescaler::Div1 => w.prescaler().div1(),
                TcClockPrescaler::Div2 => w.prescaler().div2(),
                TcClockPrescaler::Div4 => w.prescaler().div4(),
                TcClockPrescaler::Div8 => w.prescaler().div8(),
                TcClockPrescaler::Div16 => w.prescaler().div16(),
                TcClockPrescaler::Div64 => w.prescaler().div64(),
                TcClockPrescaler::Div256 => w.prescaler().div256(),
                TcClockPrescaler::Div1024 => w.prescaler().div1024(),
            }
        });
        count.wave.write(|w| w.wavegen().npwm());
        count.ctrla.modify(|_, w| w.enable().set_bit());

        Self {
            clock_freq: clock.freq(),
            tc,
            pinout,
        }
    }
}

impl<I: PinId> Pwm for $TYPE<I> {
    type Channel = Channel;
    type Time = Hertz;
    type Duty = u16;

    fn disable(&mut self, _channel: Self::Channel) {
        self.tc.count16().ctrla.modify(|_, w| w.enable().clear_bit());
        while self.tc.count16().syncbusy.read().enable().bit_is_set() {}
    }

    fn enable(&mut self, _channel: Self::Channel) {
        self.tc.count16().ctrla.modify(|_, w| w.enable().set_bit());
        while self.tc.count16().syncbusy.read().enable().bit_is_set() {}
    }

    fn get_period(&self) -> Self::Time {
        let divisor = self.tc.count16().ctrla.read().prescaler().bits();
        let top = u16::MAX;
        Hertz(self.clock_freq.0 / (1u32 << divisor) as u32 / (top as u32 + 1))
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let duty = self.tc.count16().cc[channel as usize].read().cc().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let top = u16::MAX;
        top
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        self.tc.count16().cc[channel as usize].write(|w| unsafe { w.cc().bits(duty) });
    }

    fn set_period<P>(&mut self, _period: P)
    where
        P: Into<Self::Time>,
    {
        panic!("Not implemented");
    }
}

        )+
    };
}

pwm_tc! {
    Tc0Pwm: (TC0, TC0PinoutAlt, Tc0Tc1Clock, apbamask, tc0_, TcPwm0Wrapper),
    Tc1Pwm: (TC1, TC1PinoutAlt, Tc0Tc1Clock, apbamask, tc1_, TcPwm1Wrapper),
    Tc2Pwm: (TC2, TC2PinoutAlt, Tc2Tc3Clock, apbbmask, tc2_, TcPwm2Wrapper),
    Tc3Pwm: (TC3, TC3PinoutAlt, Tc2Tc3Clock, apbbmask, tc3_, TcPwm3Wrapper),
}

#[cfg(feature = "min-samd51j")]
pwm_tc! {
    Tc4Pwm: (TC4, TC4PinoutAlt, Tc4Tc5Clock, apbcmask, tc4_, TcPwm4Wrapper),
    Tc5Pwm: (TC5, TC5PinoutAlt, Tc4Tc5Clock, apbcmask, tc5_, TcPwm5Wrapper),
}

#[cfg(feature = "min-samd51n")]
pwm_tc! {
    Tc6Pwm: (TC6, TC6PinoutAlt, Tc6Tc7Clock, apbdmask, tc6_, TcPwm6Wrapper),
    Tc7Pwm: (TC7, TC7PinoutAlt, Tc6Tc7Clock, apbdmask, tc7_, TcPwm7Wrapper),
}

// Timer/Counter for Control Applications (TCCx)

#[derive(Copy, Clone)]
pub enum Channel {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

/// This is a major syntax hack.
///
/// The previous Pinout types were enums that took specific v1::Pin types. As a
/// result, there was no way to make that implementation simultaneously
/// compatible with both v1::Pin and Pin.
///
/// BUT, the enum variant syntax is the same as the namespaced function syntax.
/// I converted the enums to structs, and I created constructor methods with the
/// same names as the previous enum variants. By constructing Pinout types with
/// functions rather than enum variants, you can make it generic over v1::Pin
/// and Pin types.
///
/// This is (mostly) backwards compatible with the current syntax, and all the
/// existing calls compile. The only incompatible change is the requirement of
/// type parameters on the Pwm types. Most of the type, the type parameters can
/// be inferred, so this is mostly backwards compatible as well. But there were
/// one or two instances where I had to add explicit type parameters to existing
/// BSP code.
macro_rules! impl_tcc_pinout {
    (
        $Type:ident: [ $(
            $( #[$attr:meta] )?
            ($func: ident, $Id: ident, $Mode:ident)
        ),+ ]
    ) => {
        pub struct $Type<I: PinId, M: PinMode> {
            _pin: Pin<I, M>,
        }

        $(
            $( #[$attr] )?
            impl $Type<$Id, $Mode> {
                #[inline]
                pub fn $func(pin: impl AnyPin<Id = $Id>) -> Self {
                    let _pin = pin.into().into_alternate();
                    Self { _pin }
                }
            }
        )+
    };
}

impl_tcc_pinout!(TCC0Pinout: [
    (Pa8, PA08, AlternateF),
    (Pa9, PA09, AlternateF),
    (Pa10, PA10, AlternateF),
    (Pa11, PA11, AlternateF),
    (Pa12, PA12, AlternateF),
    (Pa13, PA13, AlternateF),
    (Pa16, PA16, AlternateG),
    (Pa17, PA17, AlternateG),
    (Pa18, PA18, AlternateG),
    (Pa19, PA19, AlternateG),
    (Pa20, PA20, AlternateG),
    (Pa21, PA21, AlternateG),
    (Pa22, PA22, AlternateG),
    (Pa23, PA23, AlternateG),
    (Pb10, PB10, AlternateF),
    (Pb11, PB11, AlternateF),
    #[cfg(feature = "min-samd51j")]
    (Pb12, PB12, AlternateG),
    #[cfg(feature = "min-samd51j")]
    (Pb13, PB13, AlternateG),
    #[cfg(feature = "min-samd51j")]
    (Pb14, PB14, AlternateG),
    #[cfg(feature = "min-samd51j")]
    (Pb15, PB15, AlternateG),
    #[cfg(feature = "min-samd51j")]
    (Pb16, PB16, AlternateG),
    #[cfg(feature = "min-samd51j")]
    (Pb17, PB17, AlternateG),
    #[cfg(feature = "min-samd51j")]
    (Pb30, PB30, AlternateG),
    #[cfg(feature = "min-samd51j")]
    (Pb31, PB31, AlternateG),
    #[cfg(feature = "min-samd51n")]
    (Pc10, PC10, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc11, PC11, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc12, PC12, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc13, PC13, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc14, PC14, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc15, PC15, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc16, PC16, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc17, PC17, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc18, PC18, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc19, PC19, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc20, PC20, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc21, PC21, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pc4, PC04, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pc22, PC22, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pc23, PC23, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pd8, PD08, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pd9, PD09, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pd10, PD10, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pd11, PD11, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pd12, PD12, AlternateF)
]);

impl_tcc_pinout!(TCC1Pinout: [
    (Pa8, PA08, AlternateG),
    (Pa9, PA09, AlternateG),
    (Pa10, PA10, AlternateG),
    (Pa11, PA11, AlternateG),
    (Pa12, PA12, AlternateG),
    (Pa13, PA13, AlternateG),
    (Pa14, PA14, AlternateG),
    (Pa15, PA15, AlternateG),
    (Pa16, PA16, AlternateF),
    (Pa17, PA17, AlternateF),
    (Pa18, PA18, AlternateF),
    (Pa19, PA19, AlternateF),
    (Pa20, PA20, AlternateF),
    (Pa21, PA21, AlternateF),
    (Pa22, PA22, AlternateF),
    (Pa23, PA23, AlternateF),
    (Pb10, PB10, AlternateG),
    (Pb11, PB11, AlternateG),
    #[cfg(feature = "min-samd51n")]
    (Pb18, PB18, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pb19, PB19, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pb20, PB20, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pb21, PB21, AlternateF),
    #[cfg(feature = "min-samd51n")]
    (Pc10, PC10, AlternateG),
    #[cfg(feature = "min-samd51n")]
    (Pc11, PC11, AlternateG),
    #[cfg(feature = "min-samd51n")]
    (Pc12, PC12, AlternateG),
    #[cfg(feature = "min-samd51n")]
    (Pc13, PC13, AlternateG),
    #[cfg(feature = "min-samd51n")]
    (Pc14, PC14, AlternateG),
    #[cfg(feature = "min-samd51n")]
    (Pc15, PC15, AlternateG),
    #[cfg(feature = "min-samd51p")]
    (Pb26, PB26, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pb27, PB27, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pb28, PB28, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pb29, PB29, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pd20, PD20, AlternateF),
    #[cfg(feature = "min-samd51p")]
    (Pd21, PD21, AlternateF)
]);

impl_tcc_pinout!(TCC2Pinout: [
    (Pa14, PA14, AlternateF),
    (Pa15, PA15, AlternateF),
    (Pa24, PA24, AlternateF),
    (Pa30, PA30, AlternateF),
    (Pa31, PA31, AlternateF),
    (Pb2,  PB02, AlternateF)
]);

#[cfg(feature = "min-samd51j")]
impl_tcc_pinout!(TCC3Pinout: [
    (Pb12, PB12, AlternateF),
    (Pb13, PB13, AlternateF),
    (Pb16, PB16, AlternateF),
    (Pb17, PB17, AlternateF)
]);

#[cfg(feature = "min-samd51j")]
impl_tcc_pinout!(TCC4Pinout: [
    (Pb14, PB14, AlternateF),
    (Pb15, PB15, AlternateF),
    (Pb30, PB30, AlternateF),
    (Pb31, PB31, AlternateF)
]);

macro_rules! pwm_tcc {
    ($($TYPE:ident: ($TCC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE<I: PinId, M: PinMode> {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tcc: $TCC,
    #[allow(dead_code)]
    pinout: $pinout<I, M>,
}

impl<I: PinId, M: PinMode> $TYPE<I, M> {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tcc: $TCC,
        pinout: $pinout<I, M>,
        mclk: &mut MCLK,
    ) -> Self {
        let freq = freq.into();
        {
            let params = TimerParams::new(freq, clock.freq().0);
            mclk.$apmask.modify(|_, w| w.$apbits().set_bit());
            tcc.ctrla.write(|w| w.swrst().set_bit());
            while tcc.syncbusy.read().swrst().bit_is_set() {}
            tcc.ctrlbclr.write(|w| w.dir().set_bit() );
            while tcc.syncbusy.read().ctrlb().bit_is_set() {}
            tcc.ctrla.modify(|_, w| w.enable().clear_bit());
            tcc.ctrla.modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
            tcc.wave.write(|w| w.wavegen().npwm());
            while tcc.syncbusy.read().wave().bit_is_set() {}
            tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
            while tcc.syncbusy.read().per().bit_is_set() {}
            tcc.ctrla.modify(|_, w| w.enable().set_bit());
        }

        Self {
            clock_freq: clock.freq(),
            tcc,
            pinout,
        }
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_interrupt_flags(&self) -> Flags {
        Flags::from_bits_truncate(self.tcc.intflag.read().bits())
    }

    /// Clear the interrupt flags
    #[inline]
    pub fn clear_interrupt_flags(&mut self, flags: Flags) {
        self.tcc.intflag.modify(|_, w| unsafe { w.bits(flags.bits()) });
    }

    /// Enable interrupts for the specified flags
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.tcc.intenset.write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable specified interrupts
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.tcc.intenclr.write(|w| unsafe { w.bits(flags.bits()) });
    }
}

impl<I: PinId, M: PinMode> Pwm for $TYPE<I, M> {
    type Channel = Channel;
    type Time = Hertz;
    type Duty = u32;

    fn disable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
    }

    fn enable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
    }

    fn get_period(&self) -> Self::Time {
        let divisor = self.tcc.ctrla.read().prescaler().bits();
        let top = self.tcc.per().read().bits();
        Hertz(self.clock_freq.0 / (1u32 << divisor) / (top + 1) as u32)
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let cc = self.tcc.cc();
        let duty = cc[channel as usize].read().cc().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let top = self.tcc.per().read().bits();
        top
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        let cc = self.tcc.cc();
        cc[channel as usize].write(|w| unsafe { w.cc().bits(duty) });
    }

    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time>,
    {
        let period = period.into();
        let params = TimerParams::new(period, self.clock_freq.0);
        self.tcc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
        self.tcc.ctrla.modify(|_, w| {
            match params.divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            }
        });
        self.tcc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
        self.tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
        while self.tcc.syncbusy.read().per().bit() {}
    }
}

        )+
    };
}

pwm_tcc! {
    Tcc0Pwm: (TCC0, TCC0Pinout, Tcc0Tcc1Clock, apbbmask, tcc0_, TccPwm0Wrapper),
    Tcc1Pwm: (TCC1, TCC1Pinout, Tcc0Tcc1Clock, apbbmask, tcc1_, TccPwm1Wrapper),
    Tcc2Pwm: (TCC2, TCC2Pinout, Tcc2Tcc3Clock, apbcmask, tcc2_, TccPwm2Wrapper),
}

#[cfg(feature = "min-samd51j")]
pwm_tcc! {
    Tcc3Pwm: (TCC3, TCC3Pinout, Tcc2Tcc3Clock, apbcmask, tcc3_, TccPwm3Wrapper),
    Tcc4Pwm: (TCC4, TCC4Pinout, Tcc4Clock,     apbdmask, tcc4_, TccPwm4Wrapper),
}
