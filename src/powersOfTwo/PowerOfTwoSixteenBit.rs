// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum PowerOfTwoSixteenBit
{
	_1 = 1,
	_2 = 2,
	_4 = 4,
	_8 = 8,
	_16 = 16,
	_32 = 32,
	_64 = 64,
	_128 = 128,
	_256 = 256,
	_512 = 512,
	_1024 = 1024,
	_2048 = 2048,
	_4096 = 4096,
	_8192 = 8192,
	_16384 = 16384,
	_32768 = 32768,
}

impl PowerOfTwoSixteenBit
{
	// Does not check for value == 0 (that is not a power of two)
	#[inline(always)]
	pub fn isPowerOfTwo(value: u16) -> bool
	{
		debug_assert!(value != 0, "zero is not a valid power of two");
		
	    (value & (value - 1)) == 0
	}
	
	#[inline(always)]
	pub unsafe fn from_u16_unchecked(value: u16) -> PowerOfTwoSixteenBit
	{
		debug_assert!(value != 0, "zero is not a valid power of two");
		
		transmute(value)
	}
	
	#[inline(always)]
	pub unsafe fn from_u16_panic(value: u16) -> PowerOfTwoSixteenBit
	{
		match value
		{
			0 => panic!("Zero is not converted"),
			
			1 => PowerOfTwoSixteenBit::_1,
			2 => PowerOfTwoSixteenBit::_2,
			4 => PowerOfTwoSixteenBit::_4,
			8 => PowerOfTwoSixteenBit::_8,
			16 => PowerOfTwoSixteenBit::_16,
			32 => PowerOfTwoSixteenBit::_32,
			64 => PowerOfTwoSixteenBit::_64,
			128 => PowerOfTwoSixteenBit::_128,
			256 => PowerOfTwoSixteenBit::_256,
			512 => PowerOfTwoSixteenBit::_512,
			1024 => PowerOfTwoSixteenBit::_1024,
			2048 => PowerOfTwoSixteenBit::_2048,
			4096 => PowerOfTwoSixteenBit::_4096,
			8192 => PowerOfTwoSixteenBit::_8192,
			16384 => PowerOfTwoSixteenBit::_16384,
			32768 => PowerOfTwoSixteenBit::_32768,
			
			_ => panic!("The value '{}' is not a power of two", value),
		}
	}
	
	#[inline(always)]
	pub unsafe fn from_u16_panic_withZeroAs<F>(value: u16, zeroIs: F) -> PowerOfTwoSixteenBit
	where F: Fn() -> PowerOfTwoSixteenBit
	{
		match value
		{
			0 => zeroIs(),
			
			1 => PowerOfTwoSixteenBit::_1,
			2 => PowerOfTwoSixteenBit::_2,
			4 => PowerOfTwoSixteenBit::_4,
			8 => PowerOfTwoSixteenBit::_8,
			16 => PowerOfTwoSixteenBit::_16,
			32 => PowerOfTwoSixteenBit::_32,
			64 => PowerOfTwoSixteenBit::_64,
			128 => PowerOfTwoSixteenBit::_128,
			256 => PowerOfTwoSixteenBit::_256,
			512 => PowerOfTwoSixteenBit::_512,
			1024 => PowerOfTwoSixteenBit::_1024,
			2048 => PowerOfTwoSixteenBit::_2048,
			4096 => PowerOfTwoSixteenBit::_4096,
			8192 => PowerOfTwoSixteenBit::_8192,
			16384 => PowerOfTwoSixteenBit::_16384,
			32768 => PowerOfTwoSixteenBit::_32768,
			
			_ => panic!("The value '{}' is not a power of two", value),
		}
	}
	
	#[inline(always)]
	pub unsafe fn from_u16(value: u16) -> Option<PowerOfTwoSixteenBit>
	{
		match value
		{
			0 => None,
			
			1 => Some(PowerOfTwoSixteenBit::_1),
			2 => Some(PowerOfTwoSixteenBit::_2),
			4 => Some(PowerOfTwoSixteenBit::_4),
			8 => Some(PowerOfTwoSixteenBit::_8),
			16 => Some(PowerOfTwoSixteenBit::_16),
			32 => Some(PowerOfTwoSixteenBit::_32),
			64 => Some(PowerOfTwoSixteenBit::_64),
			128 => Some(PowerOfTwoSixteenBit::_128),
			256 => Some(PowerOfTwoSixteenBit::_256),
			512 => Some(PowerOfTwoSixteenBit::_512),
			1024 => Some(PowerOfTwoSixteenBit::_1024),
			2048 => Some(PowerOfTwoSixteenBit::_2048),
			4096 => Some(PowerOfTwoSixteenBit::_4096),
			8192 => Some(PowerOfTwoSixteenBit::_8192),
			16384 => Some(PowerOfTwoSixteenBit::_16384),
			32768 => Some(PowerOfTwoSixteenBit::_32768),
			
			_ => None,
		}
	}
}
