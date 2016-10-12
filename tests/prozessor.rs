
extern crate top;
extern crate libc;

use self::top::prozess_macos::{proc_get_info};
use self::top::convert::{u8_to_string};
use self::top::ffi::{Pid};
use self::top::baum::{Baum, BaumBenutz};
use self::top::proc_bsdinfo::{ProcBsdInfo};

fn the_prints(info: ProcBsdInfo)
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
  println!("tvus: {}", info.pbi_start_tvusec); }
//  println!("path: {}", pathbuf);

/*
fn into_baum(baum: Baum)
{ if baum.pid > 0
  { let bonjour: Option<(ProcBsdInfo, String)> = proc_get_info(baum.pid);
    match bonjour
    { Some((info, pathbuf)) => { the_prints((info, pathbuf)); },
      _ => {}, }}
  if !baum.childs.is_empty()
  { for i in baum.childs
    { into_baum(i); }}}
*/

#[test]
fn print_infos()
{ unsafe
  { the_prints(proc_get_info(72826).unwrap());
    println!("");
    the_prints(proc_get_info(73063).unwrap());
    println!("");
    the_prints(proc_get_info(73276).unwrap());
    println!("");
    the_prints(proc_get_info(70626).unwrap()); }}

#[test]
fn compare()
{ unsafe
  { let alt_baum: Baum = Baum::new(libc::getppid());
    let neu_baum: Baum = Baum::new(libc::getpid());
    let (in_pids, out_pids) = neu_baum.vergleich(alt_baum);
//    println!("In::{:?} | Out::{:?}", in_pids, out_pids);
//    println!("Pid::{} | PPid::{}\n", libc::getpid(), libc::getppid());

    let alt_baum2: Baum = Baum::new(70402);
//    alt_baum2.anzeigt();
    libc::kill(70402, 9);
    libc::sleep(1);
    let neu_baum2: Baum = Baum::new(70402);
//    println!("");
//    neu_baum2.anzeigt();
    let (in_pids2, out_pids2) = neu_baum2.vergleich(alt_baum2); }}
//    println!("In::{:?} | Out::{:?}", in_pids2, out_pids2); }}

// LES DEUX STATUS SONT: info.pbi_status && info.pbi_nice
// Execve: (yes c ; Ctrl(Z)) ; yes 1&>/dev/null & ; bash
