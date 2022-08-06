//! # APBx bus clocks
//!
//! This module provides abstractions allowing to deal with a synchronous
//! clocking domain, specifically modules clocked via APB bus. It provides type
//! representation for disabled and enabled synchronous clocks available through
//! APB bus and means of switching.
//!
//! - [`ApbToken<T>`] type represents a disabled clock for a peripheral of type
//!   `T`: [`ApbType`]
//! - [`ApbClk<T>`] type represents an enabled clock for a peripheral of type
//!   `T:` [`ApbType`]
//!
//! One can enable a peripheral `T` synchronous clock via
//! [`ApbToken<T>::enable`] `->` [`ApbClk<T>`] method.
//!
//! One can disable a peripheral `T` synchronous clock via
//! [`ApbClk<T>::disable`] `->` [`ApbToken<T>`] method.
//!
//! Clocks in a default state are provided
//! - in an instance of a struct [`ApbClks`]
//! - in a field [`crate::clock::v2::Tokens::apbs`]
//! - in a return value of [`crate::clock::v2::retrieve_clocks`]

use core::marker::PhantomData;

use bitflags::bitflags;
use paste::paste;

use crate::pac::{pm, PM};

use crate::typelevel::Sealed;

use super::types::*;

//==============================================================================
// Registers
//==============================================================================

/// APB mask controller
///
/// This struct mediates access to the APB `MASK` registers. Each bit in the
/// APB `MASK` registers is represented as a type-level variant of [`ApbId`].
/// And each APB clock is represented as either an `ApbToken<A>` or an
/// `ApbClk<A>`, where `A: ApbId`. `ApbClk` represents an enabled APB clock,
/// while `ApbToken` represents a disabled APB clock.
///
/// Use the [`enable`](self::enable) and [`disable`](self::disable) methods to
/// convert tokens into clocks and vice versa.
pub struct Apb(());

impl Apb {
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn pm(&self) -> &pm::RegisterBlock {
        unsafe { &*PM::ptr() }
    }

    #[inline]
    fn apbamask(&mut self) -> &pm::APBAMASK {
        &self.pm().apbamask
    }

    #[inline]
    fn apbbmask(&mut self) -> &pm::APBBMASK {
        &self.pm().apbbmask
    }

    #[inline]
    fn apbcmask(&mut self) -> &pm::APBCMASK {
        &self.pm().apbcmask
    }

