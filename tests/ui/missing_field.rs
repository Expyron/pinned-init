#![feature(allocator_api)]
extern crate pinned_init;
use pinned_init::*;

#[pin_data]
struct Foo {
    a: usize,
    b: usize,
}

fn main() {
    let _foo = pin_init!(Foo { a: 0 });
    let _foo = try_pin_init!(Foo { a: 0 });
    let _foo = init!(Foo { a: 0 });
    let _foo = try_init!(Foo { a: 0 });
}
