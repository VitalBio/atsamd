//! # Xosc - External oscillator
//!
//! A signal source for [`Gclks`][super::gclk] and [`Dplls`][super::dpll].
//!
//! There is one external oscillator that is available:
//! - [`Enabled`]`<`[`Xosc`]`<`[`marker::Xosc`]`, _>>`: [`Xosc`]
//!
//! There are two modes of operation that are available:
//! - [`Enabled`]`<`[`Xosc`]`<_, `[`CrystalMode`]`>>`: Xosc is being powered by
//!   an external crystal (2 pins)
//! - [`Enabled`]`<`[`Xosc`]`<_, `[`ClockMode`]`>>`: Xosc is being powered by an
//!   external signal (1 pin)
//!
//! To construct a Xosc in a proper mode use an appropriate construction
//! function:
//! - [`Xosc::from_clock`]
//! - [`Xosc::from_crystal`]
//! Then, enable it with a [`Xosc::enable`] function call
//!
use typenum::U0;

use crate::pac::sysctrl::xosc::GAIN_A;
use crate::pac::sysctrl::{PCLKSR, XOSC};

use crate::gpio::{FloatingDisabled, Pin, PA14, PA15};
use crate::time::Hertz;
use crate::typelevel::{Counter, Sealed};

use super::{Enabled, Source};

//==============================================================================
// Ids
//==============================================================================

/// Type-level variant representing the identity of the XOSC clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum XoscId {}
impl Sealed for XoscId {}

//==============================================================================
// Gain
//==============================================================================

/// Gain settings
///
/// The gain field is usually set by crystal frequency range. Normally:
/// - 0 -> 2MHz
/// - 1 -> 4MHz
/// - 2 -> 8MHz
/// - 3 -> 16MHz
/// - 4 -> 32MHz
///
/// However, the datasheet notes that it might vary based on capacitive load
/// and crystal characteristics. Gain is only used when automatic amplitude
/// gain control is disabled.
///
/// The `Zero` variant will leave `IMULT` and `IPTAT` at their default settings
/// of zero.
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Gain {
    Zero,
    TwoMHz,
    FourMHz,
    EightMHz,
    SixteenMHz,
    ThirtyTwoMHz,
}

impl From<Gain> for GAIN_A {
    fn from(gain: Gain) -> Self {
        match gain {
            Gain::Zero => GAIN_A::_0,
            Gain::TwoMHz => GAIN_A::_0,
            Gain::FourMHz => GAIN_A::_1,
            Gain::EightMHz => GAIN_A::_2,
            Gain::SixteenMHz => GAIN_A::_3,
            Gain::ThirtyTwoMHz => GAIN_A::_4,
        }
    }
}

//==============================================================================
// Startup
//==============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Startup {
    CYCLE1,
    CYCLE2,
    CYCLE4,
    CYCLE8,
    CYCLE16,
    CYCLE32,
    CYCLE64,
    CYCLE128,
    CYCLE256,
    CYCLE512,
    CYCLE1024,
    CYCLE2048,
    CYCLE4096,
    CYCLE8192,
    CYCLE16384,
    CYCLE32768,
}

impl From<Startup> for u8 {
    fn from(startup: Startup) -> Self {
        match startup {
            Startup::CYCLE1 => 0x0,
            Startup::CYCLE2 => 0x1,
            Startup::CYCLE4 => 0x2,
            Startup::CYCLE8 => 0x3,
            Startup::CYCLE16 => 0x4,
            Startup::CYCLE32 => 0x5,
            Startup::CYCLE64 => 0x6,
            Startup::CYCLE128 => 0x7,
            Startup::CYCLE256 => 0x8,
            Startup::CYCLE512 => 0x9,
            Startup::CYCLE1024 => 0xA,
            Startup::CYCLE2048 => 0xB,
            Startup::CYCLE4096 => 0xC,
            Startup::CYCLE8192 => 0xD,
            Startup::CYCLE16384 => 0xE,
            Startup::CYCLE32768 => 0xF,
        }
    }
}

//==============================================================================
// XoscToken
//==============================================================================

/// Token struct that is essential in order to construct an instance of an
/// [`Xosc`].
pub struct XoscToken(());

impl XoscToken {
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
    fn xosc(&self) -> &XOSC {
        &self.sysctrl().xosc
    }

    #[inline]
    fn pclksr(&self) -> &PCLKSR {
        &self.sysctrl().pclksr
    }

