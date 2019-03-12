#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env::args;
use std::env::args_os;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod delete;

fn main() {

    let ops : delete::fuse_operations = delete::fuse_operations {
        getattr: None,
        readlink: None,
        mknod: None,
        mkdir: None,
        unlink: None,
        rmdir: None,
        symlink: None,
        rename: None,
        link: None,
        chmod: None,
        chown: None,
        truncate: None,
        open: None,
        read: None,
        write: None,
        statfs: None,
        flush: None,
        release: None,
        fsync: None,
        setxattr: None,
        getxattr: None,
        listxattr: None,
        removexattr: None,
        opendir: None,
        readdir: None,
        releasedir: None,
        fsyncdir: None,
        init: None,
        destroy: None,
        access: None,
        create: None,
        lock: None,
        utimens: None,
        bmap: None,
        ioctl: None,
        poll: None,
        write_buf: None,
        read_buf: None,
        flock: None,
        fallocate: None,
        copy_file_range: None
    };

    let argc: i32 = args_os().len() as i32;

    let mut args: Vec<CString> = args_os().into_iter().map(|arg| CString::new(arg.as_bytes()).expect("Expected valid arg input")).collect();
    let mut argv: Vec<*const ::std::os::raw::c_char> = args.into_iter().map(| arg| arg.as_ptr()).collect();

    unsafe {
        delete::rust_fuse_main(argc, argv.as_mut_ptr() as *mut *mut ::std::os::raw::c_char, &ops);
    }

    println!("Hello, world!");
}
