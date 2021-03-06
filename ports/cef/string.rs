/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use eutil::slice_to_str;
use libc::{mod, size_t, c_int, c_ushort,c_void};
use libc::types::os::arch::c95::wchar_t;
use std::char;
use std::mem;
use std::ptr;
use std::slice;
use std::string;
use types::{cef_string_utf16_t, cef_string_utf8_t, cef_string_wide_t};
use types::{cef_string_userfree_utf16_t, cef_string_userfree_utf8_t, cef_string_userfree_wide_t};

//cef_string

#[no_mangle]
extern "C" fn string_wide_dtor(str: *mut wchar_t) {
    unsafe {
        libc::free(str as *mut c_void)
    }
}

#[no_mangle]
extern "C" fn string_utf8_dtor(str: *mut u8) {
    unsafe {
        libc::free(str as *mut c_void)
    }
}

#[no_mangle]
extern "C" fn string_utf16_dtor(str: *mut c_ushort) {
    unsafe {
        libc::free(str as *mut c_void)
    }
}

#[no_mangle]
pub extern "C" fn cef_string_userfree_wide_free(cs: *mut cef_string_userfree_wide_t) {
    cef_string_wide_clear(cs);
    unsafe {
        libc::free(cs as *mut c_void)
    }
}

#[no_mangle]
pub extern "C" fn cef_string_userfree_utf8_free(cs: *mut cef_string_userfree_utf8_t) {
    unsafe {
        cef_string_utf8_clear(cs);
        libc::free(cs as *mut c_void)
    }
}

#[no_mangle]
pub extern "C" fn cef_string_userfree_utf16_free(cs: *mut cef_string_userfree_utf16_t) {
    unsafe {
        cef_string_utf16_clear(cs);
        libc::free(cs as *mut c_void)
    }
}

