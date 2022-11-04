// SPDX-FileCopyrightText: The unnest authors
// SPDX-License-Identifier: MPL-2.0

#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(test), deny(clippy::panic_in_result_fn))]
#![cfg_attr(not(debug_assertions), deny(clippy::used_underscore_binding))]

//! Early break/continue/return if an optional expression evaluates to `None`.

pub mod docs {
    //! Documentation and specification

    // TODO: README.md does not contain any Rust code blocks!?
    #![allow(rustdoc::invalid_rust_codeblocks)]
    #![doc = include_str!("../README.md")]
}

/// Break if an optional expression evaluates to `None`.
#[macro_export]
macro_rules! some_or_break {
    ($expr:expr) => {
        if let ::core::option::Option::Some(some) = $expr {
            some
        } else {
            break;
        }
    };
}

/// Continue if an optional expression evaluates to `None`.
#[macro_export]
macro_rules! some_or_continue {
    ($expr:expr) => {
        if let ::core::option::Option::Some(some) = $expr {
            some
        } else {
            continue;
        }
    };
}

/// Return if an optional expression evaluates to `None`.
#[macro_export]
macro_rules! some_or_return {
    ($expr:expr) => {
        if let ::core::option::Option::Some(some) = $expr {
            some
        } else {
            return;
        }
    };
}

/// Return a residual value if an optional expression evaluates to `None`.
#[macro_export]
macro_rules! some_or_return_with {
    ($expr:expr, $with:expr) => {
        if let ::core::option::Option::Some(some) = $expr {
            some
        } else {
            return $with;
        }
    };
}

/// Break if a `Result` expression evaluates to `Err`.
#[macro_export]
macro_rules! ok_or_break {
    ($expr:expr) => {
        if let ::core::result::Result::Ok(ok) = $expr {
            ok
        } else {
            break;
        }
    };
}

/// Continue if a `Result` expression evaluates to `Err`.
#[macro_export]
macro_rules! ok_or_continue {
    ($expr:expr) => {
        if let ::core::result::Result::Ok(ok) = $expr {
            ok
        } else {
            continue;
        }
    };
}

/// Return if a `Result` expression evaluates to `Err`.
#[macro_export]
macro_rules! ok_or_return {
    ($expr:expr) => {
        if let ::core::result::Result::Ok(ok) = $expr {
            ok
        } else {
            return;
        }
    };
}

/// Return the error if a `Result` expression evaluates to `Err`.
///
/// A light-weight version of the question mark operator `?`.
///
/// See also: <https://twitter.com/mitsuhiko/status/1574111445473579008>
#[macro_export]
macro_rules! ok_or_return_err {
    ($expr:expr) => {
        match $expr {
            ::core::result::Result::Ok(ok) => ok,
            ::core::result::Result::Err(err) => return err,
        }
    };
}

/// Return a residual value if a `Result` expression evaluates to `Err`.
#[macro_export]
macro_rules! ok_or_return_with {
    ($expr:expr, $with:expr) => {
        if let ::core::result::Result::Ok(ok) = $expr {
            ok
        } else {
            return $with;
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::while_let_loop)]
    fn some_or_break() {
        loop {
            some_or_break!(None);
            unreachable!();
        }
    }

    #[test]
    #[allow(clippy::needless_continue)]
    fn some_or_continue() {
        let mut loop_count = 0;
        loop {
            loop_count += 1;
            assert_eq!(1, loop_count);
            some_or_continue!(Some(()));
            break;
        }
    }

    #[test]
    fn some_or_return() {
        some_or_return!(None);
        unreachable!();
    }

    #[test]
    #[allow(clippy::unused_unit)]
    fn some_or_return_with() {
        some_or_return_with!(None, ());
        unreachable!();
    }

    #[test]
    #[allow(clippy::while_let_loop)]
    fn ok_or_break() {
        loop {
            ok_or_break!(Err(()));
            unreachable!();
        }
    }

    #[test]
    #[allow(clippy::needless_continue)]
    fn ok_or_continue() {
        let mut loop_count = 0;
        let ok: Result<_, ()> = Ok(());
        loop {
            loop_count += 1;
            assert_eq!(1, loop_count);
            ok_or_continue!(ok);
            break;
        }
    }

    #[test]
    fn ok_or_return() {
        ok_or_return!(Err(()));
        unreachable!();
    }

    #[test]
    #[allow(clippy::unused_unit)]
    fn ok_or_return_with() {
        ok_or_return_with!(Err(()), ());
        unreachable!();
    }
}
