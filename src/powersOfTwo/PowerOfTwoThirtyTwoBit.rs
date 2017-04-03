// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PowerOfTwoThirtyTwoBit
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
	_65536 = 65536,
	_131072 = 131072,
	_262144 = 262144,
	_524288 = 524288,
	_1048576 = 1048576,
	_2097152 = 2097152,
	_4194304 = 4194304,
	_8388608 = 8388608,
	_16777216 = 16777216,
	_33554432 = 33554432,
	_67108864 = 67108864,
	_134217728 = 134217728,
	_268435456 = 268435456,
	_536870912 = 536870912,
	_1073741824 = 1073741824,
	_2147483648 = 2147483648,
}

impl AsU32 for Option<PowerOfTwoThirtyTwoBit>
{
	#[inline(always)]
	fn as_u32(&self) -> u32
	{
		if self.is_none()
		{
			0
		}
		else
		{
			self.unwrap().as_u32()
		}
	}
}

impl AsU32 for PowerOfTwoThirtyTwoBit
{
	#[inline(always)]
	fn as_u32(&self) -> u32
	{
		*self as u32
	}
}

impl PowerOfTwoThirtyTwoBit
{
	// Does not check for value == 0 (that is not a power of two)
	#[inline(always)]
	pub fn isPowerOfTwo(value: u32) -> bool
	{
		debug_assert!(value != 0, "zero is not a valid power of two");
		
	    (value & (value - 1)) == 0
	}
	
	#[inline(always)]
	pub unsafe fn from_u32_unchecked(value: u32) -> PowerOfTwoThirtyTwoBit
	{
		debug_assert!(value != 0, "zero is not a valid power of two");
		
		transmute(value)
	}
	
	#[inline(always)]
	pub unsafe fn from_u32_panic(value: u32) -> PowerOfTwoThirtyTwoBit
	{
		match value
		{
			0 => panic!("Zero is not converted"),
			
			1 => PowerOfTwoThirtyTwoBit::_1,
			2 => PowerOfTwoThirtyTwoBit::_2,
			4 => PowerOfTwoThirtyTwoBit::_4,
			8 => PowerOfTwoThirtyTwoBit::_8,
			16 => PowerOfTwoThirtyTwoBit::_16,
			32 => PowerOfTwoThirtyTwoBit::_32,
			64 => PowerOfTwoThirtyTwoBit::_64,
			128 => PowerOfTwoThirtyTwoBit::_128,
			256 => PowerOfTwoThirtyTwoBit::_256,
			512 => PowerOfTwoThirtyTwoBit::_512,
			1024 => PowerOfTwoThirtyTwoBit::_1024,
			2048 => PowerOfTwoThirtyTwoBit::_2048,
			4096 => PowerOfTwoThirtyTwoBit::_4096,
			8192 => PowerOfTwoThirtyTwoBit::_8192,
			16384 => PowerOfTwoThirtyTwoBit::_16384,
			32768 => PowerOfTwoThirtyTwoBit::_32768,
			65536 => PowerOfTwoThirtyTwoBit::_65536,
			131072 => PowerOfTwoThirtyTwoBit::_131072,
			262144 => PowerOfTwoThirtyTwoBit::_262144,
			524288 => PowerOfTwoThirtyTwoBit::_524288,
			1048576 => PowerOfTwoThirtyTwoBit::_1048576,
			2097152 => PowerOfTwoThirtyTwoBit::_2097152,
			4194304 => PowerOfTwoThirtyTwoBit::_4194304,
			8388608 => PowerOfTwoThirtyTwoBit::_8388608,
			16777216 => PowerOfTwoThirtyTwoBit::_16777216,
			33554432 => PowerOfTwoThirtyTwoBit::_33554432,
			67108864 => PowerOfTwoThirtyTwoBit::_67108864,
			134217728 => PowerOfTwoThirtyTwoBit::_134217728,
			268435456 => PowerOfTwoThirtyTwoBit::_268435456,
			536870912 => PowerOfTwoThirtyTwoBit::_536870912,
			1073741824 => PowerOfTwoThirtyTwoBit::_1073741824,
			2147483648 => PowerOfTwoThirtyTwoBit::_2147483648,
			
			_ => panic!("The value '{}' is not a power of two", value),
		}
	}
	
	#[inline(always)]
	pub unsafe fn from_u32(value: u32) -> Option<PowerOfTwoThirtyTwoBit>
	{
		match value
		{
			0 => None,
			
			1 => Some(PowerOfTwoThirtyTwoBit::_1),
			2 => Some(PowerOfTwoThirtyTwoBit::_2),
			4 => Some(PowerOfTwoThirtyTwoBit::_4),
			8 => Some(PowerOfTwoThirtyTwoBit::_8),
			16 => Some(PowerOfTwoThirtyTwoBit::_16),
			32 => Some(PowerOfTwoThirtyTwoBit::_32),
			64 => Some(PowerOfTwoThirtyTwoBit::_64),
			128 => Some(PowerOfTwoThirtyTwoBit::_128),
			256 => Some(PowerOfTwoThirtyTwoBit::_256),
			512 => Some(PowerOfTwoThirtyTwoBit::_512),
			1024 => Some(PowerOfTwoThirtyTwoBit::_1024),
			2048 => Some(PowerOfTwoThirtyTwoBit::_2048),
			4096 => Some(PowerOfTwoThirtyTwoBit::_4096),
			8192 => Some(PowerOfTwoThirtyTwoBit::_8192),
			16384 => Some(PowerOfTwoThirtyTwoBit::_16384),
			32768 => Some(PowerOfTwoThirtyTwoBit::_32768),
			65536 => Some(PowerOfTwoThirtyTwoBit::_65536),
			131072 => Some(PowerOfTwoThirtyTwoBit::_131072),
			262144 => Some(PowerOfTwoThirtyTwoBit::_262144),
			524288 => Some(PowerOfTwoThirtyTwoBit::_524288),
			1048576 => Some(PowerOfTwoThirtyTwoBit::_1048576),
			2097152 => Some(PowerOfTwoThirtyTwoBit::_2097152),
			4194304 => Some(PowerOfTwoThirtyTwoBit::_4194304),
			8388608 => Some(PowerOfTwoThirtyTwoBit::_8388608),
			16777216 => Some(PowerOfTwoThirtyTwoBit::_16777216),
			33554432 => Some(PowerOfTwoThirtyTwoBit::_33554432),
			67108864 => Some(PowerOfTwoThirtyTwoBit::_67108864),
			134217728 => Some(PowerOfTwoThirtyTwoBit::_134217728),
			268435456 => Some(PowerOfTwoThirtyTwoBit::_268435456),
			536870912 => Some(PowerOfTwoThirtyTwoBit::_536870912),
			1073741824 => Some(PowerOfTwoThirtyTwoBit::_1073741824),
			2147483648 => Some(PowerOfTwoThirtyTwoBit::_2147483648),
			
			_ => None,
		}
	}
}
