#![feature(use_extern_macros)]
#![recursion_limit="512"]

extern crate tt_call;
extern crate tt_num;
use tt_call::{tt_call, tt_replace, tt_true};
use tt_num::tt_atoi;

macro_rules! times {
    ([$($n:tt)*] $code:expr) => {
        tt_call! {
            macro = [{ private_times }]
            num = [{ $($n)* }]
            code = [{ $code }]
        }
    }
}

macro_rules! private_times {
    {
        $caller:tt
        num = [{ $($n:tt)* }]
        code = [{ $code:expr }]
    } => {
        tt_call! {
            macro = [{ tt_atoi }]
            input = [{ $($n)* }]
            acc = [{ }]
            ~~> tt_replace! {
                $caller
                condition = [{ tt_true }]
                replace_with = [{ $code }]
            }
        }
    }
}

fn main() {
    let mut i = 0;
    times!([4 2] {
        i += 1;
    });
    println!("i = {}", i);
    assert_eq!(i, 42);
}

