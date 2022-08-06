//! # Digital Phase Locked Loop (DPLL)
//!
//! [`Dpll`] allows user to multiply an input signal and supports a handful of
//! them. It maintains the output signal frequency by constant phase comparison
//! against the reference, input signal.
//!
//! There is one Dpll that is available
//!
//! - [`Enabled`]`<`[`Dpll`]`<`[`marker::Dpll`]`, _>>`: [`Dpll`]
//!
//! It can operate in 3 different modes
//!
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`PclkDriven`]`>`: `Gclk` as a reference clock
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`XoscDriven`]`>`: `Xosc{0, 1}` as a reference
//!   clock
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`Xosc32kDriven`]`>`: `Xosc32k` as a
//!   reference_clk
//!
//! It can be created by using an appropriate construction function:
//! - [`Dpll::from_pclk`]
//! - [`Dpll::from_xosc`]
//! - [`Dpll::from_xosc32k`]
//! and then enabled by [`Dpll::enable`] function call

use core::convert::Infallible;

use typenum::U0;

use crate::pac::sysctrl::dpllctrlb::REFCLK_A;
use crate::pac::sysctrl::{dpllstatus, DPLLCTRLA, DPLLCTRLB, DPLLRATIO};

use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, Sealed};

use super::gclk::GclkId;
use super::pclk::Pclk;
use super::xosc::XoscId;
use super::xosc32k::Xosc32kId;
use super::{Enabled, Source};

//==============================================================================
// DpllId
//==============================================================================

/// Type-level variant representing the identity of the DPLL clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum DpllId {}
impl Sealed for DpllId {}

//==============================================================================
// RawPredivider
//==============================================================================

/// Raw predivider for DPLLs sourced by the [`Xosc`](super::xosc::Xosc)
///
/// Represents a 10-bit value used to set the clock division factor for DPLLs
/// sourced by the `Xosc`. The actual divider can be calculated with the formula:
///
/// ```text
/// f_DPLL = f_XOSC / (2 * (raw_prediv + 1))
/// ```
///
/// This value is relevant only for a [`Dpll`] that is driven by the
/// [`Xosc`](super::xosc::Xosc). For other clock sources, the clock divider is
/// equal to 1.
pub type RawPredivider = u16;

//==============================================================================
// DpllSourceId
//==============================================================================

/// Value-level version of [`DpllSourceId`]
///
/// Indicates the clock source for a [`Dpll`]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DynDpllSourceId {
    /// The DPLL is driven by a [`Pclk`]
    Pclk,
    /// The DPLL is driven by [`Xosc`](super::xosc::Xosc)
    Xosc,
    /// The DPLL is driven by [`Xosc32k`](super::xosc32k::Xosc32k)
    Xosc32k,
}

impl From<DynDpllSourceId> for REFCLK_A {
    fn from(source: DynDpllSourceId) -> Self {
        match source {
            DynDpllSourceId::Pclk => REFCLK_A::GCLK,
            DynDpllSourceId::Xosc => REFCLK_A::REF1,
            DynDpllSourceId::Xosc32k => REFCLK_A::REF0,
        }
    }
}

/// Type-level `enum` for DPLL sources
///
/// See the documentation on [type-level enums] for more details on the
/// pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait DpllSourceId {
    /// Corresponding variant of [`DynDpllSourceId`]
    const DYN: DynDpllSourceId;
    /// Corresponding [`Pclk`] type if the DPLL source is a peripheral clock
    type Pclk;
    /// Convert the raw predivider to the actual divider
    fn predivider(raw_prediv: RawPredivider) -> u32;
}

impl<G: GclkId> DpllSourceId for G {
    const DYN: DynDpllSourceId = DynDpllSourceId::Pclk;
    type Pclk = Pclk<DpllId, G>;
    #[inline]
    fn predivider(_: RawPredivider) -> u32 {
        1
    }
}

impl DpllSourceId for XoscId {
    const DYN: DynDpllSourceId = DynDpllSourceId::Xosc;
    type Pclk = ();
    #[inline]
    fn predivider(raw_prediv: RawPredivider) -> u32 {
        2 * (1 + raw_prediv as u32)
    }
}