    #[inline]
    fn reset(&self) {
        self.xosc().reset();
    }

    #[inline]
    fn set_start_up(&mut self, start_up: Startup) {
        #[cfg(not(feature = "samda1"))]
        self.xosc()
            .modify(|_, w| unsafe { w.startup().bits(start_up.into()) });
        #[cfg(feature = "samda1")]
        self.xosc()
            .modify(|_, w| w.startup().bits(start_up.into()));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.xosc().modify(|_, w| w.ondemand().bit(on_demand));
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.xosc().modify(|_, w| w.runstdby().bit(run_standby));
    }

    #[inline]
    fn set_source(&mut self, from_crystal: bool) {
        self.xosc().modify(|_, w| w.xtalen().bit(from_crystal));
    }

    #[inline]
    fn enable(&mut self) {
        self.xosc().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.xosc().modify(|_, w| w.enable().bit(false));
    }

    #[inline]
    fn wait_ready(&self) {
        while self.pclksr().read().xoscrdy().bit_is_clear() {}
    }

    #[inline]
    fn set_gain(&mut self, gain: Gain) {
        self.xosc().modify(|_, w| w.gain().variant(gain.into()));
    }

    #[inline]
    fn set_amplitude_loop_control(&mut self, ampgc: bool) {
        self.xosc().modify(|_, w| w.ampgc().bit(ampgc));
    }
}

//==============================================================================
// Aliases
//==============================================================================

/// [`Pin`] alias for the XOSC32K input pin
///
/// This pin is required in both [`ClockMode`] and [`CrystalMode`]
pub type XIn = Pin<PA14, FloatingDisabled>;

/// [`Pin`] alias for the XOSC32K output pin
///
/// This pin is only required in [`CrystalMode`]
pub type XOut = Pin<PA15, FloatingDisabled>;

//==============================================================================
// Mode
//==============================================================================

/// Type-level `enum` for the [`Xosc`] operation mode
///
/// An [`Xosc`] can be sourced from either an external clock or a cyrstal
/// oscillator. This type-level `enum` provides the type-level variants
/// [`ClockMode`] and [`CrystalMode`].
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait Mode: Sealed {
    /// `XTALEN` field for the corresponding mode
    const XTALEN: bool;
    /// Get the gain value
    fn gain(&self) -> Gain;
    /// Get the amplitude loop control bit
    fn amplitude_loop_control(&self) -> bool;
}

/// Type-level variant of the [`Xosc`] operation [`Mode`]
///
/// Represents the [`Xosc`] configured to use an externally provided clock.
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub struct ClockMode;
impl Sealed for ClockMode {}
impl Mode for ClockMode {
    const XTALEN: bool = false;
    #[inline]
    fn gain(&self) -> Gain {
        Gain::Zero
    }
    #[inline]
    fn amplitude_loop_control(&self) -> bool {
        false
    }
}

/// Type-level variant of the [`Xosc`] operation [`Mode`]
///
/// Represents the [`Xosc`] configured to use an external crystal oscillator.
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub struct CrystalMode {
    xout: XOut,
    gain: Gain,
    amplitude_loop_control: bool,
}
impl Sealed for CrystalMode {}
impl Mode for CrystalMode {
    const XTALEN: bool = true;
    #[inline]
    fn gain(&self) -> Gain {
        self.gain
    }
    #[inline]
    fn amplitude_loop_control(&self) -> bool {
        self.amplitude_loop_control
    }
}

//==============================================================================
// Xosc
//==============================================================================

/// Struct representing a disabled external oscillator
///
/// It is generic over:
/// - a mode of operation (available modes: [`ClockMode`], [`CrystalMode`])
pub struct Xosc<M>
where
    M: Mode,
{
    token: XoscToken,
    mode: M,
    xin: XIn,
    src_freq: Hertz,
    start_up_cycles: Startup,
    on_demand: bool,
    run_standby: bool,
}

pub type EnabledXosc<M, N = U0> = Enabled<Xosc<M>, N>;

