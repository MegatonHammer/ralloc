//! Symbols and externs that `ralloc` depends on.
//!
//! This crate provides implementation/import of these in Linux, BSD, and Mac OS.
//!
//! # Important
//!
//! You CANNOT use libc library calls, due to no guarantees being made about allocations of the
//! functions in the POSIX specification. Therefore, we use the system calls directly.

#![feature(linkage, core_intrinsics)]
#![cfg_attr(target_os = "switch", feature(ptr_internals))]
#![no_std]
#![warn(missing_docs)]

#[cfg(not(any(target_os = "redox", target_os = "switch")))]
#[macro_use]
extern crate sc;

#[cfg(target_os = "redox")]
extern crate syscall;

#[cfg(target_os = "switch")]
extern crate megaton_hammer;

pub mod config;
pub mod thread_destructor;
pub mod debug;
pub mod syscalls;
