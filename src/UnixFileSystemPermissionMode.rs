// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnixFileSystemPermissionMode
{
	pub mode: u32
}

impl UnixFileSystemPermissionMode
{
	#[inline(always)]
	pub fn fromMetadata(metadata: &Metadata) -> Self
	{
		Self::fromPermissions(&metadata.permissions())
	}
	
	#[cfg(unix)]
	#[inline(always)]
	pub fn fromPermissions(permissions: &Permissions) -> Self
	{
		UnixFileSystemPermissionMode
		{
			mode: permissions.mode()
		}
	}
	
	#[inline(always)]
	pub fn isOwnerExecutable(&self) -> bool
	{
		self.mode & 0o0100 == 0o0100
	}
	
	#[inline(always)]
	pub fn isGroupExecutable(&self) -> bool
	{
		self.mode & 0o0010 == 0o0010
	}
	
	#[inline(always)]
	pub fn isOtherExecutable(&self) -> bool
	{
		self.mode & 0o0001 == 0o0001
	}
	
	#[inline(always)]
	pub fn isOwnerReadableAndExecutable(&self) -> bool
	{
		self.mode & 0o0500 == 0o0500
	}
	
	#[inline(always)]
	pub fn isAllExecutable(&self) -> bool
	{
		self.mode & 0o0111 == 0o0111
	}
	
	#[inline(always)]
	pub fn isAllWritable(&self) -> bool
	{
		self.mode & 0o0222 == 0o0222
	}
	
	#[inline(always)]
	pub fn isAllReadable(&self) -> bool
	{
		self.mode & 0o0444 == 0o0444
	}
}
