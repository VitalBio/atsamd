#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80..0x100 - PRS\\[%s\\]"]
    pub prs: [PRS; 16],
    _reserved1: [u8; 0x10],
    #[doc = "0x110..0x150 - Special Function"]
    pub sfr: [crate::Reg<sfr::SFR_SPEC>; 16],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRS {
    #[doc = "0x00 - Priority A for Slave"]
    pub pras: crate::Reg<self::prs::pras::PRAS_SPEC>,
    #[doc = "0x04 - Priority B for Slave"]
    pub prbs: crate::Reg<self::prs::prbs::PRBS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PRS\\[%s\\]"]
pub mod prs;
#[doc = "SFR register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "Special Function"]
pub mod sfr;