impl DpllSourceId for Xosc32kId {
    const DYN: DynDpllSourceId = DynDpllSourceId::Xosc32k;
    type Pclk = ();
    #[inline]
    fn predivider(_: RawPredivider) -> u32 {
        1
    }
}

//==============================================================================
// DpllToken
//==============================================================================

/// Token type required to construct a [`Dpll`] type instance.
///
/// From an [`atsamd_hal`][`crate`] external user perspective, it does not
/// contain any methods and serves only a token purpose.
///
/// Within an [`atsamd_hal`][`crate`], [`DpllToken`] struct is a low-level
/// access abstraction for HW register calls.
pub struct DpllToken(());

impl DpllToken {
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
    fn ctrla(&self) -> &DPLLCTRLA {
        &self.sysctrl().dpllctrla
    }

    #[inline]
    fn ctrlb(&self) -> &DPLLCTRLB {
        &self.sysctrl().dpllctrlb
    }

    #[inline]
    fn ratio(&self) -> &DPLLRATIO {
        &self.sysctrl().dpllratio
    }

    #[inline]
    fn status(&self) -> dpllstatus::R {
        self.sysctrl().dpllstatus.read()
    }

    /// Set the loop division, see page 168 in the Datasheet
    ///
    /// Formula for calculating the frequency:
    /// f_clk_dpll = clk_src * (LDR + 1 + (LDRFRAC / 32))
    ///
    /// `int` is including the `+ 1`,
    /// 'frac` is the same as `LDRFRAC`
    ///
    /// Write to the divider must be write synchronized
    #[inline]
    fn set_loop_div(&mut self, int: u16, frac: u8) {
        self.ratio().write(|w| unsafe {
            w.ldr().bits((int - 1) & 0xFFF);
            w.ldrfrac().bits(frac & 0xF)
        });
    }

    /// Set the clock source.
    #[inline]
    fn set_source_clock(&mut self, source: DynDpllSourceId) {
        self.ctrlb()
            .modify(|_, w| w.refclk().variant(source.into()));
    }

    /// When source is a XOSC this has effect, ignored otherwise.
    #[inline]
    fn set_source_div(&mut self, div: u16) {
        self.ctrlb()
            .modify(|_, w| unsafe { w.div().bits(div & ((1 << 10) - 1)) });
    }

    /// Ignore the lock, CLK_DPLLn is always running.
    #[inline]
    fn set_lock_bypass(&mut self, bypass: bool) {
        self.ctrlb().modify(|_, w| w.lbypass().bit(bypass));
    }

    /// Wake up fast, output the clock directly without waiting for lock.
    #[inline]
    fn set_wake_up_fast(&mut self, wuf: bool) {
        self.ctrlb().modify(|_, w| w.wuf().bit(wuf));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.ctrla().modify(|_, w| w.ondemand().bit(on_demand));
    }

    /// Check if [`Dpll`] clock is ready.
    #[inline]
    fn wait_until_ready(&self) -> nb::Result<(), Infallible> {
        if self.status().clkrdy().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    /// Check if [`Dpll`] clock is locked.
    #[inline]
    fn wait_until_locked(&self) -> nb::Result<(), Infallible> {
        if self.status().lock().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    /// Enable the [`Dpll`].
    #[inline]
    fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
    }

    /// Disable the [`Dpll`].
    #[inline]
    fn disable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().clear_bit());
    }
}

//==============================================================================
// Dpll
//==============================================================================

/// Struct representing a [`Dpll`] abstraction
///
/// It is generic over the a mode of operation (available modes: [`PclkDriven`],
/// [`XoscDriven`], [`Xosc32kDriven`])
pub struct Dpll<I>
where
    I: DpllSourceId,
{
    token: DpllToken,
    src_freq: Hertz,
    mult: u16,
    frac: u8,
    lock_bypass: bool,
    wake_up_fast: bool,
    on_demand: bool,
    pclk: I::Pclk,
    raw_prediv: RawPredivider,
}

