//! This module is intentionally private. Its contents are publicly exported
//! from the `v2` module, which is where the corresponding documentation will
//! appear.

use typenum::{U1, U2};

use crate::pac::{GCLK, NVMCTRL, PM, SYSCTRL};

use super::*;

/// Collection of low-level PAC structs
///
/// This struct serves to guard access to the low-level PAC structs. It places
/// them behind an `unsafe` barrier.
///
/// Normally, users trade the low-level PAC structs for the higher-level
/// `clock::v2` API. However, in some cases, the `clock::v2` API may not be
/// sufficient. In these cases, users can access the registers directly by
/// calling [`Pac::steal`] to recover the PAC structs.
pub struct Pac {
    sysctrl: SYSCTRL,
    gclk: GCLK,
    pm: PM,
}

impl Pac {
    /// Escape hatch allowing to access low-level PAC structs
    ///
    /// Consume the [`Pac`] and return the low-level PAC structs. This is
    /// useful when the `clock::v2` API does not provide a necessary feature, or
    /// when dealing with the legacy `clock::v1` API. For example, many of the
    /// `clock::v1` functions require access to the [`MCLK`] peripheral.
    ///
    /// # Safety
    ///
    /// Directly configuring clocks through the PAC API can invalidate the
    /// type-level guarantees of the `clock` module API.
    pub unsafe fn steal(self) -> (SYSCTRL, GCLK, PM) {
        (self.sysctrl, self.gclk, self.pm)
    }
}

pub struct Buses {
    pub ahb: ahb::Ahb,
    pub apb: apb::Apb,
}

/// Enabled clocks at power-on reset
///
/// This type is constructed using the [`por_state`] function, which consumes
/// the PAC-level clocking structs and returns the HAL-level clocking structs in
/// their reset state.
///
/// This type represents the clocks as they are configured at power-on reset.
/// The main clock, [`Gclk0`](gclk::Gclk0), runs at 8 MHz using the
/// [`Osc8m`](osc8m::Osc8m) without division. The ultra-low power
/// [base oscillator](osculp32k::OscUlpBase) is also enabled and running, as it
/// can never be disabled, and is used as a source to run [`Gclk2`](gclk::Gclk2)
/// without division. Wdt is configured to use [`Gclk2`](gclk::Gclk2) as a
/// source
///
/// As described in the [top-level](super::super) documentation for the `clock`
/// module, only [`Enabled`] clocks can be used as a [`Source`] for downstream
/// clocks. This struct contains all of the `Enabled` clocks at reset.
///
/// This struct also contains the [`Pac`] wrapper struct, which provides
/// `unsafe` access to the low-level PAC structs.
pub struct Clocks {
    /// Wrapper providing `unsafe` access to low-level PAC structs
    pub pac: Pac,
    /// Enabled AHB clocks
    pub ahbs: ahb::AhbClks,
    /// Enabled APB clocks
    pub apbs: apb::ApbClks,
    /// OSC8M clock, running at 8MHz
    pub osc8m: Enabled<osc8m::Osc8m, U1>,
    /// Main system clock, driven at 8 MHz by the Oscm8
    pub gclk0: Enabled<gclk::Gclk0<osc8m::Osc8mId>, U1>,
    /// Always-enabled base oscillator for the [`OscUlp1k`](osculp32k::OscUlp1k)
    /// and [`OscUlp32k`](osculp32k::OscUlp32k) clocks.
    pub osculp_base: Enabled<osculp32k::OscUlpBase, U2>,
    /// OscUlp32k clock
    pub osculp32k: Enabled<osculp32k::OscUlp32k, U1>,
    /// OscUlp32k clock
    pub osculp1k: Enabled<osculp32k::OscUlp1k>,
    /// Gclk2, driven at 32 kHz by the OscUlp32k
    pub gclk2: Enabled<gclk::Gclk2<osculp32k::OscUlp32kId>, U1>,
    /// WDT peripheral clock, driven at 32 kHz by Gclk2
    pub wdt: Enabled<pclk::Pclk<types::Wdt, gclk::Gclk2Id>, U1>,
}

/// Type-level tokens for unused clocks at power-on reset
///
/// This type is constructed using the [`por_state`] function, which consumes
/// the PAC-level clocking structs and returns the HAL-level clocking structs in
/// their reset state.
///
/// As described in the [top-level](super::super) documentation for the `clock`
/// module, token types are used to guanrantee the uniqueness of each clock. To
/// configure or enable a clock, you must provide the corresponding token.
///
/// For example, to enable the peripheral channel clock for [`Sercom1`], you
/// must provide the corresponding [`PclkToken`](pclk::PclkToken).
///
/// ```
/// use atsamd_hal::thumbv7em::clock::v2::{self as clock, pclk::Pclk};
///
/// let (buses, clocks, tokens) = clock::por_state(oscctrl, osc32kctrl, gclk, mclk, nvmctrl);
/// let pclk_sercom1 = Pclk::enable(tokens.pclks.sercom1, clocks.gclk0);
/// ```
///
/// [`Sercom1`]: crate::sercom::v2::Sercom1
pub struct Tokens {
    /// Tokens to create [`apb::ApbClk`]s
    pub apbs: apb::ApbTokens,
    /// Token to create [`dpll::Dpll`]
    pub dfll: dfll::DfllToken,
    /// Token to create [`dpll::Dpll`]
    pub dpll: dpll::DpllToken,
    /// Tokens to create [`gclkio::GclkIo`]s
    pub gclk_io: gclkio::Tokens,
    /// Tokens to create [`gclk::Gclk`]
    pub gclks: gclk::Tokens,
    /// Tokens to create [`pclk::Pclk`]s
    pub pclks: pclk::Tokens,
    /// Tokens [`xosc::Xosc`]
    pub xosc: xosc::XoscToken,
    /// Tokens [`xosc::Xosc`]
    pub xosc32k: xosc32k::Xosc32kToken,
    /// Tokens [`osc32k::Osc32k`]
    pub osc32k: osc32k::Osc32kToken,
    /// Tokens [`osc8m::Osc8m`]
    pub osc8m: osc8m::Osc8mToken,
}