    #[inline]
    fn enable_mask(&mut self, mask: DynApbMask) {
        unsafe {
            match mask {
                DynApbMask::A(mask) => {
                    self.apbamask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
                DynApbMask::B(mask) => {
                    self.apbbmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
                DynApbMask::C(mask) => {
                    self.apbcmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
            }
        }
    }

    #[inline]
    fn disable_mask(&mut self, mask: DynApbMask) {
        unsafe {
            match mask {
                DynApbMask::A(mask) => {
                    self.apbamask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
                DynApbMask::B(mask) => {
                    self.apbbmask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
                DynApbMask::C(mask) => {
                    self.apbcmask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
            }
        }
    }

    /// Enable the corresponding APB clock
    ///
    /// Consume an [`ApbToken`], enable the corresponding APB clock and return
    /// an [`ApbClk`]. The `ApbClk` represents proof that the corresponding APB
    /// clock has been enabled.
    #[inline]
    pub fn enable<A: ApbId>(&mut self, token: ApbToken<A>) -> ApbClk<A> {
        self.enable_mask(A::DYN.into());
        ApbClk::new(token)
    }

    /// Disable the corresponding APB clock
    ///
    /// Consume the [`ApbClk`], disable the corresponding APB clock and return
    /// the [`ApbToken`].
    #[inline]
    pub fn disable<A: ApbId>(&mut self, clock: ApbClk<A>) -> ApbToken<A> {
        self.disable_mask(A::DYN.into());
        clock.free()
    }
}

//==============================================================================
// DynApbId & DynApbMask
//==============================================================================

/// Selection of APB register masks
///
/// The mask within each variant is a [`bitflags`] struct with a binary
/// representation matching the corresponding APB `MASK` register.
#[allow(missing_docs)]
pub enum DynApbMask {
    A(DynApbAMask),
    B(DynApbBMask),
    C(DynApbCMask),
}

macro_rules! define_dyn_apb_id_masks {
    (
        $(
            $Reg:ident {
                $(
                    $( #[$( $cfg:tt )+] )?
                    $Type:ident = $BIT:literal,
                )+
            }
        )+
    ) => {
        /// Value-level `enum` of all APB clocks
        ///
        /// This is the value-level version of the [type-level enum] [`AhbId`].
        ///
        /// [type-level enum]: crate::typelevel#type-level-enum
        #[repr(u8)]
        pub enum DynApbId {
            $(
                $(
                    $( #[$( $cfg )+] )?
                    #[allow(missing_docs)]
                    $Type,
                )+
            )+
        }

        $(
            $(
                $( #[$( $cfg )+] )?
                impl ApbId for $Type {
                    const DYN: DynApbId = DynApbId::$Type;
                }
            )+
        )+

        paste! {
            $(
                bitflags! {
                    #[
                        doc =
                            "APB bridge `" $Reg "` register mask\n"
                            "\n"
                            "This is a [`bitflags`] struct with a binary representation "
                            "that exactly matches the `APB" $Reg "MASK` register."
                    ]
                    pub struct [<DynApb $Reg Mask>]: u32 {
                        $(
                            $( #[$( $cfg )+] )?
                            #[allow(missing_docs)]
                            const [<$Type:upper>] = 1 << $BIT;
                        )+
                    }
                }

            )+

            impl From<DynApbId> for DynApbMask {
                #[inline]
                fn from(id: DynApbId) -> Self {
                    use DynApbId::*;
                    match id {
                        $(
                            $(
                                $( #[$( $cfg )+] )?
                                $Type => DynApbMask::$Reg([<DynApb $Reg Mask>]::[<$Type:upper>]),
                            )+
                        )+
                    }
                }
            }
        }
    };
}

define_dyn_apb_id_masks!(
    A {
        Pac0 = 0,
        Pm = 1,
        SysCtrl = 2,
        Gclk = 3,
        Wdt = 4,
        Rtc = 5,
        Eic = 6,
    }
    B {
        Pac1 = 0,
        Dsu = 1,
        NvmCtrl = 2,
        Port = 3,
        Dmac = 4,
        Usb = 5,
    }
    C {
        Pac2 = 0,
        EvSys = 1,
        Sercom0 = 2,
        Sercom1 = 3,
        Sercom2 = 4,
        Sercom3 = 5,
        #[cfg(feature = "min-samd21g")]
        Sercom4 = 6,
        #[cfg(feature = "min-samd21g")]
        Sercom5 = 7,
        Tcc0 = 8,
        Tcc1 = 9,
        Tcc2 = 10,
        Tc3 = 11,
        Tc4 = 12,
        Tc5 = 13,
        #[cfg(feature = "min-samd21j")]
        Tc6 = 14,
        #[cfg(feature = "min-samd21j")]
        Tc7 = 15,
        Adc = 16,
        Ac = 17,
        Dac = 18,
        Ptc = 19,
        I2S = 20,
        //Ac1 = 21, Not supported?
        //Tcc3 = 24, Not supported?
    }
);

//==============================================================================
// ApbId
//==============================================================================

/// Type-level `enum` for APB clocks
///
/// See the documentation on [type-level enums] for more details on the pattern.
/// The value-level equivalent is [`DynApbId`].
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait ApbId: Sealed {
    /// Corresponding [`DynApbId`] bit mask
    const DYN: DynApbId;
}

//==============================================================================
// ApbToken
//==============================================================================

/// A type representing a synchronous peripheral clock in a disabled state
pub struct ApbToken<A: ApbId> {
    id: PhantomData<A>,
}

impl<A: ApbId> ApbToken<A> {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It is
    /// being provided by a framework in a [`ApbClks`] struct instance
    #[inline]
    unsafe fn new() -> Self {
        ApbToken { id: PhantomData }
    }
}

//==============================================================================
// ApbClk
//==============================================================================

/// A type representing a synchronous peripheral clock in an enabled state
pub struct ApbClk<A: ApbId> {
    token: ApbToken<A>,
}

impl<A: ApbId> ApbClk<A> {
    #[inline]
    fn new(token: ApbToken<A>) -> Self {
        ApbClk { token }
    }

    #[inline]
    fn free(self) -> ApbToken<A> {
        self.token
    }
}

//==============================================================================
// ApbTokens
//==============================================================================

#[allow(missing_docs)]
pub struct ApbTokens {
    pub usb: ApbToken<Usb>,
    pub ev_sys: ApbToken<EvSys>,
    pub sercom0: ApbToken<Sercom0>,
    pub sercom1: ApbToken<Sercom1>,
    pub sercom2: ApbToken<Sercom2>,
    pub sercom3: ApbToken<Sercom3>,
    #[cfg(feature = "min-samd21g")]
    pub sercom4: ApbToken<Sercom4>,
    #[cfg(feature = "min-samd21g")]
    pub sercom5: ApbToken<Sercom5>,
    pub tcc0: ApbToken<Tcc0>,
    pub tcc1: ApbToken<Tcc1>,
    pub tcc2: ApbToken<Tcc2>,
    pub tc3: ApbToken<Tc3>,
    pub tc4: ApbToken<Tc4>,
    pub tc5: ApbToken<Tc5>,
    #[cfg(feature = "min-samd21j")]
    pub tc6: ApbToken<Tc6>,
    #[cfg(feature = "min-samd21j")]
    pub tc7: ApbToken<Tc7>,
    pub adc: ApbToken<Adc>,
    pub ac: ApbToken<Ac>,
    pub dac: ApbToken<Dac>,
    pub i2s: ApbToken<I2S>,
    // pub ac1: ApbToken<Ac1>,
    // pub tcc3: ApbToken<Tcc3>,
}

impl ApbTokens {
    pub(super) unsafe fn new() -> Self {
        Self {
            usb: ApbToken::new(),
            ev_sys: ApbToken::new(),
            sercom0: ApbToken::new(),
            sercom1: ApbToken::new(),
            sercom2: ApbToken::new(),
            sercom3: ApbToken::new(),
            #[cfg(feature = "min-samd21g")]
            sercom4: ApbToken::new(),
            #[cfg(feature = "min-samd21g")]
            sercom5: ApbToken::new(),
            tcc0: ApbToken::new(),
            tcc1: ApbToken::new(),
            tcc2: ApbToken::new(),
            tc3: ApbToken::new(),
            tc4: ApbToken::new(),
            tc5: ApbToken::new(),
            #[cfg(feature = "min-samd21j")]
            tc6: ApbToken::new(),
            #[cfg(feature = "min-samd21j")]
            tc7: ApbToken::new(),
            adc: ApbToken::new(),
            ac: ApbToken::new(),
            dac: ApbToken::new(),
            i2s: ApbToken::new(),
            //ac1: ApbToken::new(),
            //tcc3: ApbToken::new(),
        }
    }
}

//==============================================================================
// ApbClks
//==============================================================================

#[allow(missing_docs)]
pub struct ApbClks {
    pub pac0: ApbClk<Pac0>,
    pub pm: ApbClk<Pm>,
    pub sys_ctrl: ApbClk<SysCtrl>,
    pub gclk: ApbClk<Gclk>,
    pub wdt: ApbClk<Wdt>,
    pub rtc: ApbClk<Rtc>,
    pub eic: ApbClk<Eic>,
    pub pac1: ApbClk<Pac1>,
    pub dsu: ApbClk<Dsu>,
    pub nvm_ctrl: ApbClk<NvmCtrl>,
    pub port: ApbClk<Port>,
    pub dmac: ApbClk<Dmac>,
    pub pac2: ApbClk<Pac2>,
}

impl ApbClks {
    #[inline]
    pub(super) unsafe fn new() -> Self {
        ApbClks {
            pac0: ApbClk::new(ApbToken::new()),
            pm: ApbClk::new(ApbToken::new()),
            sys_ctrl: ApbClk::new(ApbToken::new()),
            gclk: ApbClk::new(ApbToken::new()),
            wdt: ApbClk::new(ApbToken::new()),
            rtc: ApbClk::new(ApbToken::new()),
            eic: ApbClk::new(ApbToken::new()),
            pac1: ApbClk::new(ApbToken::new()),
            dsu: ApbClk::new(ApbToken::new()),
            nvm_ctrl: ApbClk::new(ApbToken::new()),
            port: ApbClk::new(ApbToken::new()),
            dmac: ApbClk::new(ApbToken::new()),
            pac2: ApbClk::new(ApbToken::new()),
        }
    }
}
