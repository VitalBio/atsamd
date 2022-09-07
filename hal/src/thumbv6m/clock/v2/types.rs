//! Module defining or exporting peripheral types for the ['ahb'], ['apb'] and
//! ['pclk'] modules
//!
//! The `ahb`, `apb` and `pclk` modules each define structs that are
//! generic over a type parameter representing a peripheral. Some peripheral
//! modules already define suitable types for this purpose. For example,
//! [`sercom`] defines the [`Sercom0`], [`Sercom1`], etc. types. But other
//! peripherals are either not yet implemented in the HAL or do not define a
//! suitable type. This module defines a type for such peripherals. If/when a
//! suitable type is added for a given peripheral, the type defined here should
//! be deprecated or removed.
//!
//! [`ahb`]: super::ahb
//! [`apb`]: super::apb
//! [`pclk`]: super::pclk
//! [`sercom`]: crate::sercom
//! [`Sercom0`]: crate::sercom::Sercom0
//! [`Sercom1`]: crate::sercom::Sercom1

use crate::typelevel::Sealed;

pub use crate::sercom::{Sercom0, Sercom1, Sercom2, Sercom3};
#[cfg(any(feature = "min-samda1g", feature = "min-samd21g"))]
pub use crate::sercom::{Sercom4, Sercom5};

macro_rules! create_types {
    (
        $(
            $Type:ident
        ),+
    ) => {
        $(
            /// Marker type representing the corresponding peripheral
            ///
            /// This type is defined by and used within the [`clock`](super)
            /// module. See the the [`types`](self) module documentation for
            /// more details.
            pub enum $Type {}
            impl Sealed for $Type {}
        )+
    };
}

create_types!(Ac, AcDig, AcAna);
create_types!(Adc);
create_types!(Dac);
create_types!(Dmac);
create_types!(Dpll32k);
create_types!(Dsu);
create_types!(Eic);
create_types!(
    EvSys, EvSys0, EvSys1, EvSys2, EvSys3, EvSys4, EvSys5, EvSys6, EvSys7, EvSys8, EvSys9, EvSys10,
    EvSys11
);
create_types!(Gclk);
create_types!(Hpb0, Hpb1, Hpb2);
create_types!(NvmCtrl);
create_types!(I2S, I2S0, I2S1);
create_types!(Pac0, Pac1, Pac2);
create_types!(Pm);
create_types!(Port);
create_types!(Ptc);
create_types!(Rtc);
create_types!(SlowClk);
create_types!(SysCtrl);
create_types!(Tcc0Tcc1, Tcc0, Tcc1);
create_types!(Tcc2Tc3, Tcc2, Tc3);
create_types!(Tc4Tc5, Tc4, Tc5);
#[cfg(any(feature = "samda1", feature = "min-samd21j"))]
create_types!(Tc6Tc7, Tc6, Tc7);
create_types!(Usb);
create_types!(Wdt);
