# Wheelhoss

A library and tool to help package applications for [Sandstorm](https://sandstorm.io)

Wheelhoss 0.1.2 has exactly one feature:

- update `sandstorm-files.list` to include Python source files

## Example

Wheelhoss can update `sandstorm-files.list` to add Python source files, which
correspond to Python bytecode files already listed in `sandstorm-files.list`.

```bash
$ grep __init__ .sandstorm/sandstorm-files.list
…
opt/app/env/lib/python3.10/site-packages/django/__pycache__/__init__.cpython-310.pyc
…
$ wheelhoss-fileslist-include-python-source-files
$ grep __init__ .sandstorm/sandstorm-files.list
…
opt/app/env/lib/python3.10/site-packages/django/__init__.py
…
opt/app/env/lib/python3.10/site-packages/django/__pycache__/__init__.cpython-310.pyc
…
```

## License

Wheelhoss is distributed under the terms of both the MIT license and the Apache
License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYING](COPYING) for details.

### Dependencies

#### fs3

https://crates.io/crates/fs3

`fs3` is a fork of `fs2` and is distributed under the terms of both the [MIT
license](https://github.com/oxidecomputer/fs3-rs/blob/0.5.0/LICENSE-MIT) and
the [Apache License (Version
2.0)](https://github.com/oxidecomputer/fs3-rs/blob/0.5.0/LICENSE-APACHE).

`fs3` depends on the `glibc` crate, which distributed under the terms of either
the [MIT license](https://github.com/rust-lang/libc/blob/0.2.117/LICENSE-MIT)
or the [Apache License (Version
2.0)](https://github.com/rust-lang/libc/blob/0.2.117/LICENSE-APACHE) at the
option of the recipient.
