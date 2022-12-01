use crate::clock::EicClock;
use crate::pac;

pub mod pin;

/// An External Interrupt Controller which is being configured.
pub struct ConfigurableEIC {
    eic: pac::EIC,
}

impl ConfigurableEIC {
    fn new(eic: pac::EIC) -> Self {
        Self { eic }
    }

    pub fn button_debounce_pins_ulp32k(&mut self, debounce_pins: &[pin::ExternalInterruptID]) {
        self.button_debounce_pins(debounce_pins, true)
    }

    pub fn button_debounce_pins_gclk(&mut self, debounce_pins: &[pin::ExternalInterruptID]) {
        self.button_debounce_pins(debounce_pins, false)
    }

    /// button_debounce_pins enables debouncing for the
    /// specified pins, with a configuration appropriate
    /// for debouncing physical buttons.
    fn button_debounce_pins(&mut self, debounce_pins: &[pin::ExternalInterruptID], ulp32k: bool) {
        self.eic.dprescaler.modify(|_, w| {
            if ulp32k {
                w.tickon().set_bit()
            } else {
                w.tickon().clear_bit()
            }
            .states0().set_bit()    // Require 7 0 samples to see a falling edge.
            .states1().set_bit()    // Require 7 1 samples to see a rising edge.
            .prescaler0().div16()
            .prescaler1().div16()
        });

        let mut debounceen: u32 = 0;
        for pin in debounce_pins {
            debounceen |= 1 << *pin as u32;
        }
        self.eic.debouncen.write(|w| unsafe { w.bits(debounceen) });
    }

    /// finalize enables the EIC.
    pub fn finalize(self) -> EIC {
        self.into()
    }
}

/// init_with_ulp32k initializes the EIC and wires it up to the
/// ultra-low-power 32kHz clock source. finalize() must be called
/// before the EIC is ready for use.
pub fn init_with_ulp32k(mclk: &mut pac::MCLK, _clock: &EicClock, eic: pac::EIC) -> ConfigurableEIC {
    mclk.apbamask.modify(|_, w| w.eic_().set_bit());

    eic.ctrla.modify(|_, w| w.swrst().set_bit());
    while eic.syncbusy.read().swrst().bit_is_set() {
        cortex_m::asm::nop();
    }

    // Use the low-power 32k clock.
    eic.ctrla.modify(|_, w| w.cksel().set_bit());

    ConfigurableEIC::new(eic)
}

/// init_with_gclk initializes the EIC and wires it up to the
/// gclk clock source. finalize() must be called before the EIC
/// is ready for use.
pub fn init_with_gclk(mclk: &mut pac::MCLK, _clock: &EicClock, eic: pac::EIC) -> ConfigurableEIC {
    mclk.apbamask.modify(|_, w| w.eic_().set_bit());

    eic.ctrla.modify(|_, w| w.swrst().set_bit());
    while eic.syncbusy.read().swrst().bit_is_set() {
        cortex_m::asm::nop();
    }

    // Use the gclk.
    eic.ctrla.modify(|_, w| w.cksel().clear_bit());

    ConfigurableEIC::new(eic)
}

/// A configured External Interrupt Controller.
pub struct EIC {
    eic: pac::EIC,
}

impl From<ConfigurableEIC> for EIC {
    fn from(eic: ConfigurableEIC) -> Self {
        eic.eic.ctrla.modify(|_, w| w.enable().set_bit());
        while eic.eic.syncbusy.read().enable().bit_is_set() {
            cortex_m::asm::nop();
        }

        Self { eic: eic.eic }
    }
}

/// Either a configured (enabled) or configurable (disabled) external interrupt controller.
pub trait OptionallyConfigurableEIC {
    unsafe fn eic(&self) -> &pac::EIC;
}

impl OptionallyConfigurableEIC for ConfigurableEIC {
    unsafe fn eic(&self) -> &pac::EIC {
        &self.eic
    }
}

impl OptionallyConfigurableEIC for EIC {
    unsafe fn eic(&self) -> &pac::EIC {
        &self.eic
    }
}

