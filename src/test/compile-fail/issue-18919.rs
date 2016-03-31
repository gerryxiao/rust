// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

type FuncType<'f> = Fn(&isize) -> isize + 'f;

fn ho_func(f: Option<FuncType>) {
    //~^ ERROR: the trait `std::marker::Sized` is not implemented for the type
}

fn main() {}
