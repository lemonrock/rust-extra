// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


#[repr(C, packed)]
pub struct Array52<T>(pub [T; 52]);

impl<T> Array<T> for Array52<T>
{
	const Size: usize = 52;
	
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

impl<T: Copy> Copy for Array52<T>
{
}

impl<T: Copy> Clone for Array52<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Array52(self.0)
	}
}

impl<T: Debug> Debug for Array52<T>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		self.0[..].fmt(formatter)
	}
}

impl<T: PartialEq> PartialEq for Array52<T>
{
	#[inline(always)]
	fn eq(&self, other: &Array52<T>) -> bool
	{
		self.0[..].eq(&other.0[..])
	}
}

impl<T: Eq> Eq for Array52<T>
{
}

impl<T: PartialOrd> PartialOrd for Array52<T>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Array52<T>) -> Option<Ordering>
	{
		PartialOrd::partial_cmp(&&self.0[..], &&other.0[..])
	}

	#[inline(always)]
	fn lt(&self, other: &Array52<T>) -> bool
	{
		PartialOrd::lt(&&self.0[..], &&other.0[..])
	}

	#[inline(always)]
	fn le(&self, other: &Array52<T>) -> bool
	{
		PartialOrd::le(&&self.0[..], &&other.0[..])
	}

	#[inline(always)]
	fn ge(&self, other: &Array52<T>) -> bool
	{
		PartialOrd::ge(&&self.0[..], &&other.0[..])
	}

	#[inline(always)]
	fn gt(&self, other: &Array52<T>) -> bool
	{
		PartialOrd::gt(&&self.0[..], &&other.0[..])
	}
}

impl<T: Ord> Ord for Array52<T>
{
	#[inline(always)]
	fn cmp(&self, other: &Array52<T>) -> Ordering
	{
		Ord::cmp(&&self.0[..], &&other.0[..])
	}
}

impl<T: Hash> Hash for Array52<T>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		Hash::hash(&self.0[..], state)
	}
}

impl<'a, T> IntoIterator for &'a Array52<T>
{
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Iter<'a, T>
	{
		self.0.iter()
	}
}

impl<'a, T> IntoIterator for &'a mut Array52<T>
{
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> IterMut<'a, T>
	{
		self.0.iter_mut()
	}
}

impl<T> AsRef<[T]> for Array52<T>
{
	#[inline(always)]
	fn as_ref(&self) -> &[T]
	{
		&self.0[..]
	}
}

impl<T> AsMut<[T]> for Array52<T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T]
	{
		&mut self.0[..]
	}
}
