//! # Osc32k - Internal 32 kHz oscillator

#![allow(missing_docs)]

use typenum::U0;

use crate::pac::sysctrl::{OSC8M, PCLKSR};

use crate::time::Hertz;
use crate::typelevel::{Counter, Sealed};

use super::{Enabled, Source};

//==============================================================================
// Ids
//==============================================================================

/// Type-level variant representing the identity of the OSC8M clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Osc8mId {}
impl Sealed for Osc8mId {}

//==============================================================================
// Frequency Range
//==============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum FreqRange {
    Range4To6Mhz,
    Range6To8Mhz,
    Range8To11Mhz,
    Range11To15Mhz,
}

impl From<FreqRange> for u8 {
    fn from(freq_range: FreqRange) -> Self {
        match freq_range {
            FreqRange::Range4To6Mhz => 0x0,
            FreqRange::Range6To8Mhz => 0x1,
            FreqRange::Range8To11Mhz => 0x2,
            FreqRange::Range11To15Mhz => 0x3,
        }
    }
}

//==============================================================================
// Prescaler
//==============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Prescaler {
    Prescaler1,
    Prescaler2,
    Prescaler4,
    Prescaler8,
}

impl From<Prescaler> for u8 {
    fn from(prescaler: Prescaler) -> Self {
        match prescaler {
            Prescaler::Prescaler1 => 0x0,
            Prescaler::Prescaler2 => 0x1,
            Prescaler::Prescaler4 => 0x2,
            Prescaler::Prescaler8 => 0x3,
        }
    }
}

//==============================================================================
// Osc8mToken
//==============================================================================

pub struct Osc8mToken(());

impl Osc8mToken {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn sysctrl(&self) -> &crate::pac::sysctrl::RegisterBlock {
        unsafe { &*crate::pac::SYSCTRL::ptr() }
    }

    #[inline]
    fn osc8m(&self) -> &OSC8M {
        &self.sysctrl().osc8m
    }

    #[inline]
    fn pclksr(&self) -> &PCLKSR {
        &self.sysctrl().pclksr
    }

    #[inline]
    fn set_calibration(&mut self, calib: u16) {
        self.osc8m().modify(|_, w| unsafe { w.calib().bits(calib) });
    }

    #[inline]
    fn set_frequency_range(&mut self, freq_range: FreqRange) {
        self.osc8m()
            .modify(|_, w| w.frange().bits(freq_range.into()));
    }

    #[inline]
    fn set_prescaler(&mut self, prescaler: Prescaler) {
        self.osc8m().modify(|_, w| w.presc().bits(prescaler.into()));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.osc8m().modify(|_, w| w.ondemand().bit(on_demand));
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.osc8m().modify(|_, w| w.runstdby().bit(run_standby));
    }

    #[inline]
    fn enable(&mut self) {
        self.osc8m().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.osc8m().modify(|_, w| w.enable().bit(false));
    }

    #[inline]
    fn wait_ready(&self) {
        while self.pclksr().read().osc8mrdy().bit_is_clear() {}
    }
}

//==============================================================================
// Osc8m
//==============================================================================

pub struct Osc8m {
    token: Osc8mToken,
    run_standby: bool,
    on_demand_mode: bool,
    prescaler: Prescaler,
}

pub type EnabledOsc8m<N = U0> = Enabled<Osc8m, N>;

impl Osc8m {
    /// Returns the frequency of the oscillator
    #[inline]
    pub fn freq(&self) -> Hertz {
        match self.prescaler {
            Prescaler::Prescaler1 => Hertz(8_000),
            Prescaler::Prescaler2 => Hertz(4_000),
            Prescaler::Prescaler4 => Hertz(2_000),
            Prescaler::Prescaler8 => Hertz(1_000),
        }
    }

    #[inline]
    pub fn new(token: Osc8mToken) -> Self {
        Self {
            token,
            run_standby: false,
            on_demand_mode: true,
            prescaler: Prescaler::Prescaler1, // No division
        }
    }

    #[inline]
    pub fn free(self) -> Osc8mToken {
        self.token
    }

    /// Controls how [`Osc8m`] behaves when a peripheral clock request is
    /// detected
    #[inline]
    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand_mode = on_demand;
        self
    }

    /// Controls how [`Osc8m`] should behave during standby
    #[inline]
    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }

    #[inline]
    pub fn prescaler(mut self, prescaler: Prescaler) -> Self {
        self.prescaler = prescaler;
        self
    }

    /// Wait until the clock source is ready
    #[inline]
    pub fn wait_ready(&self) {
        self.token.wait_ready();
    }

    /// Override the factory-default calibration value
    #[inline]
    pub fn set_calibration(&mut self, calib: u16) {
        self.token.set_calibration(calib);
    }

    /// Override the factory-default frequency range value
    #[inline]
    pub fn set_frequency_range(&mut self, freq_range: FreqRange) {
        self.token.set_frequency_range(freq_range);
    }

    #[inline]
    pub fn enable(mut self) -> EnabledOsc8m {
        self.token.set_on_demand(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.set_prescaler(self.prescaler);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<N: Counter> EnabledOsc8m<N> {
    pub fn disable(mut self) -> Osc8m {
        self.0.token.disable();
        self.0
    }
}

impl<N: Counter> Source for EnabledOsc8m<N> {
    type Id = Osc8mId;

    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