/// Consume the PAC clocking structs and return a HAL-level
/// representation of the clocks at power-on reset
///
/// This function consumes the [`OSCCTRL`], [`OSC32KCTRL`], [`GCLK`] and
/// [`MCLK`] PAC structs and returns the [`Buses`], [`Clocks`] and [`Tokens`].
/// The `Buses` provide access to enable or disable the AHB and APB bus clocks.
/// The `Clocks` represent the set of [`Enabled`] clocks at reset. And the
/// `Tokens` can be used to configure and enable the remaining clocks.
///
/// For example, the following code demonstrates a number of common operations.
/// First, the PAC structs are traded for HAL types. Next, the GCLK5 token is
/// used to create an instance of [`Gclk5`], sourced from the already running
/// [`Dfll`](dfll::Dfll). The [`GclkDivider`](gclk::GclkDivider) is set to 24,
/// and `Gclk5` is [`Enabled`] with a 2 MHz output frequency. Next, `Gclk5` is
/// used as the [`Pclk`](pclk::Pclk) [`Source`] for [`Dpll0`](dpll::Dpll0). Once
/// the peripheral channel clock has been enabled, the `Dpll0` itself can be
/// created from it. The loop divider is set to 60, which raises the output
/// frequency to 120 MHz. Finally, the main clock, [`Gclk0`](gclk::Gclk0) is
/// swapped to use `Dpll0` instead of the `Dfll`.
///
/// ```
/// use atsamd_hal::thumbv7em::clock::v2::{self as clock, gclk, pclk, dpll};
///
/// let (_buses, clocks, tokens) = clock::por_state(oscctrl, osc32kctrl, gclk, mclk, nvmctrl);
/// let (gclk5, dfll) = gclk::Gclk::new(tokens.gclks.gclk5, clocks.dfll);
/// let gclk5 = gclk5.div(gclk::GclkDiv::Div(24)).enable();
/// let (pclk_dpll0, gclk5) = pclk::Pclk::enable(tokens.pclks.dpll0, gclk5);
/// let dpll0 = dpll::Dpll0::from_pclk(tokens.dpll0, pclk_dpll0)
///     .set_loop_div(60, 0)
///     .enable()
///     .unwrap_or_else(|_| panic!("Dpll did not pass assertion checks!"));
/// let (gclk0, dfll, dpll0) = clocks.gclk0.swap(dfll, dpll0);
/// ```
///
/// See the [top-level](super::super) documentation of the `clock` module for
/// more details.
#[inline]
pub fn por_state(
    sysctrl: SYSCTRL,
    gclk: GCLK,
    pm: PM,
    nvmctrl: &mut NVMCTRL,
) -> (Buses, Clocks, Tokens) {
    // Safe because no bus, clock or token struct is instantiated more than once
    // We also know that the PAC structs cannot be obtained more than once
    // without `unsafe` code
    unsafe {
        let buses = Buses {
            ahb: ahb::Ahb::new(),
            apb: apb::Apb::new(),
        };
        let pac = Pac { sysctrl, gclk, pm };

        let osc8m = Enabled::<_>::new(osc8m::Osc8m::new(osc8m::Osc8mToken::new()));
        let (gclk0, osc8m) = gclk::Gclk0::new(gclk::GclkToken::new(), osc8m);
        let gclk0 = Enabled::new(gclk0);

        let osculp_tokens = osculp32k::Tokens::new();
        let osculp_base = osculp32k::OscUlpBase::new();
        let (osculp32k, osculp_base) =
            osculp32k::OscUlp32k::new(osculp_tokens.osculp32k, osculp_base);
        let (osculp1k, osculp_base) = osculp32k::OscUlp1k::new(osculp_tokens.osculp1k, osculp_base);
        let (gclk2, osculp32k) = gclk::Gclk2::new(gclk::GclkToken::new(), osculp32k);
        let gclk2 = Enabled::<_>::new(gclk2);
        let (wdt, gclk2) = pclk::Pclk::enable(pclk::PclkToken::<_>::new(), gclk2);
        let wdt = Enabled::new(wdt);

        let clocks = Clocks {
            pac,
            ahbs: ahb::AhbClks::new(),
            apbs: apb::ApbClks::new(),
            osc8m,
            gclk0,
            osculp_base,
            osculp32k,
            osculp1k,
            gclk2,
            wdt,
        };
        let tokens = Tokens {
            apbs: apb::ApbTokens::new(),
            dfll: dfll::DfllToken::new(),
            dpll: dpll::DpllToken::new(),
            gclk_io: gclkio::Tokens::new(),
            gclks: gclk::Tokens::new(nvmctrl),
            pclks: pclk::Tokens::new(),
            xosc: xosc::XoscToken::new(),
            xosc32k: xosc32k::Xosc32kToken::new(),
            osc32k: osc32k::Osc32kToken::new(),
            osc8m: osc8m::Osc8mToken::new(),
        };
        (buses, clocks, tokens)
    }
}
