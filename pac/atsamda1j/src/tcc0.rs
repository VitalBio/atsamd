#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: crate::Reg<ctrlbclr::CTRLBCLR_SPEC>,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: crate::Reg<ctrlbset::CTRLBSET_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x0c - Recoverable Fault A Configuration"]
    pub fctrla: crate::Reg<fctrla::FCTRLA_SPEC>,
    #[doc = "0x10 - Recoverable Fault B Configuration"]
    pub fctrlb: crate::Reg<fctrlb::FCTRLB_SPEC>,
    #[doc = "0x14 - Waveform Extension Configuration"]
    pub wexctrl: crate::Reg<wexctrl::WEXCTRL_SPEC>,
    #[doc = "0x18 - Driver Control"]
    pub drvctrl: crate::Reg<drvctrl::DRVCTRL_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x1e - Debug Control"]
    pub dbgctrl: crate::Reg<dbgctrl::DBGCTRL_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x20 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x24 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x28 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x2c - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x30 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved_14_count: [u8; 0x04],
    #[doc = "0x38 - Pattern"]
    pub patt: crate::Reg<patt::PATT_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x3c - Waveform Control"]
    pub wave: crate::Reg<wave::WAVE_SPEC>,
    _reserved_17_per: [u8; 0x04],
    _reserved_18_cc: [u8; 0x10],
    _reserved19: [u8; 0x10],
    #[doc = "0x64 - Pattern Buffer"]
    pub pattb: crate::Reg<pattb::PATTB_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x68 - Waveform Control Buffer"]
    pub waveb: crate::Reg<waveb::WAVEB_SPEC>,
    _reserved_21_perb: [u8; 0x04],
    _reserved_22_ccb: [u8; 0x10],
}
impl RegisterBlock {
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith6_mode(&self) -> &crate::Reg<count_dith6_mode::COUNT_DITH6_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<count_dith6_mode::COUNT_DITH6_MODE_SPEC>)
        }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith5_mode(&self) -> &crate::Reg<count_dith5_mode::COUNT_DITH5_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<count_dith5_mode::COUNT_DITH5_MODE_SPEC>)
        }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith4_mode(&self) -> &crate::Reg<count_dith4_mode::COUNT_DITH4_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<count_dith4_mode::COUNT_DITH4_MODE_SPEC>)
        }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count(&self) -> &crate::Reg<count::COUNT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<count::COUNT_SPEC>)
        }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith6_mode(&self) -> &crate::Reg<per_dith6_mode::PER_DITH6_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<per_dith6_mode::PER_DITH6_MODE_SPEC>)
        }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith5_mode(&self) -> &crate::Reg<per_dith5_mode::PER_DITH5_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<per_dith5_mode::PER_DITH5_MODE_SPEC>)
        }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith4_mode(&self) -> &crate::Reg<per_dith4_mode::PER_DITH4_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<per_dith4_mode::PER_DITH4_MODE_SPEC>)
        }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per(&self) -> &crate::Reg<per::PER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<per::PER_SPEC>)
        }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6_mode(&self) -> &[crate::Reg<cc_dith6_mode::CC_DITH6_MODE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const [crate::Reg<cc_dith6_mode::CC_DITH6_MODE_SPEC>; 4])
        }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5_mode(&self) -> &[crate::Reg<cc_dith5_mode::CC_DITH5_MODE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const [crate::Reg<cc_dith5_mode::CC_DITH5_MODE_SPEC>; 4])
        }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4_mode(&self) -> &[crate::Reg<cc_dith4_mode::CC_DITH4_MODE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const [crate::Reg<cc_dith4_mode::CC_DITH4_MODE_SPEC>; 4])
        }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc(&self) -> &[crate::Reg<cc::CC_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const [crate::Reg<cc::CC_SPEC>; 4])
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith6_mode(&self) -> &crate::Reg<perb_dith6_mode::PERB_DITH6_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const crate::Reg<perb_dith6_mode::PERB_DITH6_MODE_SPEC>)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith5_mode(&self) -> &crate::Reg<perb_dith5_mode::PERB_DITH5_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const crate::Reg<perb_dith5_mode::PERB_DITH5_MODE_SPEC>)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith4_mode(&self) -> &crate::Reg<perb_dith4_mode::PERB_DITH4_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const crate::Reg<perb_dith4_mode::PERB_DITH4_MODE_SPEC>)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb(&self) -> &crate::Reg<perb::PERB_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const crate::Reg<perb::PERB_SPEC>)
        }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith6_mode(&self) -> &[crate::Reg<ccb_dith6_mode::CCB_DITH6_MODE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize)
                as *const [crate::Reg<ccb_dith6_mode::CCB_DITH6_MODE_SPEC>; 4])
        }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith5_mode(&self) -> &[crate::Reg<ccb_dith5_mode::CCB_DITH5_MODE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize)
                as *const [crate::Reg<ccb_dith5_mode::CCB_DITH5_MODE_SPEC>; 4])
        }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith4_mode(&self) -> &[crate::Reg<ccb_dith4_mode::CCB_DITH4_MODE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize)
                as *const [crate::Reg<ccb_dith4_mode::CCB_DITH4_MODE_SPEC>; 4])
        }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb(&self) -> &[crate::Reg<ccb::CCB_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize)
                as *const [crate::Reg<ccb::CCB_SPEC>; 4])
        }
    }
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLBCLR register accessor: an alias for `Reg<CTRLBCLR_SPEC>`"]
pub type CTRLBCLR = crate::Reg<ctrlbclr::CTRLBCLR_SPEC>;
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "CTRLBSET register accessor: an alias for `Reg<CTRLBSET_SPEC>`"]
pub type CTRLBSET = crate::Reg<ctrlbset::CTRLBSET_SPEC>;
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "FCTRLA register accessor: an alias for `Reg<FCTRLA_SPEC>`"]
pub type FCTRLA = crate::Reg<fctrla::FCTRLA_SPEC>;
#[doc = "Recoverable Fault A Configuration"]
pub mod fctrla;
#[doc = "FCTRLB register accessor: an alias for `Reg<FCTRLB_SPEC>`"]
pub type FCTRLB = crate::Reg<fctrlb::FCTRLB_SPEC>;
#[doc = "Recoverable Fault B Configuration"]
pub mod fctrlb;
#[doc = "WEXCTRL register accessor: an alias for `Reg<WEXCTRL_SPEC>`"]
pub type WEXCTRL = crate::Reg<wexctrl::WEXCTRL_SPEC>;
#[doc = "Waveform Extension Configuration"]
pub mod wexctrl;
#[doc = "DRVCTRL register accessor: an alias for `Reg<DRVCTRL_SPEC>`"]
pub type DRVCTRL = crate::Reg<drvctrl::DRVCTRL_SPEC>;
#[doc = "Driver Control"]
pub mod drvctrl;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "COUNT register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Count"]
pub mod count;
#[doc = "COUNT_DITH4_MODE register accessor: an alias for `Reg<COUNT_DITH4_MODE_SPEC>`"]
pub type COUNT_DITH4_MODE = crate::Reg<count_dith4_mode::COUNT_DITH4_MODE_SPEC>;
#[doc = "Count"]
pub mod count_dith4_mode;
#[doc = "COUNT_DITH5_MODE register accessor: an alias for `Reg<COUNT_DITH5_MODE_SPEC>`"]
pub type COUNT_DITH5_MODE = crate::Reg<count_dith5_mode::COUNT_DITH5_MODE_SPEC>;
#[doc = "Count"]
pub mod count_dith5_mode;
#[doc = "COUNT_DITH6_MODE register accessor: an alias for `Reg<COUNT_DITH6_MODE_SPEC>`"]
pub type COUNT_DITH6_MODE = crate::Reg<count_dith6_mode::COUNT_DITH6_MODE_SPEC>;
#[doc = "Count"]
pub mod count_dith6_mode;
#[doc = "PATT register accessor: an alias for `Reg<PATT_SPEC>`"]
pub type PATT = crate::Reg<patt::PATT_SPEC>;
#[doc = "Pattern"]
pub mod patt;
#[doc = "WAVE register accessor: an alias for `Reg<WAVE_SPEC>`"]
pub type WAVE = crate::Reg<wave::WAVE_SPEC>;
#[doc = "Waveform Control"]
pub mod wave;
#[doc = "PER register accessor: an alias for `Reg<PER_SPEC>`"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "Period"]
pub mod per;
#[doc = "PER_DITH4_MODE register accessor: an alias for `Reg<PER_DITH4_MODE_SPEC>`"]
pub type PER_DITH4_MODE = crate::Reg<per_dith4_mode::PER_DITH4_MODE_SPEC>;
#[doc = "Period"]
pub mod per_dith4_mode;
#[doc = "PER_DITH5_MODE register accessor: an alias for `Reg<PER_DITH5_MODE_SPEC>`"]
pub type PER_DITH5_MODE = crate::Reg<per_dith5_mode::PER_DITH5_MODE_SPEC>;
#[doc = "Period"]
pub mod per_dith5_mode;
#[doc = "PER_DITH6_MODE register accessor: an alias for `Reg<PER_DITH6_MODE_SPEC>`"]
pub type PER_DITH6_MODE = crate::Reg<per_dith6_mode::PER_DITH6_MODE_SPEC>;
#[doc = "Period"]
pub mod per_dith6_mode;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc;
#[doc = "CC_DITH4_MODE register accessor: an alias for `Reg<CC_DITH4_MODE_SPEC>`"]
pub type CC_DITH4_MODE = crate::Reg<cc_dith4_mode::CC_DITH4_MODE_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith4_mode;
#[doc = "CC_DITH5_MODE register accessor: an alias for `Reg<CC_DITH5_MODE_SPEC>`"]
pub type CC_DITH5_MODE = crate::Reg<cc_dith5_mode::CC_DITH5_MODE_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith5_mode;
#[doc = "CC_DITH6_MODE register accessor: an alias for `Reg<CC_DITH6_MODE_SPEC>`"]
pub type CC_DITH6_MODE = crate::Reg<cc_dith6_mode::CC_DITH6_MODE_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith6_mode;
#[doc = "PATTB register accessor: an alias for `Reg<PATTB_SPEC>`"]
pub type PATTB = crate::Reg<pattb::PATTB_SPEC>;
#[doc = "Pattern Buffer"]
pub mod pattb;
#[doc = "WAVEB register accessor: an alias for `Reg<WAVEB_SPEC>`"]
pub type WAVEB = crate::Reg<waveb::WAVEB_SPEC>;
#[doc = "Waveform Control Buffer"]
pub mod waveb;
#[doc = "PERB register accessor: an alias for `Reg<PERB_SPEC>`"]
pub type PERB = crate::Reg<perb::PERB_SPEC>;
#[doc = "Period Buffer"]
pub mod perb;
#[doc = "PERB_DITH4_MODE register accessor: an alias for `Reg<PERB_DITH4_MODE_SPEC>`"]
pub type PERB_DITH4_MODE = crate::Reg<perb_dith4_mode::PERB_DITH4_MODE_SPEC>;
#[doc = "Period Buffer"]
pub mod perb_dith4_mode;
#[doc = "PERB_DITH5_MODE register accessor: an alias for `Reg<PERB_DITH5_MODE_SPEC>`"]
pub type PERB_DITH5_MODE = crate::Reg<perb_dith5_mode::PERB_DITH5_MODE_SPEC>;
#[doc = "Period Buffer"]
pub mod perb_dith5_mode;
#[doc = "PERB_DITH6_MODE register accessor: an alias for `Reg<PERB_DITH6_MODE_SPEC>`"]
pub type PERB_DITH6_MODE = crate::Reg<perb_dith6_mode::PERB_DITH6_MODE_SPEC>;
#[doc = "Period Buffer"]
pub mod perb_dith6_mode;
#[doc = "CCB register accessor: an alias for `Reg<CCB_SPEC>`"]
pub type CCB = crate::Reg<ccb::CCB_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb;
#[doc = "CCB_DITH4_MODE register accessor: an alias for `Reg<CCB_DITH4_MODE_SPEC>`"]
pub type CCB_DITH4_MODE = crate::Reg<ccb_dith4_mode::CCB_DITH4_MODE_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith4_mode;
#[doc = "CCB_DITH5_MODE register accessor: an alias for `Reg<CCB_DITH5_MODE_SPEC>`"]
pub type CCB_DITH5_MODE = crate::Reg<ccb_dith5_mode::CCB_DITH5_MODE_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith5_mode;
#[doc = "CCB_DITH6_MODE register accessor: an alias for `Reg<CCB_DITH6_MODE_SPEC>`"]
pub type CCB_DITH6_MODE = crate::Reg<ccb_dith6_mode::CCB_DITH6_MODE_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith6_mode;
