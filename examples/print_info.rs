extern crate baum;
extern crate libc;

use baum::convert::u8_to_string;
use baum::ffi::Pid;
use baum::proc_bsdinfo::ProcBsdInfo;
use baum::baum::{Baum, BaumBenutz};
use baum::process_macos::proc_get_info;

/// Affiche les informations sur le pid parent
fn main() {
    unsafe {
        let pid = libc::getppid();
        let baum: Baum = Baum::new(pid);
        baum.anzeigt();

        let proz: Option<ProcBsdInfo> = proc_get_info(pid);
        match proz {
            Some(info) => {
                println!("\n------------------------------\n");
                println!("name: {}",
                         u8_to_string(&info.pbi_name, 2 * Pid::MAXCOMLEN as usize));
                println!("comm: {}",
                         u8_to_string(&info.pbi_comm, Pid::MAXCOMLEN as usize));
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
            }
            None => {
                println!("ERROR: Can't get infos of this pid");
            }
        };
    }
}
