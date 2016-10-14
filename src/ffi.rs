use ::libc;

pub const PROC_ALL_PIDS: libc::uint64_t = 1;

pub const PROC_PIDTBSDINFO: libc::c_int = 3;
pub const PROC_PIDPATHINFO_MAXSIZE: libc::uint64_t = 4096;
pub const PROC_PIDPATHINFO_MAXSIZE_U: libc::size_t = 4096;

pub const MAXCOMLEN: libc::uint64_t = 16;
pub const MAXCOMLEN_U: libc::size_t = 16;

#[cfg(target_os = "macos")]
extern "C" {
    pub fn proc_pidinfo (
        pid: libc::c_int,
        flavor: libc::c_int,
        arg: libc::uint64_t,
        buffer: *mut proc_bsdinfo,
        buffersize: libc::c_int
    ) -> libc::c_int;
    pub fn proc_pidpath (
        pid: libc::c_int,
        buffer: *const libc::c_uchar,
        buffersize: libc::uint64_t
    ) -> libc::c_int;
    pub fn proc_listpids (
        option: libc::uint64_t,
        typeinfo: libc::uint64_t,
        buffer: *const libc::c_void,
        buffersize: libc::c_int
    ) -> libc::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct proc_bsdinfo {
        pub pbi_flags: libc::uint64_t,
        pub pbi_status: libc::uint64_t,
        pub pbi_xstatus: libc::uint64_t,
        pub pbi_pid: libc::uint64_t,
        pub pbi_ppid: libc::uint64_t,
        pub pbi_uid: libc::uint64_t,
        pub pbi_gid: libc::uint64_t,
        pub pbi_ruid: libc::uint64_t,
        pub pbi_rgid: libc::uint64_t,
        pub pbi_svuid: libc::uint64_t,
        pub pbi_svgid: libc::uint64_t,
        pub rfu_1: libc::uint64_t,
        pub pbi_comm: [libc::c_char; MAXCOMLEN_U],
        pub pbi_name: [libc::c_char; 2*MAXCOMLEN_U],
        pub pbi_nfiles: libc::uint64_t,
        pub pbi_pgid: libc::uint64_t,
        pub pbi_pjobc: libc::uint64_t,
        pub e_tdev: libc::uint64_t,
        pub e_tpgid: libc::uint64_t,
        pub pbi_nice: libc::uint64_t,
        pub pbi_start_tvsec: libc::uint64_t,
        pub pbi_start_tvusec: libc::uint64_t,
}
