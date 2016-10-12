
extern crate top;
extern crate libc;

use self::top::prozess_macos::{proc_get_info};
use self::top::convert::{u8_to_string};
use self::top::ffi::{Pid};
use self::top::baum::{Baum};
use self::top::proc_bsdinfo::{ProcBsdInfo};

fn the_prints((info, pathbuf): (ProcBsdInfo, String))
{ println!("name: {}", u8_to_string(&info.pbi_name, 2 * Pid::MAXCOMLEN as usize));
  println!("comm: {}", u8_to_string(&info.pbi_comm, Pid::MAXCOMLEN as usize));
  println!("flag: {}", info.pbi_flags);
  println!("stat: {}", info.pbi_status);
  println!("xsta: {}", info.pbi_xstatus);
  println!(" pid: {}", info.pbi_pid);
  println!("ppid: {}", info.pbi_ppid);
  println!(" uid: {}", info.pbi_uid);
  println!(" gid: {}", info.pbi_gid);
  println!("ruid: {}", info.pbi_ruid);
  println!("rgid: {}", info.pbi_rgid);
  println!("suid: {}", info.pbi_svuid);
  println!("sgid: {}", info.pbi_svgid);
  println!("pjob: {}", info.pbi_pjobc);
  println!("tdev: {}", info.e_tdev);
  println!("tgid: {}", info.e_tpgid);
  println!("nice: {}", info.pbi_nice);
  println!(" tvs: {}", info.pbi_start_tvsec);
  println!("tvus: {}", info.pbi_start_tvusec);
  println!("path: {}", pathbuf); }

fn into_baum(baum: Baum)
{ if baum.pid > 0
  { let bonjour: Option<(ProcBsdInfo, String)> = proc_get_info(baum.pid);
    match bonjour
    { Some((info, pathbuf)) => { the_prints((info, pathbuf)); },
      _ => {}, }}
  if !baum.childs.is_empty()
  { for i in baum.childs
    { into_baum(i); }}}

#[test]
fn print_infos()
{ into_baum(Baum::default()); }
