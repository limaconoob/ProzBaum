use ::libc;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum Pid {
  PROC_ALL_PIDS = 0x01,
  PROC_CHILD_PIDS = 0x07,
  MAXCOMLEN = 0x10,
  MAXPATHLEN = 0xFF,
  PROC_PIDPATHINFO_MAXSIZE = 4 * 0xFF,
  MAXCHILDS = 4096,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Proz {
  EPROC_CTTY = 0x01,
  EPROC_SLEADER = 0x02,
  WMESGLEN = 0x07,
  COMAPT_MAXLOGNAME = 0x0C,
}

/*
pub unsafe extern fn sysctl(name: *mut c_int, namelen: c_uint, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int

int sysctl(int *name, u_int namelen, void *oldp, size_t *oldlenp, void *newp, size_t newlen);

int mib[2];
size_t len;
struct kinfo_proc info;
mib[0] = CTL_KERN;
mib[1] = KERN_PROC;
len = sizeof(int);
sysctl(mib, 2, &info, &len, NULL, 0);
};
*/

/*
#[repr(C)]
pub struct Proc<'a>
{
}

#[repr(C)]
pub struct Session<'a>
{
}

#[repr(C)]
pub struct PcRed<'a>
{
}

#[repr(C)]
pub struct UcRed<'a>
{
}

#[repr(C)]
pub struct VmSpace<'a>
{
}

#[repr(C)]
pub struct EProc<'a, 'b>
{ e_paddr: &Proc,
  e_sess: &Session,
  e_pcred: PcRed,
  e_ucred: UcRed,
  e_vm: VmSpace,
  e_ppid: libc::pid_t,
  e_pgid: libc::pid_t,
  e_jobc: libc::c_short,
  e_tdev: libc::dev_t,
  e_tpgid: libc::pid_t,
  e_tsess: &Session,
  e_wmes: &'a [libc::c_char; WMESGLEN + 1],
  e_xsize: libc::int32_t,
  e_xrssize: libc::c_short,
  e_xccount: libc::c_short,
  e_xswrss: libc::c_short,
  e_flag: libc::int32_t,
  e_login: &'a [libc::c_char; COMAPT_MAXLOGNAME],
  e_spare: &'b [libc::int32_t; 4],
}

#[repr(C)]
pub struct SigActs<'a>
{
}

#[repr(C)]
pub struct ItimerVal
{ it_interval: libc::timeval,
  it_value: libc::timeval, }

#[repr(C)]
pub struct Pst
{ __p_forw: &Proc,
  __p_back: &Proc, }

#[repr(C)]
enum Union
{ P_Un
  { p_st1: Pst,
    __p_starttime: libc::timeval, }, }

#[repr(C)]
pub struct KpProc<'a>
{ p_un:                 Union::P_Un,
  p_vmspace:            *mut VmSpace,
  p_sigacts:            *mut SigActs,
  p_flag:               libc::c_int,
  p_stat:               libc::c_char,
  p_pid:                libc::pid_t,
  p_oppid:              libc::pid_t,
  p_dupfd:              libc::c_int,
  user_stack:           *mut libc::c_char,
  exit_thread:          *mut libc::c_void,
  p_debugger:           libc::c_int,
  sigwait:              libc::c_uint,
  p_estcpu:             libc::c_uint,
  p_cpticks:            libc::c_int,
  p_pctcpu:             libc::uint32_t,
  p_wchan:              *mut libc::c_void,
  p_wmesg:              *mut libc::c_char,
  p_swtime:             libc::c_uint,
  p_slptime:            libc::c_uint,
  p_realtimer:          ItimerVal,
  p_rtime:              libc::timeval,
  p_uticks:             libc::uint64_t,
  p_sticks:             libc::uint64_t,
  p_iticks:             libc::uint64_t,
  p_traceflag:          libc::c_int,
  p_tracep:             Vnode,
  p_siglist:            libc::c_int,
  p_textvp:             Vnode,
  p_holdcnt:            libc::c_int,
  p_sigmask:            libc::sigset_t,
  p_sigignore:          libc::sigset_t,
  p_sigcatch:           libc::sigset_t,
  p_priority:           libc::c_uchar,
  p_usrpri:             libc::c_uchar,
  p_nice:               libc::c_char,
  p_pgrp:               *mut Pgrp,
  p_addr:               *mut User,
  p_xstat:              libc::c_ushort,
  p_acflag:             libc::c_ushort,
  p_ru:                 *mut Rusage, }

#[repr(C)]
pub struct KinfoProc
{ pub kp_proc:          KpProc,
  pub kp_eproc:         EProc, }
*/

#[repr(C)]
pub struct ProcBsdInfo
{ pub pbi_flags:        libc::uint32_t,
  pub pbi_status:       libc::uint32_t,
  pub pbi_xstatus:      libc::uint32_t,
  pub pbi_pid:          libc::uint32_t,
  pub pbi_ppid:         libc::uint32_t,
  pub pbi_uid:          libc::uid_t,
  pub pbi_gid:          libc::gid_t,
  pub pbi_ruid:         libc::uid_t,
  pub pbi_rgid:         libc::gid_t,
  pub pbi_svuid:        libc::uid_t,
  pub pbi_svgid:        libc::gid_t,
  pub rfu_1:            libc::uint32_t,
  pub pbi_comm:         [libc::c_char; Pid::MAXCOMLEN as usize],
  pub pbi_name:         [libc::c_char; 2 * Pid::MAXCOMLEN as usize],
  pub pbi_nfiles:       libc::uint32_t,
  pub pbi_pgid:         libc::uint32_t,
  pub pbi_pjobc:        libc::uint32_t,
  pub e_tdev:           libc::uint32_t,
  pub e_tpgid:          libc::uint32_t,
  pub pbi_nice:         libc::uint32_t,
  pub pbi_start_tvsec:  libc::uint32_t,
  pub pbi_start_tvusec: libc::uint32_t, }

//int proc_pidinfo(int pid, int flavor, uint64_t arg,  void *buffer, int buffersize)
//int proc_listchildpids(pid_t ppid, void * buffer, int buffersize)

#[cfg(target_os = "macos")]
extern "C"
{ pub fn proc_listchildpids(_ppid: libc::pid_t, _buff: *mut libc::c_void, _size: libc::c_int) -> libc::c_int;
  pub fn proc_pidpath(_pid: libc::c_int, _buff: &[libc::c_char], _size: libc::uint32_t) -> libc::c_int;
  pub fn proc_pidinfo(_pid: libc::c_int, _flav: libc::c_int, _arg: libc::uint64_t, _buff: *mut libc::c_void, _size: libc::c_int) -> libc::c_int; }
