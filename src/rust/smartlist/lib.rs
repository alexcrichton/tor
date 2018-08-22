// Copyright (c) 2016-2018, The Tor Project, Inc. */
// See LICENSE for licensing information */

extern crate libc;

mod smartlist;

pub use smartlist::*;

#[cfg(test)]
#[global_allocator]
static ALLOCATOR: std::alloc::System = std::alloc::System;