impl<G: GclkId> Dpll<G> {
    /// Create a [`Dpll`] from a [`Pclk`]
    ///
    /// The corresponding [`Gclk`](super::gclk::Gclk) frequency must be between
    /// 32 kHz and 3.2 MHz.
    #[inline]
    pub fn from_pclk(token: DpllToken, pclk: Pclk<DpllId, G>) -> Self {
        let src_freq = pclk.freq();
        let (mult, frac) = (1, 0);
        Self {
            token,
            src_freq,
            mult,
            frac,
            lock_bypass: false,
            wake_up_fast: false,
            on_demand: true,
            pclk,
            raw_prediv: 1,
        }
    }

    /// Deconstruct the [`Dpll`], release the token, and return the [`Pclk`]
    #[inline]
    pub fn free(self) -> (DpllToken, Pclk<DpllId, G>) {
        (self.token, self.pclk)
    }
}

impl Dpll<Xosc32kId> {
    /// Create a [`Dpll`] from an [`Xosc32k`](super::xosc32k::Xosc32k)
    ///
    /// [`Increment`] the `Xosc32k` [`Enabled`] [`Counter`] to indicate it is
    /// being used by the `Dpll`
    #[inline]
    pub fn from_xosc32k<S>(token: DpllToken, xosc32k: S) -> (Self, S::Inc)
    where
        S: Source<Id = Xosc32kId> + Increment,
    {
        let src_freq = xosc32k.freq();
        let (mult, frac) = (1, 0);

        let dpll = Self {
            token,
            src_freq,
            mult,
            frac,
            lock_bypass: false,
            wake_up_fast: false,
            on_demand: true,
            pclk: (),
            raw_prediv: 1,
        };
        (dpll, xosc32k.inc())
    }

    /// Deconstruct the [`Dpll`], release the token, and [`Decrement`] the
    /// [`Xosc32k`](super::xosc32k::Xosc32k) [`Enabled`] [`Counter`]
    #[inline]
    pub fn free<S>(self, xosc32k: S) -> (DpllToken, S::Dec)
    where
        S: Source<Id = Xosc32kId> + Decrement,
    {
        (self.token, xosc32k.dec())
    }
}

impl Dpll<XoscId> {
    /// Create a [`Dpll`] from an external oscillator
    ///
    /// After division by the clock divider (see [`RawPredivider`]), the
    /// input frequency must be between 32 kHz and 3.2 MHz.
    ///
    /// [`Increment`] the `Xosc` [`Enabled`] [`Counter`] to indicate it is
    /// being used by the `Dpll`
    #[inline]
    pub fn from_xosc<S>(token: DpllToken, xosc: S, raw_prediv: RawPredivider) -> (Self, S::Inc)
    where
        S: Source<Id = XoscId> + Increment,
    {
        let src_freq = xosc.freq();
        let (mult, frac) = (1, 0);

        let dpll = Self {
            token,
            src_freq,
            mult,
            frac,
            lock_bypass: false,
            wake_up_fast: false,
            on_demand: true,
            pclk: (),
            raw_prediv,
        };
        (dpll, xosc.inc())
    }

    /// Set the raw predivider, see [`RawPredivider`]
    #[inline]
    pub fn set_raw_prediv(mut self, raw_prediv: RawPredivider) -> Self {
        self.raw_prediv = raw_prediv;
        self
    }

    /// Deconstruct the [`Dpll`], release the token, and [`Decrement`] the
    /// [`Xosc`](super::xosc::Xosc) [`Enabled`] [`Counter`]
    #[inline]
    pub fn free<S>(self, xosc: S) -> (DpllToken, S::Dec)
    where
        S: Source<Id = XoscId> + Decrement,
    {
        (self.token, xosc.dec())
    }
}

