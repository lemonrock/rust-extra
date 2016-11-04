// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


#[repr(C, packed)]
pub struct Array128<T>(pub [T; 128]);

impl<T> Array<T> for Array128<T>
{
	const Size: usize = 128;
	const Mask: usize = 128 - 1;
	
	#[inline(always)]
	unsafe fn get_unchecked(&self, index: usize) -> &T
	{
		self.0.get_unchecked(index)
	}
	
	#[inline(always)]
	unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T
	{
		self.0.get_unchecked_mut(index)
	}
}

impl<T: Copy> Copy for Array128<T>
{
}

impl<T: Copy> Clone for Array128<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Array128(self.0)
	}
}

impl<T: Debug> Debug for Array128<T>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		self.0[..].fmt(formatter)
	}
}

impl<T: PartialEq> PartialEq for Array128<T>
{
	#[inline(always)]
	fn eq(&self, other: &Array128<T>) -> bool
	{
		self.0[..].eq(&other.0[..])
	}
}

impl<T: Eq> Eq for Array128<T>
{
}

impl<T: PartialOrd> PartialOrd for Array128<T>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Array128<T>) -> Option<Ordering>
	{
		PartialOrd::partial_cmp(&&self.0[..], &&other.0[..])
	}

	#[inline(always)]
	fn lt(&self, other: &Array128<T>) -> bool
	{
		PartialOrd::lt(&&self.0[..], &&other.0[..])
	}

	#[inline(always)]
	fn le(&self, other: &Array128<T>) -> bool
	{
		PartialOrd::le(&&self.0[..], &&other.0[..])
	}

	#[inline(always)]
	fn ge(&self, other: &Array128<T>) -> bool
	{
		PartialOrd::ge(&&self.0[..], &&other.0[..])
	}

	#[inline(always)]
	fn gt(&self, other: &Array128<T>) -> bool
	{
		PartialOrd::gt(&&self.0[..], &&other.0[..])
	}
}

impl<T: Ord> Ord for Array128<T>
{
	#[inline(always)]
	fn cmp(&self, other: &Array128<T>) -> Ordering
	{
		Ord::cmp(&&self.0[..], &&other.0[..])
	}
}

impl<T: Hash> Hash for Array128<T>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		Hash::hash(&self.0[..], state)
	}
}

impl<'a, T> IntoIterator for &'a Array128<T>
{
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Iter<'a, T>
	{
		self.0.iter()
	}
}

impl<'a, T> IntoIterator for &'a mut Array128<T>
{
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> IterMut<'a, T>
	{
		self.0.iter_mut()
	}
}

impl<T> AsRef<[T]> for Array128<T>
{
	#[inline(always)]
	fn as_ref(&self) -> &[T]
	{
		&self.0[..]
	}
}

impl<T> AsMut<[T]> for Array128<T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T]
	{
		&mut self.0[..]
	}
}
