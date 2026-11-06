#![feature(dropck_eyepatch)]
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

// if a struct with generic type T implements Drop,
// the compiler's drop check assumes that the struct accesses T (mutably?)
// the dropcheck eyepatch disables that.
pub struct Boks<T> {
    p: NonNull<T>,
    _t: PhantomData<T>,
}

impl<T> Boks<T> {
    pub fn new(t: T) -> Self {
        Boks {
            // Safety: Box never creates a null pointer
            p: Box::into_non_null(Box::new(t)),
            _t: PhantomData,
        }
    }
}

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety: p was constructed from Box in the first place,
        //          and hasnt been freed as self is still alive
        drop(unsafe { Box::from_raw(self.p.as_mut()) });
        // Reader's note: the safety arguments in this module depend on p never being reassigned.
        // If a separate function did self.p = NullPtr then the argument doesnt hold.
        // Because main is in this same module, main can just change p which isn't great.
        // So the safety depends on:
        //  - there are no modifications to p within the module
        //  - there are no modifications to p outside the module because p is not visible outside
        //  the module
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: constructed from valid T,
        //          turned into a pointer via Box
        //          not freed because self is alive
        // Hence, valid
        unsafe { self.p.as_ref() }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: check Deref impl
        unsafe { &mut *self.p.as_mut() }
    }
}

struct Oisann<T: Debug>(T);
impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = 42;
    let b = Boks::new(x);
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boks::new(&mut y);
    // println!("{:?}", y);
    // there is an implicit drop(b) at the end that takes &mut b.
    // so using y again is having two mutable references active at the same time
    // However, if we remove the Drop implementation, this assumption doesnt hold.

    let mut z = 42;
    let b = Boks::new(Oisann(&mut z));
    // println!("{:?}", z);
    // there is a drop(b) below here which will use &mut z
    // and the above shouldn't compile without PhantomData but it will.
    // at this point, Boks is not covariant (in fact Boks is invariant because of mut)

    // here alice gives an example in the chat to show Boks is invariant
    let s = String::from("hei");
    let mut boks1 = Boks::new(&*s);
    let boks2: Boks<&'static str> = Boks::new("heisann");
    boks1 = boks2;

    // there is extended discussion here on empty on the video that I didnt quite catch
}
