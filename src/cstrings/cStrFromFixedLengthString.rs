// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


#[inline(always)]
fn cStrFromFixedLengthString<'a>(ptr: *const c_char, maximumLength: usize) -> &'a CStr
{
	let len = strnlen(ptr, maximumLength);
	if unlikely(len == maximumLength)
	{
		transmute(from_raw_parts(ptr, maximumLength))
	}
	else
	{
		transmute(from_raw_parts(ptr, len + 1))
	}
}
