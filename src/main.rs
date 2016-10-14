extern crate baum_rs;
extern crate libc;

use std::{mem, ptr};
use std::ops::{Div, Mul};

use baum_rs::prelude as baum;

fn main() {
    unsafe {
//s     print!("rust::proc_listpids({}, {}, {}, {})", mem::size_of::<libc::uint64_t>(), mem::size_of::<libc::uint64_t>(), mem::size_of::<*const libc::c_int>(), mem::size_of::<libc::c_int>());
        print!("rust::proc_listpids({}, {}, {:p}, {})", 
               baum::PROC_ALL_PIDS, 0, ptr::null_mut::<*const libc::c_int>(), 0
              );
        let bufsize: libc::size_t = baum::proc_listpids(baum::PROC_ALL_PIDS, 0, ptr::null_mut(), 0) as libc::size_t;
        print!(" -> {} ;; {}*{}/{}", bufsize, bufsize, 2, mem::size_of::<libc::uint64_t>());
        let bufsize2: libc::size_t = bufsize.mul(&2).div(&mem::size_of::<libc::uint64_t>());
        println!(" -> {}", bufsize2);

        let mut pids: Vec<libc::uint64_t> = Vec::with_capacity(bufsize2);
        pids.resize(bufsize2, 0);
//s     print!("rust::proc_listpids({}, {}, {}, {})", mem::size_of::<libc::uint64_t>(), mem::size_of::<libc::uint64_t>(), mem::size_of::<*const libc::c_void>(), mem::size_of::<libc::c_int>());
        print!("rust::proc_listpids({}, {}, {:p}, {})",
               baum::PROC_ALL_PIDS, 0, pids.as_ptr(),
               mem::size_of::<libc::uint64_t>().mul(&pids.len()) as libc::c_int
              );
        let bufsize: libc::size_t = baum::proc_listpids(
              baum::PROC_ALL_PIDS, 0,
              pids.as_ptr() as *const libc::c_void,
              mem::size_of::<libc::uint64_t>().mul(&pids.len()) as libc::c_int
        ) as libc::size_t;
        println!(" -> {}", bufsize);
        let num_pids: libc::size_t = bufsize.div(&mem::size_of::<libc::uint64_t>());

        (0..{num_pids/2}).all(|index| {
            let tmp = *pids.get_unchecked(index);
            *pids.get_unchecked_mut(index) = *pids.get_unchecked(num_pids - index - 1);
            *pids.get_unchecked_mut(num_pids - index - 1) = tmp;
            true
        });
        (0..num_pids).all(|index| {
//s          println!("rust::my_proc_get_info({})", mem::size_of::<libc::c_int>());
            println!("rust::my_proc_get_info({})", *pids.get_unchecked(index) as libc::c_int);

            baum::proc_get_info(*pids.get_unchecked(index) as libc::c_int);
            print!("\n-------------------------------\n\n");
            true
        });
    }
}