impl<M> Xosc<M>
where
    M: Mode,
{
    /// Returns the frequency of the oscillator
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.src_freq
    }

    /// Sets the number of cycles allowed to pass before Clock Failure Detection
    /// (CFD) starts monitoring the external oscillator.
    #[inline]
    pub fn set_start_up(mut self, start_up: Startup) -> Self {
        self.start_up_cycles = start_up;
        self
    }
    /// Controls the on demand functionality of the clock source
    ///
    /// Only starts the clock source when a peripheral uses it
    ///
    /// If cleared the clock will be always active
    /// See Datasheet c. 13.5 for details
    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand = on_demand;
        self
    }

    /// Controls the clock source behaviour during standby
    ///
    /// See Datasheet c. 28.6.2
    #[inline]
    pub fn set_run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }

    /// Modify hardware to realise the desired state
    /// stored within the [`Xosc`]
    ///
    /// Returns the enabled Xosc
    #[inline]
    pub fn enable(mut self) -> EnabledXosc<M> {
        self.token.reset();
        self.token.set_source(M::XTALEN);
        self.token.set_start_up(self.start_up_cycles);
        self.token.set_on_demand(self.on_demand);
        self.token.set_run_standby(self.run_standby);
        self.token
            .set_amplitude_loop_control(self.mode.amplitude_loop_control());
        self.token.set_gain(self.mode.gain());
        self.token.enable();
        Enabled::new(self)
    }
}

impl Xosc<ClockMode> {
    /// Construct a [`Xosc`] from a single pin oscillator clock signal
    #[inline]
    pub fn from_clock(token: XoscToken, xin: impl Into<XIn>, src_freq: impl Into<Hertz>) -> Self {
        let xin = xin.into().into_floating_disabled();
        let start_up_cycles = Startup::CYCLE1;
        // Mimic default reset state
        let on_demand = true;
        let run_standby = false;
        Self {
            token,
            mode: ClockMode,
            xin,
            src_freq: src_freq.into(),
            start_up_cycles,
            on_demand,
            run_standby,
        }
    }

    /// Deconstruct the Xosc and return the inner XoscToken
    #[inline]
    pub fn free(self) -> (XoscToken, XIn) {
        (self.token, self.xin)
    }
}

impl Xosc<CrystalMode> {
    /// Construct a [`Xosc`] from a two pin crystal oscillator signal
    ///
    /// The crystal oscillator frequency must be supported, for valid
    /// frequencies see [`CrystalCurrent`].
    ///
    /// By default `Amplitude Loop Control` is set, see
    /// [`Xosc::set_amplitude_loop_control`]
    #[inline]
    pub fn from_crystal(
        token: XoscToken,
        xin: impl Into<XIn>,
        xout: impl Into<XOut>,
        src_freq: impl Into<Hertz>,
    ) -> Self {
        let xin = xin.into();
        let xout = xout.into();
        let src_freq = src_freq.into();

        // Lowers power usage and protects the crystal
        let amplitude_loop_control = true;

        let start_up_cycles = Startup::CYCLE1;
        let on_demand = true;
        let run_standby = false;
        let mode = CrystalMode {
            xout,
            gain: Gain::Zero,
            amplitude_loop_control,
        };
        Self {
            token,
            mode,
            xin,
            src_freq,
            start_up_cycles,
            on_demand,
            run_standby,
        }
    }

    /// Controls the automatic amplitude loop control
    ///
    /// Recommended option, ensures the crystal is not overdriven,
    /// and lowers power consumption. See datasheet c. 54.13 p. 1811
    #[inline]
    pub fn set_amplitude_loop_control(mut self, ampgc: bool) -> Self {
        self.mode.amplitude_loop_control = ampgc;
        self
    }

    /// Set the amplitude gain value
    #[inline]
    pub fn set_gain(mut self, gain: Gain) -> Self {
        self.mode.gain = gain;
        self
    }

    /// Deconstruct the Xosc and return the inner XoscToken
    #[inline]
    pub fn free(self) -> (XoscToken, XIn, XOut) {
        (self.token, self.xin, self.mode.xout)
    }
}

impl<M> EnabledXosc<M>
where
    M: Mode,
{
    /// Disable the [`Xosc`]
    ///
    /// Only possible when nothing uses the `Xosc`
    #[inline]
    pub fn disable(mut self) -> Xosc<M> {
        self.0.token.disable();
        self.0
    }
}

impl<M, N> EnabledXosc<M, N>
where
    M: Mode,
    N: Counter,
{
    /// Busy-wait until ready
    #[inline]
    pub fn wait_ready(&self) {
        self.0.token.wait_ready()
    }
}

//==============================================================================
// Source
//==============================================================================

impl<M, N> Source for EnabledXosc<M, N>
where
    M: Mode,
    N: Counter,
{
    type Id = XoscId;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
