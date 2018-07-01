#![feature(async_await, await_macro, futures_api)]

extern crate futures;

use futures::executor::block_on;

async fn foo() -> i32 {
    return 42;
}

async fn bar() -> i32 {
    return await!(foo());
}

fn main() {
    let i = block_on(bar());
    println!("{}", i);
}
