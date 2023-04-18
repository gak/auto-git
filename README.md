# Auto git

Automatically commit, pull and push changes to a git repository.

This is a quick and dirty cli tool to automate the process of committing, pulling and pushing changes to a git repository.

## Warning

Not tested much. Use at your own risk.

## Installation from source

No binaries yet. You need to build from source using Rust.

```bash
cargo install auto-git
```

## Usage

```bash

# Automatically commit, pull and push to the "wip" branch.
# You will need to set up remote tracking for the branch.

> auto-git wip

```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
