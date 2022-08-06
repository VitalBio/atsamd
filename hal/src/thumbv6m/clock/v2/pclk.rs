//! # Pclk - Peripheral Channel (Clock)
//!
//! Peripheral clocks serve as a last element in a chain within a clocking
//! system and are directly associated with a specific peripheral in a 1:1
//! manner. Some of them are reserved for clocking system internal purposes,
//! like reference clock for Dfll or Dpll.
//!
//! Every [`Pclk`] can be powered by any instantiated and enabled
//! [`Gclk`][`super::gclk::Gclk`].
//!
//! Abstractions representing peripherals that depend on a configured
//! corresponding [`Pclk`] instance should consume it and release it upon
//! destruction. Thus, it is possible to freeze adequate part of the clocking
//! tree that running peripheral depends on.

use core::marker::PhantomData;

use paste::paste;
use seq_macro::seq;

use crate::pac;
use crate::pac::gclk::clkctrl::GEN_A;

use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment, Sealed};

use super::gclk::{DynGclkId, GclkId};
use super::Source;

//==============================================================================
// PclkToken
//==============================================================================

/// Token type required to construct a [`Pclk`] type instance.
///
/// From a [`atsamd_hal`][`crate`] external user perspective, it does not
/// contain any methods and serves only a token purpose.
///
/// Within a [`atsamd_hal`][`crate`], [`PclkToken`] struct is a low-level access
/// abstraction for HW register calls.
pub struct PclkToken<P: PclkId> {
    pclk: PhantomData<P>,
}

impl<P: PclkId> PclkToken<P> {
    /// Create a new instance of [`PclkToken`]
    ///
    /// # Safety
    ///
    /// Users must never create two simultaneous instances of this `struct` with
    /// the same [`PclkType`]
    #[inline]
    pub(super) unsafe fn new() -> Self {
        PclkToken { pclk: PhantomData }
    }

    #[inline]
    fn gclk(&self) -> &pac::gclk::RegisterBlock {
        unsafe { &*pac::GCLK::ptr() }
    }

    /// Provide access to clkctrl, primary control interface for Pclk
    #[inline]
    fn clkctrl(&self) -> &pac::gclk::CLKCTRL {
        &self.gclk().clkctrl
    }

    /// Set a clock as the [`Pclk`] source
    #[inline]
    fn set_source(&mut self, source: DynPclkSourceId) {
        self.clkctrl().modify(|_, w| unsafe {
            w.id().bits(P::DYN as u8);
            w.gen().variant(source.into())
        });
    }

    /// Enable the [`Pclk`]
    #[inline]
    fn enable(&mut self) {
        self.clkctrl().modify(|_, w| unsafe {
            w.id().bits(P::DYN as u8);
            w.clken().set_bit()
        });
    }

    /// Disable the [`Pclk`]
    #[inline]
    fn disable(&mut self) {
        self.clkctrl().modify(|_, w| unsafe {
            w.id().bits(P::DYN as u8);
            w.clken().clear_bit()
        });
    }
}

//==============================================================================
// PclkId types
//==============================================================================

/// Module containing only the types implementing [`PclkId`]
///
/// Because there are so many types that implement `PclkId`, it is helpful to
/// have them defined in a separate module, so that you can import all of them
/// using a wildcard (`*`) without importing anything else, i.e.
///
/// ```
/// use pclk::ids::*;
/// ```
pub mod ids {

    pub use crate::sercom::{Sercom0, Sercom1, Sercom2, Sercom3};
    #[cfg(feature = "min-samd21g")]
    pub use crate::sercom::{Sercom4, Sercom5};

    pub use super::super::dfll::DfllId;
    pub use super::super::dpll::DpllId;
    pub use super::super::gclk::{
        Gclk0Id, Gclk1Id, Gclk2Id, Gclk3Id, Gclk4Id, Gclk5Id, Gclk6Id, Gclk7Id,
    };
    #[cfg(feature = "min-samd21j")]
    pub use super::super::types::Tc6Tc7;
    pub use super::super::types::{
        AcAna, AcDig, Adc, Dac, Dpll32k, Eic, EvSys0, EvSys1, EvSys10, EvSys11, EvSys2, EvSys3,
        EvSys4, EvSys5, EvSys6, EvSys7, EvSys8, EvSys9, Rtc, SlowClk, Tc4Tc5, Tcc0Tcc1, Tcc2Tc3,
        Usb, Wdt, I2S0, I2S1,
    };
}

