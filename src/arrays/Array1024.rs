// This file is part of rust-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT. No part of rust-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rust-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rust-extra/master/COPYRIGHT.


#[repr(C, packed)]
pub struct Array1024<T>(pub [T; 1024]);

impl<T: Copy> Copy for Array1024<T>
{
}

impl<T: Copy> Clone for Array1024<T>
{
	fn clone(&self) -> Self
	{
		Array1024(self.0)
	}
}

impl<T: Debug> Debug for Array1024<T>
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		self.0[..].fmt(formatter)
	}
}

impl<T: PartialEq> PartialEq for Array1024<T>
{
	fn eq(&self, other: &Array1024<T>) -> bool
	{
		self.0[..].eq(&other.0[..])
	}
}

impl<T: Eq> Eq for Array1024<T>
{
}

impl<T: PartialOrd> PartialOrd for Array1024<T>
{
	#[inline]
	fn partial_cmp(&self, other: &Array1024<T>) -> Option<Ordering>
	{
		PartialOrd::partial_cmp(&&self.0[..], &&other.0[..])
	}

	#[inline]
	fn lt(&self, other: &Array1024<T>) -> bool
	{
		PartialOrd::lt(&&self.0[..], &&other.0[..])
	}

	#[inline]
	fn le(&self, other: &Array1024<T>) -> bool
	{
		PartialOrd::le(&&self.0[..], &&other.0[..])
	}

	#[inline]
	fn ge(&self, other: &Array1024<T>) -> bool
	{
		PartialOrd::ge(&&self.0[..], &&other.0[..])
	}

	#[inline]
	fn gt(&self, other: &Array1024<T>) -> bool
	{
		PartialOrd::gt(&&self.0[..], &&other.0[..])
	}
}

impl<T: Ord> Ord for Array1024<T>
{
	#[inline]
	fn cmp(&self, other: &Array1024<T>) -> Ordering
	{
		Ord::cmp(&&self.0[..], &&other.0[..])
	}
}

impl<T: Hash> Hash for Array1024<T>
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		Hash::hash(&self.0[..], state)
	}
}

impl<'a, T> IntoIterator for &'a Array1024<T>
{
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Iter<'a, T>
	{
		self.0.iter()
	}
}

impl<'a, T> IntoIterator for &'a mut Array1024<T>
{
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;

	fn into_iter(self) -> IterMut<'a, T>
	{
		self.0.iter_mut()
	}
}

impl<T> AsRef<[T]> for Array1024<T>
{
	#[inline]
	fn as_ref(&self) -> &[T]
	{
		&self.0[..]
	}
}

impl<T> AsMut<[T]> for Array1024<T>
{
	#[inline]
	fn as_mut(&mut self) -> &mut [T]
	{
		&mut self.0[..]
	}
}
