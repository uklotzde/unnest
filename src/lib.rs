// SPDX-FileCopyrightText: The if-none authors
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
macro_rules! if_none_break {
    ($var:expr) => {
        // Suppress clippy warnings when using the macros on std::result:.Result::ok()
        #[allow(clippy::match_result_ok)]
        if let Some(some) = $var {
            some
        } else {
            break;
        }
    };
}

/// Continue if an optional expression evaluates to `None`.
#[macro_export]
macro_rules! if_none_continue {
    ($var:expr) => {
        // Suppress clippy warnings when using the macros on std::result:.Result::ok()
        #[allow(clippy::match_result_ok)]
        if let Some(some) = $var {
            some
        } else {
            continue;
        }
    };
}

/// Return if an optional expression evaluates to `None`.
#[macro_export]
macro_rules! if_none_return {
    ($var:expr) => {
        // Suppress clippy warnings when using the macros on std::result:.Result::ok()
        #[allow(clippy::match_result_ok)]
        if let Some(some) = $var {
            some
        } else {
            return;
        }
    };
}

/// Return a residual value if an optional expression evaluates to `None`.
#[macro_export]
macro_rules! if_none_return_with {
    ($var:expr, $ret:expr) => {
        // Suppress clippy warnings when using the macros on std::result:.Result::ok()
        #[allow(clippy::match_result_ok)]
        if let Some(some) = $var {
            some
        } else {
            return $ret;
        }
    };
}
