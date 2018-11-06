hex2bin
=======

Multithreaded hexadecimal dumps into binary files converter, inspired by
[pizip-hex2bin](https://github.com/mersinvald/pizip/tree/master/misc/hex2bin/)
by @mersinvald.

*Name of the tool is a subject of a possible change in the future.*

## Disclaimer

For now the converter is merely a research project, but who knows how the
things will turn out! Nevertheless, API of the tool is a subject to a constant
change. Okay, you have been warned, that's what disclaimers are for, I believe.

## Checking out the tool

Currently *hex2bin* is only capable of converting non-separated one-byte
unsigned integers in a hex format without a base. Error handling on the current
stage is rather poor, e.g. an attempt to process a hex dump with an uneven
number of bytes will raise a panic.

Here is an example which shows how to generate such a hex dump with a `xxd`
tool:

```
% xxd -p binary-input | tr -d '\n' > hex-output
```

In the future the `hex2bin` tool might provide a *reversed* functionality, i.e.
generating a hex dump from a binary fail.

When you have a continuous hex dump (without separators, that is) you should
finally check how the tool works and how blazingly fast it is:

```
% cargo run --release -- \
    -c $[200 * 1024 * 1024] \
    -t 4 \
    -i hex-output \
    -o binary-output \
    crossbeam
```

To understand the various options provided by the tool reffer to a built-in
help command: 

```
% cargo run --release -- help
```

### License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

License: MIT/Apache-2.0
