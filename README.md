tt-num
======

[![Build Status](https://api.travis-ci.org/durka/tt-num.svg?branch=master)](https://travis-ci.org/durka/tt-num)
[![Latest Version](https://img.shields.io/crates/v/tt-num.svg)](https://crates.io/crates/tt-num)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/tt-num)

**This library is an entry in the [`tt_call`](https://crates.io/crates/tt-call) series of modular interoperable
tt-muncher building blocks.**

Macros are contained here that perform a variety of operations on "numbers".
These may be semi-human-readable literals, or encodings for macros to use
(like unary and nested-parens). Operations will include conversions and math.

Included macros:

- `tt_atoi`: convert decimal-encoded numbers (e.g. `[4 2]`) to a unary
  encoding that macros can use

