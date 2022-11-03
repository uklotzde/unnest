<!-- SPDX-FileCopyrightText: The unnest authors -->
<!-- SPDX-License-Identifier: MPL-2.0 -->

# unnest

[![Crates.io](https://img.shields.io/crates/v/unnest.svg)](https://crates.io/crates/unnest)
[![Docs.rs](https://docs.rs/unnest/badge.svg)](https://docs.rs/unnest)
[![Deps.rs](https://deps.rs/repo/github/uklotzde/unnest/status.svg)](https://deps.rs/repo/github/uklotzde/unnest)
[![Security audit](https://github.com/uklotzde/unnest/actions/workflows/security-audit.yaml/badge.svg)](https://github.com/uklotzde/unnest/actions/workflows/security-audit.yaml)
[![Continuous integration](https://github.com/uklotzde/unnest/actions/workflows/continuous-integration.yaml/badge.svg)](https://github.com/uklotzde/unnest/actions/workflows/continuous-integration.yaml)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

Macros for an _unnested_ control flow. In contrast to
[`let-else` statements](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#let-else-statements)
these macros evaluate to expressions.

## License

Licensed under the Mozilla Public License 2.0 (MPL-2.0) (see [MPL-2.0.txt](LICENSES/MPL-2.0.txt) or <https://www.mozilla.org/MPL/2.0/>).

Permissions of this copyleft license are conditioned on making available source code of licensed files and modifications of those files under the same license (or in certain cases, one of the GNU licenses). Copyright and license notices must be preserved. Contributors provide an express grant of patent rights. However, a larger work using the licensed work may be distributed under different terms and without source code for files added in the larger work.

### Contribution

Any contribution intentionally submitted for inclusion in the work by you shall be licensed under the Mozilla Public License 2.0 (MPL-2.0).

It is required to add the following header with the corresponding [SPDX short identifier](https://spdx.dev/ids/) to the top of each file:

```rust
// SPDX-License-Identifier: MPL-2.0
```
