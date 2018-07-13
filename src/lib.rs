#![feature(use_extern_macros)]

extern crate tt_call;
use tt_call::tt_return;

#[macro_export]
macro_rules! tt_atoi {
    // done
    {
        $caller:tt
        input = [{ }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_return! {
            $caller
            input = [{ $($a)* }]
        }
    };

    // 0
    {
        $caller:tt
        input = [{ 0 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* }]
        }
    };

    // 1
    {
        $caller:tt
        input = [{ 1 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . }]
        }
    };

    // 2
    {
        $caller:tt
        input = [{ 2 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . . }]
        }
    };

    // 3
    {
        $caller:tt
        input = [{ 3 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . . . }]
        }
    };

    // 4
    {
        $caller:tt
        input = [{ 4 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . . . . }]
        }
    };

    // 5
    {
        $caller:tt
        input = [{ 5 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . . . . . }]
        }
    };

    // 6
    {
        $caller:tt
        input = [{ 6 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . . . . . . }]
        }
    };

    // 7
    {
        $caller:tt
        input = [{ 7 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . . . . . . . }]
        }
    };

    // 8
    {
        $caller:tt
        input = [{ 8 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . . . . . . . . }]
        }
    };

    // 9
    {
        $caller:tt
        input = [{ 9 $($n:tt)* }]
        acc = [{ $($a:tt)* }]
    } => {
        tt_atoi! {
            $caller
            input = [{ $($n)* }]
            acc = [{ $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* $($a)* . . . . . . . . . }]
        }
    };
}

