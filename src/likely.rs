// This file is part of rust_extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust_extra/master/COPYRIGHT. No part of rust_extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust_extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust_extra/master/COPYRIGHT.


#[inline(always)]
pub fn likely(expression: bool) -> bool
{
	unsafe { ::core::intrinsics::likely(expression) }
}
