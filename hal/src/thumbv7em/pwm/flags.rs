//! Flag definitions

use bitflags::bitflags;

//=============================================================================
// Interrupt flags
//=============================================================================
const OVF: u32 = 0x00_00_00_01;
const TRG: u32 = 0x00_00_00_02;
const CNT: u32 = 0x00_00_00_04;
const ERR: u32 = 0x00_00_00_08;
const UFS: u32 = 0x00_00_04_00;
const DFS: u32 = 0x00_00_08_00;
const FAULTA: u32 = 0x00_00_10_00;
const FAULTB: u32 = 0x00_00_20_00;
const FAULT0: u32 = 0x00_00_40_00;
const FAULT1: u32 = 0x00_00_80_00;
const MC0: u32 = 0x00_01_00_00;
const MC1: u32 = 0x00_02_00_00;
const MC2: u32 = 0x00_04_00_00;
const MC3: u32 = 0x00_08_00_00;
const MC4: u32 = 0x00_10_00_00;
const MC5: u32 = 0x00_20_00_00;

bitflags! {
    /// Interrupt bit flags for PWM
    ///
    /// The binary format of the underlying bits exactly matches the INTFLAG bits.
    pub struct Flags: u32 {
        const OVF = OVF;
        const TRG = TRG;
        const CNT = CNT;
        const ERR = ERR;
        const UFS = UFS;
        const DFS = DFS;
        const FAULTA = FAULTA;
        const FAULTB = FAULTB;
        const FAULT0 = FAULT0;
        const FAULT1 = FAULT1;
        const MC0 = MC0;
        const MC1 = MC1;
        const MC2 = MC2;
        const MC3 = MC3;
        const MC4 = MC4;
        const MC5 = MC5;
    }
}