impl<I> Dpll<I>
where
    I: DpllSourceId,
{
    /// Set the [`Dpll`] loop divider
    ///
    /// Calculated as
    ///
    /// ```text
    /// f_clk_dpll = clk_src * (int + (frac / 32))
    /// ```
    ///
    /// The `+ 1` in the datasheet is not forgotten, it is handled by the
    /// underlying register write function
    ///
    /// Example 1:
    /// ```text
    /// clk_src = 2 MHz
    /// int = 50
    /// frac = 0
    ///
    /// 2 * 50 = 100 MHz
    /// ```
    /// Example 2:
    /// ```text
    /// clk_src = 32 kHz
    /// int = 3000
    /// frac = 24
    ///
    /// 0.032 * (3000 +  24/32) = 96.024 MHz
    /// ```
    #[inline]
    pub fn set_loop_div(mut self, int: u16, frac: u8) -> Self {
        self.mult = int;
        self.frac = frac;
        self
    }

    /// Set to ignore the phase-lock, CLK_DPLL is always running regardless of
    /// lock status
    #[inline]
    pub fn set_lock_bypass(mut self, bypass: bool) -> Self {
        self.lock_bypass = bypass;
        self
    }

    /// Set to skip waiting for [`Dpll`] lock before outputting clock
    #[inline]
    pub fn set_wake_up_fast(mut self, wuf: bool) -> Self {
        self.wake_up_fast = wuf;
        self
    }

    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand = on_demand;
        self
    }

    /// Return the frequency of the [`Dpll`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(
            self.src_freq.0 / I::predivider(self.raw_prediv)
                * (self.mult as u32 + self.frac as u32 / 32),
        )
    }

    /// Enables [`Dpll`] and performs assertions in local configuration
    ///
    /// - Performs HW register writes
    #[inline]
    pub fn enable(self) -> Result<EnabledDpll<I>, Self> {
        let predivider = I::predivider(self.raw_prediv);
        let input_frequency = self.src_freq.0 / predivider;
        let output_frequency = self.freq().0;

        // If Xosc mode: Predivider should be within a range <2, 2048>
        // Else: Predivider should be 1
        if (1..=2048).contains(&predivider)
            && (32_000..=2_000_000).contains(&input_frequency)
            && (48_000_000..=96_000_000).contains(&output_frequency)
        {
            unsafe { Ok(self.force_enable()) }
        } else {
            Err(self)
        }
    }

    /// Forcibly enables [`Dpll`] without additional checks in local
    /// configuration
    ///
    /// - Performs HW register writes
    #[inline]
    pub unsafe fn force_enable(mut self) -> EnabledDpll<I> {
        // Enable the specified mode
        self.token.set_source_clock(I::DYN);
        match I::DYN {
            DynDpllSourceId::Xosc => self.token.set_source_div(self.raw_prediv),
            _ => {}
        }
        // Set the loop divider ratio and other settings
        self.token.set_loop_div(self.mult, self.frac);
        self.token.set_lock_bypass(self.lock_bypass);
        self.token.set_wake_up_fast(self.wake_up_fast);
        self.token.set_on_demand(self.on_demand);
        // Enable the [`Dpll`]
        self.token.enable();
        Enabled::new(self)
    }
}

pub type EnabledDpll<I, N = U0> = Enabled<Dpll<I>, N>;

impl<I> EnabledDpll<I>
where
    I: DpllSourceId,
{
    /// Disable the [`Dpll`]
    #[inline]
    pub fn disable(mut self) -> Dpll<I> {
        self.0.token.disable();
        self.0
    }
}

impl<I, N> EnabledDpll<I, N>
where
    I: DpllSourceId,
    N: Counter,
{
    /// Check if [`Dpll`] has achieved lock
    #[inline]
    pub fn wait_until_locked(&self) -> nb::Result<(), Infallible> {
        self.0.token.wait_until_locked()
    }

    /// Check if [`Dpll`] is ready
    #[inline]
    pub fn wait_until_ready(&self) -> nb::Result<(), Infallible> {
        self.0.token.wait_until_ready()
    }
}

//==============================================================================
// Source
//==============================================================================

impl<I, N> Source for EnabledDpll<I, N>
where
    I: DpllSourceId,
    N: Counter,
{
    type Id = DpllId;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
