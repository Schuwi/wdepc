// Audit: DONE
use std::ptr::null_mut;

use libc::{c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

// Reference: "SAT_ATA_PASS_THROUGH16_LEN 16" [5.1]
// Reference: "Table 89" [1.2]
pub const ATA_16_LEN: usize = 16;
// Reference: "SAT_ATA_PASS_THROUGH12_LEN 12" [5.1]
// Reference: "Table 85" [1.2]
pub const ATA_12_LEN: usize = 12;
// Reference: "ATA PASS-THROUGH(16)" [1.1]
pub const ATA_16: u8 = 0x85;
// Reference: "SAT_ATA_PASS_THROUGH12 0xa1     /* clashes with MMC BLANK command */" [5.1]
// Reference: "12.2.2 ATA PASS-THROUGH (12) command" [1.2]
pub const ATA_12: u8 = 0xa1;

// Reference: "define(`SG_IO', `0x00002285')" [2.1] [2.2]
// Reference: "SG_IO 0x2285" [3.2]
pub const SG_IO: c_ulong = 0x2285;

// Reference: "SG_DXFER_NONE (-1)" [4.1]
pub const SG_DXFER_NONE: c_int = -1;
// Reference: "SG_DXFER_TO_FROM_DEV (-2)" [4.1]
pub const SG_DXFER_TO_DEV: c_int = -2;
// Reference: "SG_DXFER_FROM_DEV (-3)" [4.1]
pub const SG_DXFER_FROM_DEV: c_int = -3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// Reference: "typedef struct sg_io_hdr" [3.3] [4.1]
pub struct SgIoHdr {
    pub interface_id: c_int, // Reference: "'S' for SCSI generic (required)" [4.1]
    pub dxfer_direction: c_int,
    pub cmd_len: c_uchar,
    pub mx_sb_len: c_uchar,
    pub iovec_count: c_ushort,
    pub dxfer_len: c_uint,
    pub dxferp: *mut c_void,
    pub cmdp: *mut c_uchar,
    // Audit: "void __user *sbp;" in newer kernel versions [4.1]
    pub sbp: *mut c_uchar,
    pub timeout: c_uint,
    pub flags: c_uint,
    pub pack_id: c_int,
    pub usr_ptr: *mut c_void,
    pub status: c_uchar,
    pub masked_status: c_uchar,
    pub msg_status: c_uchar,
    pub sb_len_wr: c_uchar,
    pub host_status: c_ushort,
    pub driver_status: c_ushort,
    pub resid: c_int,
    pub duration: c_uint,
    pub info: c_uint,
}

impl Default for SgIoHdr {
    fn default() -> Self {
        // Reference: "The memset() call is pretty important, setting unused input fields to safe values." [3.1]
        SgIoHdr {
            // Reference: "'S' for SCSI generic (required)" [4.1], also [3.3]
            interface_id: 'S' as c_int,
            dxfer_direction: 0,
            cmd_len: 0,
            mx_sb_len: 0,
            iovec_count: 0,
            dxfer_len: 0,
            dxferp: null_mut(),
            cmdp: null_mut(),
            sbp: null_mut(),
            timeout: 0,
            flags: 0,
            pack_id: 0,
            usr_ptr: null_mut(),
            status: 0,
            masked_status: 0,
            msg_status: 0,
            sb_len_wr: 0,
            host_status: 0,
            driver_status: 0,
            resid: 0,
            duration: 0,
            info: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
#[allow(dead_code)]
// Reference: "Command Descriptions", "Table 92", "Table 93" [6.1] pg. 129f.
pub enum AtaCmd {
    CheckPowerMode = 0xe5,
    // Audit: Unused
    ReadLogExt = 0x2f,
    ReadLogExtDma = 0x47,
    SetFeature = 0xef,
}

impl AtaCmd {
    pub fn ck_cond(&self) -> bool {
        // Reference: "The CK_COND (Check Condition) bit may be used to request the SATL to return a copy of ATA register information in the sense data upon command completion." [1.2]
        match self {
            AtaCmd::CheckPowerMode => true,
            AtaCmd::ReadLogExt => false,
            AtaCmd::ReadLogExtDma => false,
            AtaCmd::SetFeature => false,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
#[allow(dead_code)]
// Audit: Must fit into bitmask `0b0001_1110` (4 bits)
// Reference: "12.2.2 ATA PASS-THROUGH (12) command" & "12.2.3 ATA PASS-THROUGH (16) command" [1.2]
// Reference: "(multiple_count << 5) | (protocol << 1)" [5.1]
pub enum Protocol {
    // Reference: "UDMA Data In" [1.2]
    InDma = 10 << 1,
    // Reference: "UDMA Data Out" [1.2]
    OutDma = 11 << 1,
    // Reference: "Non-data" [1.2]
    None = 3 << 1,
    // Reference: "protocol = 4;   /* PIO data-in */" [5.1]
    // Reference: "PIO Data-In" [1.2]
    PioIn = 4 << 1,
    // Reference: "PIO Data-Out" [1.2]
    PioOut = 5 << 1,
    // Reference: "DMA" [1.2]
    Dma = 6 << 1,
}

impl Protocol {
    pub fn t_dir(&self) -> u8 {
        // Reference: "bool t_dir = true;  /* false -> to device, true -> from device */" [5.1]
        // Reference: "If the T_DIR bit is set to zero, then the SATL shall transfer data from the application client to the ATA device." [1.2]
        // Reference: "If the T_DIR bit is set to one, then the SATL shall transfer data from the ATA device to the application client." [1.2]
        match self {
            Protocol::InDma => 1,
            Protocol::OutDma => 0,
            Protocol::None => 0,
            Protocol::PioIn => 1,
            Protocol::PioOut => 0,
            Protocol::Dma => 0,
        }
    }
}

// Reference: "12.2.3 ATA PASS-THROUGH (16) command" [1.2]
pub fn build_ata_passthrough16(
    cmd: AtaCmd,
    protocol: Protocol,
    feature: c_ushort,
    sector_count: c_ushort,
    sector_number: c_ushort,
    cylinder: c_uint,
) -> [u8; ATA_16_LEN] {
    let mut cdb: [u8; ATA_16_LEN] = [0; ATA_16_LEN];
    cdb[0] = ATA_16; // opcode

    // Audit: extend bit indicates 48-bit LBA, see [5.1]
    // Reference: "apt_cdb[1] = (multiple_count << 5) | (protocol << 1);" [5.1]
    // Reference: "if (extend) apt_cdb[1] |= 0x1;" [5.1]
    cdb[1] = protocol as u8 | 1; // proto, extend


    // Audit: t_length=0x2 indicates sector_count shall be used to determine the number of sectors to transfer [1.2]
    // Audit: byte_block=1 indicates that `sector_count` blocks (of 512 bytes [5.1]) shall be transferred [1.2]
                                 // off_line = 0, ck_cond = ?, t_dir = ?, byt_blok = 1, t_length = 02h(sector count)
    cdb[2] =
        0b000 << 6 | if cmd.ck_cond() { 1 << 5 } else { 0 } | protocol.t_dir() << 3 | 1 << 2 | 0x2;

    cdb[3] = (feature >> 8) as u8;
    cdb[4] = feature as u8;

    // sector_count
    cdb[5] = (sector_count >> 8) as u8;
    cdb[6] = sector_count as u8;

    // lba_low
    cdb[7] = (sector_number >> 8) as u8;
    cdb[8] = sector_number as u8;

    //lba_mid
    cdb[9] = (cylinder >> 8) as u8;
    cdb[10] = cylinder as u8;

    // lba_high
    cdb[11] = (cylinder >> 24) as u8;
    cdb[12] = (cylinder >> 16) as u8;

    // Audit: 0xa0 = 0b1010_0000
    // Audit: [1.2] describes these bits as "Obsolete" (as of SAT 1.0, see "Table 87")
    // Audit: Apparently all of the four commands in use (AtaCmd) either require these two bits to be set
    // Audit: or do not care about them at all. [6.1]
    // Reference: "Table 96" [6.1]
    // Reference: "Table 139" [6.1]
    // Reference: "Table 181" [6.1]
    // Reference: "Table 211" [6.1]
    // device
    cdb[13] = 0xa0;

    // command
    cdb[14] = cmd as u8;

    // control
    cdb[15] = 0;

    cdb
}

// Reference: "12.2.2 ATA PASS-THROUGH (12) command" [1.2]
pub fn build_ata_passthrough12(
    cmd: AtaCmd,
    protocol: Protocol,
    feature: c_ushort,
    sector_count: c_ushort,
    sector_number: c_ushort,
    cylinder: c_ushort,
) -> [u8; ATA_12_LEN] {
    let mut cdb: [u8; ATA_12_LEN] = [0; ATA_12_LEN];
    cdb[0] = ATA_12; // opcode

    // Reference: "apt12_cdb[1] = (multiple_count << 5) | (protocol << 1);" [5.1]
    // Audit: ATA PASS-THROUGH (12) does not support `extend` bit (as of SAT 1.0), see [1.2]
    cdb[1] = protocol as u8; // proto, extend = 0

    // Audit: t_length=0x2 indicates sector_count shall be used to determine the number of sectors to transfer [1.2]
    // Audit: byte_block=1 indicates that `sector_count` blocks (of 512 bytes [5.1]) shall be transferred [1.2]
                             // off_line = 0, ck_cond = ?, t_dir = ?, byt_blok = 1, t_length = 02h(sector count)
    cdb[2] =
        0b000 << 6 | if cmd.ck_cond() { 1 << 5 } else { 0 } | protocol.t_dir() << 3 | 1 << 2 | 0x2;

    // features
    cdb[3] = feature as u8;

    // sector_count
    cdb[4] = sector_count as u8;

    // lba_low
    cdb[5] = sector_number as u8;

    //lba_mid
    cdb[6] = cylinder as u8;

    // lba_high
    cdb[7] = (cylinder >> 8) as u8;

    // Audit: See build_ata_passthrough16
    // device
    cdb[8] = 0xa0;

    // command
    cdb[9] = cmd as u8;

    // reserved
    cdb[10] = 0;

    // control
    cdb[11] = 0;

    cdb
}
