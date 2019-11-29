extern crate once_cell;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::os::unix::ffi::OsStrExt;

use once_cell::sync::OnceCell;

use super::Config;


static CONFIG: OnceCell<Config> = OnceCell::new();


#[no_mangle]
extern "C" fn osx_handle_url(url: *const c_char) {
    let c_url = unsafe {
        CStr::from_ptr(url)
    };
    let rs_url = c_url.to_str().unwrap();
    let config = CONFIG.get().unwrap();
    super::open_url_in_container(
        &config.firefox_command,
        rs_url,
        &config.container,
    );
}


pub fn main() {
    {
        let config = Config::load();
        CONFIG.set(config).expect("failed to set config global");
    }

    let args: Vec<_> = std::env::args_os()
        .map(|s| CString::new(s.as_bytes()).unwrap())
        .collect()
    ;
    let mut c_args: Vec<_> = args
        .iter()
        .map(|s| s.to_bytes_with_nul().as_ptr())
        .collect()
    ;
    let c_args_ptr = c_args.as_mut_ptr() as *mut *const c_char;
    let c_args_len = c_args.len() as c_int;
    let ret = unsafe {
        osx_objc_main(c_args_len, c_args_ptr)
    };
    std::process::exit(ret);
}

extern "C" {
    fn osx_objc_main(argc: c_int, argv: *mut *const c_char) -> c_int;
}
