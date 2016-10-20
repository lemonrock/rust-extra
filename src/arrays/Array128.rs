// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


#[repr(C, packed)]
pub struct Array128<T>
{
	pub array: [T; 128]
}

impl<T: Copy> Copy for Array128<T>
{
}

impl<T: Copy> Clone for Array128<T>
{
	fn clone(&self) -> Self
	{
		Array128
		{
			array: self.array
		}
	}
}

impl<T: Debug> Debug for Array128<T>
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		self.array[..].fmt(formatter)
	}
}

impl<T: PartialEq> PartialEq for Array128<T>
{
	fn eq(&self, other: &Array128<T>) -> bool
	{
		self.array[..].eq(&other.array[..])
	}
}

impl<T: Eq> Eq for Array128<T>
{
}

impl<T: PartialOrd> PartialOrd for Array128<T>
{
	#[inline]
	fn partial_cmp(&self, other: &Array128<T>) -> Option<Ordering>
	{
		PartialOrd::partial_cmp(&&self.array[..], &&other.array[..])
	}

	#[inline]
	fn lt(&self, other: &Array128<T>) -> bool
	{
		PartialOrd::lt(&&self.array[..], &&other.array[..])
	}

	#[inline]
	fn le(&self, other: &Array128<T>) -> bool
	{
		PartialOrd::le(&&self.array[..], &&other.array[..])
	}

	#[inline]
	fn ge(&self, other: &Array128<T>) -> bool
	{
		PartialOrd::ge(&&self.array[..], &&other.array[..])
	}

	#[inline]
	fn gt(&self, other: &Array128<T>) -> bool
	{
		PartialOrd::gt(&&self.array[..], &&other.array[..])
	}
}

impl<T: Ord> Ord for Array128<T>
{
	#[inline]
	fn cmp(&self, other: &Array128<T>) -> Ordering
	{
		Ord::cmp(&&self.array[..], &&other.array[..])
	}
}

impl<T: Hash> Hash for Array128<T>
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		Hash::hash(&self.array[..], state)
	}
}

impl<'a, T> IntoIterator for &'a Array128<T>
{
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Iter<'a, T>
	{
		self.array.iter()
	}
}

impl<'a, T> IntoIterator for &'a mut Array128<T>
{
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;

	fn into_iter(self) -> IterMut<'a, T>
	{
		self.array.iter_mut()
	}
}

impl<T> AsRef<[T]> for Array128<T>
{
	#[inline]
	fn as_ref(&self) -> &[T]
	{
		&self.array[..]
	}
}

impl<T> AsMut<[T]> for Array128<T>
{
	#[inline]
	fn as_mut(&mut self) -> &mut [T]
	{
		&mut self.array[..]
	}
}
