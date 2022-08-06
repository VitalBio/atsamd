//! # Osculp32k - Ultra Low power 32 kHz oscillator

#![allow(missing_docs)]

use typenum::U0;

use crate::pac::sysctrl::OSCULP32K;

use crate::time::Hertz;
use crate::typelevel::{Counter, Increment, PrivateIncrement, Sealed};

use super::{Enabled, Source};

//==============================================================================
// Tokens
//==============================================================================

pub struct OscUlpBaseToken(());

pub struct OscUlp1kToken(());

pub struct OscUlp32kToken(());

pub struct Tokens {
    pub osculp1k: OscUlp1kToken,
    pub osculp32k: OscUlp32kToken,
}

impl Tokens {
    /// Create a new set of tokens
    ///
    /// Safety: There must never be more than one instance of a token at any
    /// given time.
    pub(super) unsafe fn new() -> Self {
        Self {
            osculp1k: OscUlp1kToken(()),
            osculp32k: OscUlp32kToken(()),
        }
    }
}

impl OscUlpBaseToken {
    #[inline]
    fn osculp32k(&self) -> &OSCULP32K {
        unsafe { &(*crate::pac::SYSCTRL::ptr()).osculp32k }
    }

    #[inline]
    fn set_calibration(&mut self, calib: u8) {
        self.osculp32k()
            .modify(|_, w| unsafe { w.calib().bits(calib) });
    }

    #[inline]
    fn wrtlock(&mut self) {
        self.osculp32k().modify(|_, w| w.wrtlock().bit(true));
    }
}

//==============================================================================
// OscUlpBase
//==============================================================================

pub struct OscUlpBase {
    token: OscUlpBaseToken,
}

pub type EnabledOscUlpBase<N = U0> = Enabled<OscUlpBase, N>;

impl OscUlpBase {
    /// Create the ultra-low power base oscillator
    ///
    /// Safety: There must never be more than one instance of this struct at any
    /// given time.
    #[inline]
    pub(super) unsafe fn new() -> EnabledOscUlpBase {
        let token = OscUlpBaseToken(());
        Enabled::new(Self { token })
    }
}

impl<N: Counter> EnabledOscUlpBase<N> {
    /// Override the factory-default calibration value
    #[inline]
    pub fn set_calibration(&mut self, calib: u8) {
        self.0.token.set_calibration(calib);
    }

    /// Set the write-lock, which will last until POR
    ///
    /// This function sets the write-lock bit, which lasts until power-on reset.
    /// It also consumes and drops the [`XoscBase`], which destroys API access
    /// to the registers.
    #[inline]
    pub fn write_lock(mut self) {
        self.0.token.wrtlock();
    }
}

//==============================================================================
// Ids
//==============================================================================

/// Type-level variant representing the identity of the OSCULP1K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum OscUlp1kId {}

impl Sealed for OscUlp1kId {}

/// Type-level variant representing the identity of the OSCULP32K clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum OscUlp32kId {}

impl Sealed for OscUlp32kId {}

//==============================================================================
// OscUlp1k
//==============================================================================

pub struct OscUlp1k {
    #[allow(dead_code)]
    token: OscUlp1kToken,
}

pub type EnabledOscUlp1k<N = U0> = Enabled<OscUlp1k, N>;

impl OscUlp1k {
    /// Enable the 1 kHz output from OSCULP32K
    ///
    /// This clock is derived from the [`Enabled`] [`OscUlpBase`] clock.
    #[inline]
    pub(super) unsafe fn new<N: Increment>(
        token: OscUlp1kToken,
        base: EnabledOscUlpBase<N>,
    ) -> (EnabledOscUlp1k, EnabledOscUlpBase<N::Inc>) {
        (Enabled::new(Self { token }), base.inc())
    }
}

impl<N: Counter> Source for EnabledOscUlp1k<N> {
    type Id = OscUlp1kId;

    fn freq(&self) -> Hertz {
        Hertz(1024)
    }
}

//==============================================================================
// OscUlp32k
//==============================================================================

pub struct OscUlp32k {
    #[allow(dead_code)]
    token: OscUlp32kToken,
}

pub type EnabledOscUlp32k<N = U0> = Enabled<OscUlp32k, N>;

impl OscUlp32k {
    /// Enable the 32 kHz output from OSCULP32K
    ///
    /// This clock is derived from the [`Enabled`] [`OscUlpBase`] clock.
    ///
    /// ```
    /// let token = tokens.osculp.osculp32k;
    /// let (osculp1k, osculp) = OscUlp1k::enable(token, osculp);
    /// ```
    #[inline]
    pub(super) unsafe fn new<N: Increment>(
        token: OscUlp32kToken,
        base: EnabledOscUlpBase<N>,
    ) -> (EnabledOscUlp32k, EnabledOscUlpBase<N::Inc>) {
        (Enabled::new(Self { token }), base.inc())
    }
}

impl<N: Counter> Source for EnabledOscUlp32k<N> {
    type Id = OscUlp32kId;

    fn freq(&self) -> Hertz {
        Hertz(32_768)
    }
}