use ids::*;

/// Append the list of all [`PclkId`] types and `snake_case` id names to the
/// arguments of a macro call
///
/// This macro will perform the embedded macro call with a list of tuples
/// appended to the arguments. Each tuple contains a type implementing
/// [`PclkId`], its corresponding `PCHCTRL` register index, and the `snake_case`
/// name of the corresponding token in the [`pclk::Tokens`](Tokens) struct.
///
/// **Note:** The entries within [`DynPclkId`] do not match the type names.
/// Rather, they match the `snake_case` names converted to `CamelCase`.
///
/// An optional attribute is added just before each tuple. These are mainly used
/// to declare the conditions under which the corresponding peripheral exists.
/// For example, `Sercom6` and `Sercom7` are tagged with
/// `#[cfg(feature = "min-samd51n")]`.
///
/// The example below shows the pattern that should be used to match against the
/// appended tokens.
///
/// ```
/// macro_rules! some_macro {
///     (
///         $first_arg:tt,
///         $second_arg:tt
///         $(
///             $( #[$cfg:meta] )?
///             ($Type:ident = $N:literal, $Id:ident)
///         )+
///     ) =>
///     {
///         // implementation here ...
///     }
/// }
///
/// with_pclk_types_ids!(some_macro!(first, second));
/// ```
macro_rules! with_pclk_types_ids {
    ( $some_macro:ident ! ( $( $args:tt )* ) ) => {
        $some_macro!(
            $( $args )*
            (DfllId = 0, dfll)
            (DpllId = 1, dpll)
            (Dpll32k = 2, dpll32k)
            (Wdt = 3, wdt)
            (Rtc = 4, rtc)
            (Eic = 5, eic)
            (Usb = 6, usb)
            (EvSys0 = 7, ev_sys0)
            (EvSys1 = 8, ev_sys1)
            (EvSys2 = 9, ev_sys2)
            (EvSys3 = 10, ev_sys3)
            (EvSys4 = 11, ev_sys4)
            (EvSys5 = 12, ev_sys5)
            (EvSys6 = 13, ev_sys6)
            (EvSys7 = 14, ev_sys7)
            (EvSys8 = 15, ev_sys8)
            (EvSys9 = 16, ev_sys9)
            (EvSys10 = 17, ev_sys10)
            (EvSys11 = 18, ev_sys11)
            (SlowClk = 19, slow)
            (Sercom0 = 20, sercom0)
            (Sercom1 = 21, sercom1)
            (Sercom2 = 22, sercom2)
            (Sercom3 = 23, sercom3)
            #[cfg(feature = "min-samd21g")]
            (Sercom4 = 24, sercom4)
            #[cfg(feature = "min-samd21g")]
            (Sercom5 = 25, sercom5)
            (Tcc0Tcc1 = 26, tcc0_tcc1)
            (Tcc2Tc3 = 27, tcc2_tc3)
            (Tc4Tc5 = 28, tc4_tc5)
            #[cfg(feature = "min-samd21j")]
            (Tc6Tc7 = 29, tc6_tc7)
            (Adc = 30, adc)
            (AcDig = 31, ac_dig)
            (AcAna = 32, ac_ana)
            (Dac = 33, dac)
            // (Ptc = 34, ptc) Not supported?
            (I2S0 = 35, i2s0)
            (I2S1 = 36, i2s1)
            // (Tcc3 = 37, tcc3) Not supported?
        );
    };
}

pub(super) use with_pclk_types_ids;

//==============================================================================
// PclkId
//==============================================================================

/// Type-level `enum` for the 48 peripheral channel clock variants
pub trait PclkId: Sealed {
    /// Corresponding variant of [`DynPclkId`]
    const DYN: DynPclkId;
}

