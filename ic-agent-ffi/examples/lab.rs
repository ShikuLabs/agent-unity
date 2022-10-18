#![feature(unsize)]

use ic_agent::identity::AnonymousIdentity;
use ic_agent::Identity;
use std::alloc::{alloc, Layout};
use std::borrow::{Borrow, BorrowMut};
use std::convert::Infallible;
use std::marker::Unsize;
use std::ptr;

fn main() {
    unsafe {
        let layout = Layout::new::<*mut dyn Identity>();
        let mut ptr = alloc(layout) as *mut AnonymousIdentity as *mut dyn Identity;
        ret_fat_ptr_02(&mut ptr, AnonymousIdentity {});

        let boxed = Box::from_raw(ptr);
        println!("{}", boxed.sender().unwrap());
    }
}

fn ret_fat_ptr<T, U>(t: T) -> *mut U
where
    T: Unsize<U>,
    U: ?Sized,
{
    let boxed = Box::new(t);
    let raw = Box::into_raw(boxed);

    raw
}

fn ret_fat_ptr_02<T, U>(p2p: *mut *mut U, t: T)
where
    T: Unsize<U>,
    U: ?Sized,
{
    let boxed = Box::new(t);
    let raw = Box::into_raw(boxed);

    unsafe {
        *p2p = raw;
    }
}
