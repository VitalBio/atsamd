#[macro_export]
macro_rules! clocking_preset_gclk0_48mhz_gclk1_internal_32mhz {
    (
        $gclk0:expr,
        $osc8m:expr,
        $tokens:expr
    ) => {{
        use atsamd_hal::clock::v2::*;

        let osc32k = osc32k::new($tokens.osc32k).enable();
        let (gclk1, osc32k) = gclk::Gclk::new($tokens.gclks.gclk1, osc32k);
        let gclk1 = gclk1.enable();

        let (pclk_dfll, gclk1) = pclk::Pclk::enable($tokens.pclks.dfll, gclk1);
        let coarse = super::super::calibration::dfll48m_coarse_cal();
        let dfll = dfll::Dfll::in_closed_mode(
            $tokens.dfll,
            pclk_dfll,
            (48_000_000u32 / 32768) as u16,
            coarse / 4,
            10,
        )
        .enable();

        let (gclk0, osc8m, dfll) = $gclk0.swap($osc8m, dfll);
        (gclk0, gclk1, osc32k, dfll, osc8m)
    }};
}

#[macro_export]
macro_rules! clocking_preset_gclk0_48mhz_gclk1_external_32mhz {
    (
        $gclk0:expr,
        $osc8m:expr,
        $xosc32k_in:expr,
        $xosc32k_out:expr,
        $tokens:expr
    ) => {{
        use atsamd_hal::clock::v2::*;

        let xosc32k = xosc32k::from_crystal($tokens.osc32k, $xosc32k_in, $xosc32k_out).enable();
        let (gclk1, xosc32k) = gclk::Gclk::new($tokens.gclks.gclk1, xosc32k);
        let gclk1 = gclk1.enable();

        let (pclk_dfll, gclk1) = pclk::Pclk::enable($tokens.pclks.dfll, gclk1);
        let coarse = super::super::calibration::dfll48m_coarse_cal();
        let dfll = dfll::Dfll::in_closed_mode(
            $tokens.dfll,
            pclk_dfll,
            (48_000_000u32 / 32768) as u16,
            coarse / 4,
            10,
        )
        .enable();

        let (gclk0, osc8m, dfll) = $gclk0.swap($osc8m, dfll);
        (gclk0, gclk1, xosc32k, dfll, osc8m)
    }};
}
