// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


pub trait Array<T>
{
	const Size: usize;
	const Mask: usize;
	
	#[inline(always)]
	fn mask() -> usize
	{
		Self::Mask
	}
	
	#[inline(always)]
	unsafe fn get_unchecked(&self, index: usize) -> &T;
	
	#[inline(always)]
	unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T;
	
	#[inline(always)]
	fn as_ptr(&self) -> *const T;
	
	#[inline(always)]
	fn as_mut_ptr(&mut self) -> *mut T;
	
	#[inline(always)]
	fn as_ptr_at(&self, index: usize) -> *const T
	{
		unsafe { self.as_ptr().offset(index as isize) }
	}
	
	#[inline(always)]
	fn as_mut_ptr_at(&mut self, index: usize) -> *mut T
	{
		unsafe { self.as_mut_ptr().offset(index as isize) }
	}
}
