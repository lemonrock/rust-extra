// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(core_intrinsics)]
#![feature(associated_consts)]


extern crate core;
extern crate libc;


use ::std::fs::Metadata;
use ::std::fs::Permissions;
#[cfg(unix)] use ::std::os::unix::fs::PermissionsExt;


pub mod arrays;
pub mod cstrings;
pub mod powersOfTwo;


include!("do_while_loop.rs");
include!("likely.rs");
include!("offsetOf.rs");
include!("true_immutable_thread_local.rs");
include!("unlikely.rs");
include!("u4.rs");
include!("u5.rs");
include!("u6.rs");
include!("u31.rs");
include!("UnixFileSystemPermissionMode.rs");
