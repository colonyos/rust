#![allow(dead_code)]

use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::str;

#[link(name = "cryptolib")]
extern "C" {
    fn prvkey() -> *const c_char;
    fn id(p: *const c_char) -> *const c_char;
    fn sign(msg: *const c_char, p: *const c_char) -> *const c_char;
    fn hash(s: *const c_char) -> *const c_char;
    fn recoverid(msg: *const c_char, p: *const c_char) -> *const c_char;
}

pub fn gen_prvkey() -> String {
    let c_buf: *const c_char = unsafe { prvkey() };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();

    String::from(str_slice)
}

pub fn gen_id(p: &String) -> String {
    let p_cstr = CString::new(p.clone()).unwrap();

    let c_buf: *const c_char = unsafe { id(p_cstr.as_ptr()) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();

    String::from(str_slice)
}

pub fn gen_signature(msg: &String, p: &String) -> String {
    let msg_cstr = CString::new(msg.clone()).unwrap();
    let p_cstr = CString::new(p.clone()).unwrap();

    let c_buf: *const c_char = unsafe { sign(msg_cstr.as_ptr(), p_cstr.as_ptr()) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();

    String::from(str_slice)
}

pub fn gen_hash(s: &String) -> String {
    let s_cstr = CString::new(s.clone()).unwrap();

    let c_buf: *const c_char = unsafe { hash(s_cstr.as_ptr()) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();

    String::from(str_slice)
}

pub fn recid(msg: &String, s: &String) -> String {
    let msg_cstr = CString::new(msg.clone()).unwrap();
    let s_cstr = CString::new(s.clone()).unwrap();

    let c_buf: *const c_char = unsafe { recoverid(msg_cstr.as_ptr(), s_cstr.as_ptr()) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();

    String::from(str_slice)
}