#[no_mangle]
pub extern "C" fn cef_string_utf8_clear(cs: *mut cef_string_utf8_t) {
    unsafe {
        (*cs).dtor.map(|dtor| dtor((*cs).str));
        (*cs).length = 0;
        (*cs).str = 0 as *mut u8;
        (*cs).dtor = mem::transmute(0 as *const u8);
    }
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn cef_string_userfree_utf8_alloc() -> *mut cef_string_utf8_t {
    unsafe {
        libc::calloc(1, mem::size_of::<cef_string_utf8_t>() as u64) as *mut cef_string_utf8_t
    }
}

#[no_mangle]
pub extern "C" fn cef_string_utf8_set(src: *const u8, src_len: size_t, output: *mut cef_string_utf8_t, copy: c_int) -> c_int {
    cef_string_utf8_clear(output);
    unsafe {
       if copy != 0 {
           if !src.is_null() && src_len > 0 {
             (*output).str = libc::calloc(1, src_len + 1) as *mut u8;
             if (*output).str.is_null() {
                 return 0;
             }

             ptr::copy_memory((*output).str, src, src_len as uint);
             (*output).length = src_len;
             (*output).dtor = Some(string_utf8_dtor);
           }
       } else {
         (*output).str = mem::transmute(src);
         (*output).length = src_len;
         (*output).dtor = mem::transmute(0 as *const u8);
       }
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn cef_string_utf8_cmp(a: *const cef_string_utf8_t, b: *const cef_string_utf8_t) -> c_int {
    unsafe {
       slice::raw::buf_as_slice((*a).str as *const u8, (*a).length as uint, |astr:&[u8]| {
            slice::raw::buf_as_slice((*b).str as *const u8, (*b).length as uint, |bstr:&[u8]| {
                  match astr.cmp(bstr) {
                       Less => -1,
                       Equal => 0,
                       Greater => 1
                  }
            })
       })
    }
}

#[no_mangle]
pub extern "C" fn cef_string_utf8_to_utf16(src: *const u8, src_len: size_t, output: *mut cef_string_utf16_t) -> c_int {
    slice_to_str(src, src_len as uint, |result| {
        let conv = result.utf16_units().collect::<Vec<u16>>();
        cef_string_utf16_set(conv.as_ptr(), conv.len() as size_t, output, 1);
        1
    })
}

#[no_mangle]
pub extern "C" fn cef_string_utf16_to_utf8(src: *const u16, src_len: size_t, output: *mut cef_string_utf8_t) -> c_int {
    unsafe {
       slice::raw::buf_as_slice(src, src_len as uint, |ustr| {
            match string::String::from_utf16(ustr) {
                Some(str) => {
                    cef_string_utf8_set(str.as_bytes().as_ptr(), str.len() as size_t, output, 1);
                    1 as c_int
                },
                None =>  0 as c_int
            }
       })
    }
}

#[no_mangle]
pub extern "C" fn cef_string_utf16_clear(cs: *mut cef_string_utf16_t) {
    unsafe {
        (*cs).dtor.map(|dtor| dtor((*cs).str));
        (*cs).length = 0;
        (*cs).str = 0 as *mut c_ushort;
        (*cs).dtor = mem::transmute(0 as *const u8);
    }
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn cef_string_userfree_utf16_alloc() -> *mut cef_string_utf16_t {
    unsafe {
        libc::calloc(1, mem::size_of::<cef_string_utf16_t>() as u64) as *mut cef_string_utf16_t
    }
}

#[no_mangle]
pub extern "C" fn cef_string_utf16_set(src: *const c_ushort, src_len: size_t, output: *mut cef_string_utf16_t, copy: c_int) -> c_int {
    cef_string_utf16_clear(output);
    unsafe {
       if copy != 0 {
           if !src.is_null() && src_len > 0 {
             (*output).str = libc::calloc(1, (src_len + 1) * mem::size_of::<c_ushort>() as u64) as
                 *mut u16;
             if (*output).str.is_null() {
                 return 0;
             }

             ptr::copy_memory((*output).str, src, src_len as uint);
             (*output).length = src_len;
             (*output).dtor = Some(string_utf16_dtor);
           }
       } else {
         (*output).str = mem::transmute(src);
         (*output).length = src_len;
         (*output).dtor = mem::transmute(0 as *const u8);
       }
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn cef_string_utf16_cmp(a: *const cef_string_utf16_t, b: *const cef_string_utf16_t) -> c_int {
    unsafe {
       slice::raw::buf_as_slice(mem::transmute((*a).str), (*a).length as uint, |astr:&[u16]| {
            slice::raw::buf_as_slice(mem::transmute((*b).str), (*b).length as uint, |bstr:&[u16]| {
                  match astr.cmp(bstr) {
                       Less => -1,
                       Equal => 0,
                       Greater => 1
                  }
            })
       })
    }
}

#[no_mangle]
pub extern "C" fn cef_string_wide_clear(cs: *mut cef_string_wide_t) {
    unsafe {
        (*cs).dtor.map(|dtor| dtor((*cs).str));
        (*cs).length = 0;
        (*cs).str = 0 as *mut wchar_t;
        (*cs).dtor = mem::transmute(0 as *const u8);
    }
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn cef_string_userfree_wide_alloc() -> *mut cef_string_wide_t {
    unsafe {
        libc::calloc(1, mem::size_of::<cef_string_wide_t>() as u64) as *mut cef_string_wide_t
    }
}

#[no_mangle]
pub extern "C" fn cef_string_wide_set(src: *const wchar_t, src_len: size_t, output: *mut cef_string_wide_t, copy: c_int) -> c_int {
    cef_string_wide_clear(output);
    unsafe {
       if copy != 0 {
           if !src.is_null() && src_len > 0 {
             (*output).str = libc::calloc(1, (src_len + 1) * mem::size_of::<wchar_t>() as u64) as
                 *mut wchar_t;
             if (*output).str.is_null() {
                 return 0;
             }

             ptr::copy_memory((*output).str, src, src_len as uint);
             (*output).length = src_len;
             (*output).dtor = Some(string_wide_dtor);
           }
       } else {
         (*output).str = mem::transmute(src);
         (*output).length = src_len;
         (*output).dtor = mem::transmute(0 as *const u8);
       }
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn cef_string_wide_cmp(a: *const cef_string_wide_t, b: *const cef_string_wide_t) -> c_int {
    unsafe {
       slice::raw::buf_as_slice((*a).str as *const wchar_t, (*a).length as uint, |astr:&[wchar_t]| {
            slice::raw::buf_as_slice((*b).str as *const wchar_t, (*b).length as uint, |bstr:&[wchar_t]| {
                  match astr.cmp(bstr) {
                       Less => -1,
                       Equal => 0,
                       Greater => 1
                  }
            })
       })
    }
}

#[no_mangle]
pub extern "C" fn cef_string_utf8_to_wide(src: *const u8, src_len: size_t, output: *mut cef_string_wide_t) -> c_int {
    if mem::size_of::<wchar_t>() == mem::size_of::<u16>() {
         return cef_string_utf8_to_utf16(src, src_len, output as *mut cef_string_utf16_t);
    }
    slice_to_str(src, src_len as uint, |result| {
        let conv = result.chars().map(|c| c as u32).collect::<Vec<u32>>();
        cef_string_wide_set(conv.as_ptr() as *const wchar_t, conv.len() as size_t, output, 1)
    })
}

/// Wraps a borrowed reference to a UTF-16 CEF string.
pub struct CefStringRef<'a> {
    pub c_object: &'a *const cef_string_utf16_t,
}

impl<'a> CefStringRef<'a> {
    pub unsafe fn from_c_object(c_object: &'a *const cef_string_utf16_t) -> CefStringRef<'a> {
        CefStringRef {
            c_object: c_object,
        }
    }
}

#[no_mangle]
pub extern "C" fn cef_string_wide_to_utf8(src: *const wchar_t, src_len: size_t, output: *mut cef_string_utf8_t) -> c_int {
    if mem::size_of::<wchar_t>() == mem::size_of::<u16>() {
         return cef_string_utf16_to_utf8(src as *const u16, src_len, output);
    }
    unsafe {
       slice::raw::buf_as_slice(src, src_len as uint, |ustr| {
            let conv = ustr.iter().map(|&c| char::from_u32(c as u32).unwrap_or('\uFFFD')).collect::<String>();
            cef_string_utf8_set(conv.as_bytes().as_ptr(), conv.len() as size_t, output, 1)
       })
    }
}

#[no_mangle]
pub extern "C" fn cef_string_ascii_to_utf16(src: *const u8, src_len: size_t, output: *mut cef_string_utf16_t) -> c_int {
    slice_to_str(src, src_len as uint, |result| {
        let conv = result.utf16_units().collect::<Vec<u16>>();
        cef_string_utf16_set(conv.as_ptr(), conv.len() as size_t, output, 1)
    })
}

#[no_mangle]
pub extern "C" fn cef_string_ascii_to_wide(src: *const u8, src_len: size_t, output: *mut cef_string_wide_t) -> c_int {
    unsafe {
        slice::raw::buf_as_slice(src, src_len as uint, |ustr| {
            let conv = ustr.iter().map(|&c| c as u8).collect::<Vec<u8>>();
            cef_string_wide_set(conv.as_ptr() as *const wchar_t, conv.len() as size_t, output, 1)
        })
    }
}

pub fn empty_utf16_string() -> cef_string_utf16_t {
    cef_string_utf16_t {
        str: ptr::null_mut(),
        length: 0,
        dtor: None,
    }
}

