
use libc;
use convert::{u8_to_u32, u8_to_u64};
use ffi::Pid;

#[repr(C)]
pub struct ProcBsdInfo {
    pub pbi_flags: libc::uint32_t,
    pub pbi_status: libc::uint32_t,
    pub pbi_xstatus: libc::uint32_t,
    pub pbi_pid: libc::uint32_t,
    pub pbi_ppid: libc::uint32_t,
    pub pbi_uid: libc::uid_t,
    pub pbi_gid: libc::gid_t,
    pub pbi_ruid: libc::uid_t,
    pub pbi_rgid: libc::gid_t,
    pub pbi_svuid: libc::uid_t,
    pub pbi_svgid: libc::gid_t,
    pub rfu_1: libc::uint32_t,
    pub pbi_comm: [libc::c_uchar; Pid::MAXCOMLEN as usize],
    pub pbi_name: [libc::c_uchar; 2 * Pid::MAXCOMLEN as usize],
    pub pbi_nfiles: libc::uint32_t,
    pub pbi_pgid: libc::uint32_t,
    pub pbi_pjobc: libc::uint32_t,
    pub e_tdev: libc::uint32_t,
    pub e_tpgid: libc::uint32_t,
    pub pbi_nice: libc::uint32_t,
    pub pbi_start_tvsec: libc::uint64_t,
    pub pbi_start_tvusec: libc::uint64_t,
}

impl ProcBsdInfo {
    pub fn new(proz: Vec<u8>) -> Self {
        let mut info = ProcBsdInfo {
            pbi_flags: u8_to_u32(&proz, 0),
            pbi_status: u8_to_u32(&proz, 4),
            pbi_xstatus: u8_to_u32(&proz, 8),
            pbi_pid: u8_to_u32(&proz, 12),
            pbi_ppid: u8_to_u32(&proz, 16),
            pbi_uid: u8_to_u32(&proz, 20),
            pbi_gid: u8_to_u32(&proz, 24),
            pbi_ruid: u8_to_u32(&proz, 28),
            pbi_rgid: u8_to_u32(&proz, 32),
            pbi_svuid: u8_to_u32(&proz, 36),
            pbi_svgid: u8_to_u32(&proz, 40),
            rfu_1: u8_to_u32(&proz, 44),
            pbi_comm: [0; Pid::MAXCOMLEN as usize],
            pbi_name: [0; 2 * Pid::MAXCOMLEN as usize],
            pbi_nfiles: u8_to_u32(&proz, 96),
            pbi_pgid: u8_to_u32(&proz, 100),
            pbi_pjobc: u8_to_u32(&proz, 104),
            e_tdev: u8_to_u32(&proz, 108),
            e_tpgid: u8_to_u32(&proz, 112),
            pbi_nice: u8_to_u32(&proz, 116),
            pbi_start_tvsec: u8_to_u64(&proz, 120),
            pbi_start_tvusec: u8_to_u64(&proz, 128),
        };
        info.pbi_comm.clone_from_slice(&proz[48..(48 + Pid::MAXCOMLEN as usize)]);
        info.pbi_name.clone_from_slice(&proz[64..(64 + 2 * Pid::MAXCOMLEN as usize)]);
        info
    }
}
