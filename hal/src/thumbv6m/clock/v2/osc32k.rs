//! # Osc32k - Internal 32 kHz oscillator

#![allow(missing_docs)]

use typenum::U0;

use crate::pac::sysctrl::{OSC32K, PCLKSR};

use crate::time::Hertz;
use crate::typelevel::{Counter, Sealed};

use super::{Enabled, Source};

//==============================================================================
// Ids
//==============================================================================

/// Type-level variant representing the identity of the OSC32K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Osc32kId {}
impl Sealed for Osc32kId {}

//==============================================================================
// Startup
//==============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Startup {
    CYCLE3,
    CYCLE4,
    CYCLE6,
    CYCLE10,
    CYCLE18,
    CYCLE34,
    CYCLE66,
    CYCLE130,
}

impl From<Startup> for u8 {
    fn from(startup: Startup) -> Self {
        match startup {
            Startup::CYCLE3 => 0x0,
            Startup::CYCLE4 => 0x1,
            Startup::CYCLE6 => 0x2,
            Startup::CYCLE10 => 0x3,
            Startup::CYCLE18 => 0x4,
            Startup::CYCLE34 => 0x5,
            Startup::CYCLE66 => 0x6,
            Startup::CYCLE130 => 0x7,
        }
    }
}

//==============================================================================
// Osc32kToken
//==============================================================================

pub struct Osc32kToken(());

impl Osc32kToken {
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
    fn osc32k(&self) -> &OSC32K {
        &self.sysctrl().osc32k
    }

    #[inline]
    fn pclksr(&self) -> &PCLKSR {
        &self.sysctrl().pclksr
    }

    #[inline]
    fn set_start_up(&mut self, start_up: Startup) {
        #[cfg(not(feature = "samda1"))]
        self.osc32k()
            .modify(|_, w| unsafe { w.startup().bits(start_up.into()) });
        #[cfg(feature = "samda1")]
        self.osc32k()
            .modify(|_, w| w.startup().bits(start_up.into()));
    }

    #[inline]
    fn set_calibration(&mut self, calib: u8) {
        self.osc32k()
            .modify(|_, w| unsafe { w.calib().bits(calib) });
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.osc32k().modify(|_, w| w.ondemand().bit(on_demand));
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.osc32k().modify(|_, w| w.runstdby().bit(run_standby));
    }

    #[inline]
    fn enable_32k(&mut self, enabled: bool) {
        self.osc32k().modify(|_, w| w.en32k().bit(enabled));
    }

    #[inline]
    fn enable(&mut self) {
        self.osc32k().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn wrtlock(&mut self) {
        self.osc32k().modify(|_, w| w.wrtlock().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.osc32k().modify(|_, w| w.enable().bit(false));
    }

    #[inline]
    fn wait_ready(&self) {
        while self.pclksr().read().osc32krdy().bit_is_clear() {}
    }
}

//==============================================================================
// Osc32k
//==============================================================================

pub struct Osc32k {
    token: Osc32kToken,
    run_standby: bool,
    on_demand_mode: bool,
    start_up: Startup,
}

pub type EnabledOsc32k<N = U0> = Enabled<Osc32k, N>;

impl Osc32k {
    #[inline]
    pub fn new(token: Osc32kToken) -> Self {
        Self {
            token,
            run_standby: false,
            on_demand_mode: true,
            start_up: Startup::CYCLE66,
        }
    }

    #[inline]
    pub fn free(self) -> Osc32kToken {
        self.token
    }

    /// Set for how long the clock output should be masked during startup
    #[inline]
    pub fn start_up(mut self, start_up: Startup) -> Self {
        self.start_up = start_up;
        self
    }

    /// Controls how [`Osc32k`] behaves when a peripheral clock request is
    /// detected
    #[inline]
    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand_mode = on_demand;
        self
    }

    /// Controls how [`Osc32k`] should behave during standby
    #[inline]
    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }

    /// Wait until the clock source is ready
    #[inline]
    pub fn wait_ready(&self) {
        self.token.wait_ready();
    }

    /// Override the factory-default calibration value
    #[inline]
    pub fn set_calibration(&mut self, calib: u8) {
        self.token.set_calibration(calib);
    }

    /// Set the write-lock, which will last until POR
    ///
    /// This function sets the write-lock bit, which lasts until power-on reset.
    /// It also consumes and drops the [`Osc32k`], which destroys API access
    /// to the registers.
    #[inline]
    pub fn write_lock(mut self) {
        self.token.wrtlock();
    }

    #[inline]
    pub fn enable(mut self) -> EnabledOsc32k {
        self.token.set_on_demand(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token.set_start_up(self.start_up);
        self.token.enable_32k(true);
        self.token.enable();
        Enabled::new(self)
    }
}

impl<N: Counter> EnabledOsc32k<N> {
    /// Set the write-lock, which will last until POR
    ///
    /// This function sets the write-lock bit, which lasts until power-on reset.
    /// It also consumes and drops the [`Osc32k`], which destroys API access
    /// to the registers.
    #[inline]
    pub fn write_lock(mut self) {
        self.0.token.wrtlock();
    }

    pub fn disable(mut self) -> Osc32k {
        self.0.token.disable();
        self.0.token.enable_32k(false);
        self.0
    }
}

impl<N: Counter> Source for EnabledOsc32k<N> {
    type Id = Osc32kId;

    fn freq(&self) -> Hertz {
        Hertz(32_768)
    }
}
