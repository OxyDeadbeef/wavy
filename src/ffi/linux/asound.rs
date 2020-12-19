// Copyright Jeron Aldaron Lau 2019 - 2020.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

#![allow(unsafe_code)]

use std::os::raw::{c_char, c_int, c_void};

// Link to libasound
dl_api::linker!(extern "C" Alsa "libasound.so.2" {
    fn snd_device_name_hint(
        card: c_int,
        iface: *const c_char,
        hints: *mut *mut *mut c_void,
    ) -> c_int;
    fn snd_device_name_get_hint(hint: *const c_void, id: *const c_char)
        -> *mut c_char;
    fn snd_device_name_free_hint(hints: *mut *mut c_void) -> c_int;
});

//
extern "C" {
    pub(super) fn free(ptr: *mut c_void);
}

thread_local! {
    static ALSA: Option<Alsa> = Alsa::new().ok();
}

#[path = "device_list.rs"]
pub(super) mod device_list;