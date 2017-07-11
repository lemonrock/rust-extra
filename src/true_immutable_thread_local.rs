// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


#[macro_export]
macro_rules! true_immutable_thread_local
{
	($type: ty, $initializer: block) =>
	{
		{
			// Done this way so that values do not need to implement 'Sync', which is pointless for a thread local static...
			#[thread_local] static mut THREAD_LOCAL: *const () = ::std::ptr::null_mut();
			unsafe
			{
				if THREAD_LOCAL.is_null()
				{
					let value = Box::new($initializer);
					THREAD_LOCAL = Box::into_raw(value) as *const _;
				}
				&*(THREAD_LOCAL as *const $type)
			}
		}
	}
}