macro_rules! pclk_id {
    (
        $(
            $( #[$cfg:meta] )?
            ($Type:ident = $N:literal, $id:ident)
        )+
    ) => {
        paste! {
            /// Value-level `enum` of all peripheral channel clocks
            ///
            /// This is the value-level equivalent of the [type-level enum]
            /// [`PclkId`]. When cast to an integer type, like `u8`, each variant
            /// of this `enum` maps to the corresponding index in the array of
            /// `PCHCTRL` registers
            ///
            /// [type-level enum]: crate::typelevel#type-level-enum
            #[allow(missing_docs)]
            pub enum DynPclkId {
                $(
                    $( #[$cfg] )?
                    [<$id:camel>] = $N,
                )+
            }

            $(
                $( #[$cfg] )?
                impl PclkId for $Type {
                    const DYN: DynPclkId = DynPclkId::[<$id:camel>];
                }
            )+
        }
    };
}

with_pclk_types_ids!(pclk_id!());

//==============================================================================
// PclkSourceId
//==============================================================================

/// Value-level version of [`PclkSourceId`]
///
/// Peripheral channel clocks must be sourced from a GCLK, so `DynPclkSourceId`
/// is just a type alias for [`DynGclkId`]
pub type DynPclkSourceId = DynGclkId;

/// Convert from [`DynPclkSourceId`] to the equivalent [PAC](crate::pac) type
impl From<DynPclkSourceId> for GEN_A {
    fn from(source: DynPclkSourceId) -> Self {
        use DynGclkId::*;
        use GEN_A::*;
        seq!(N in 0..=7 {
            match source {
                #(
                    Gclk~N => GCLK~N,
                )*
            }
        })
    }
}

/// Type-level `enum` for PCLK sources
///
/// [`Pclk`]s can only be driven by [`Gclk`]s, so the only valid variants are
/// [`GclkId`]s. See the documentation on [type-level enums] for more details
/// on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait PclkSourceId: GclkId {}

impl<G: GclkId> PclkSourceId for G {}

//==============================================================================
// Pclk - Peripheral Channel Clock
//==============================================================================

/// Struct representing a [`Pclk`] abstraction
///
/// It is generic over:
/// - a peripheral it is bound to via concept of [`PclkType`]
/// - a clock source ([`PclkSourceMarker`]; variants are provided through
///   [`marker::Gclk0`], [`marker::Gclk1`], `marker::GclkX` types)
pub struct Pclk<P, I>
where
    P: PclkId,
    I: PclkSourceId,
{
    token: PclkToken<P>,
    src: PhantomData<I>,
    freq: Hertz,
}

impl<P, I> Pclk<P, I>
where
    P: PclkId,
    I: PclkSourceId,
{
    /// Enable a peripheral channel clock
    ///
    /// Increase the clock source user counter
    #[inline]
    pub fn enable<S>(mut token: PclkToken<P>, gclk: S) -> (Self, S::Inc)
    where
        S: Source<Id = I> + Increment,
    {
        let freq = gclk.freq();
        token.set_source(I::DYN);
        token.enable();
        let pclk = Pclk {
            token,
            src: PhantomData,
            freq,
        };
        (pclk, gclk.inc())
    }

    /// Disable the peripheral channel clock
    ///
    /// Decrease the clock source user counter
    #[inline]
    pub fn disable<S>(mut self, gclk: S) -> (PclkToken<P>, S::Dec)
    where
        S: Source<Id = I> + Decrement,
    {
        self.token.disable();
        (self.token, gclk.dec())
    }

    /// Return the [`Pclk`] frequency
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }
}

impl<P, I> Sealed for Pclk<P, I>
where
    P: PclkId,
    I: PclkSourceId,
{
}

//==============================================================================
// Tokens
//==============================================================================

macro_rules! define_pclk_tokens_struct {
    (
        $( #[$docs:meta] )?
        $Tokens:ident
        $(
            $( #[$cfg:meta] )?
            ($Type:ident = $_:literal, $id:ident)
        )+
    ) =>
    {
        $( #[$docs] )?
        #[allow(missing_docs)]
        pub struct $Tokens {
            $(
                $( #[$cfg] )?
                pub $id: PclkToken<$Type>,
            )+
        }

        impl $Tokens {
            #[inline]
            pub(super) fn new() -> Self {
                unsafe {
                    $Tokens {
                        $(
                            $( #[$cfg] )?
                            $id: PclkToken::new(),
                        )+
                    }
                }
            }
        }
    };
}

pub(super) use define_pclk_tokens_struct;

with_pclk_types_ids!(define_pclk_tokens_struct!(
    /// Struct containing all possible peripheral clock tokens
    Tokens
));
