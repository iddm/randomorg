# random.org
[![Build status](https://travis-ci.org/vityafx/randomorg.svg?branch=master)](https://travis-ci.org/vityafx/randomorg)
[![Crates](https://img.shields.io/crates/v/randomorg.svg)](https://crates.io/crates/randomorg)
[![Docs](https://docs.rs/randomorg/badge.svg)](https://docs.rs/randomorg)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


A https://random.org client library. The randomness comes from atmospheric noise, which
for many purposes is better than the pseudo-random number algorithms typically used in computer
programs.

## Status
Everything is implemented. Note, that the `random.org` service API is at beta stage of development,
however the library will try to be up-to-date.

## The documentation
The [documentation](https://docs.rs/toornament) which may help you using this library.

## Implementation
- Immutable interface, no need to synchronize, thread-safe (`Sync` and `Send`).
- No unsafe blocks
- `reqwest` crate is used for performing requests
- `chrono` for dates
- `serde` for serialization and deserialization

## Usage
Start by creating `Random` instance and perform needed operations after.

```rust,no_run
extern crate randomorg;

fn main() {
    use randomorg::Random;
    let r = Random::new("API KEY HERE").unwrap();
    println!("Result: {:?}", r.generate_integers(-100, 100, 15, true));
}
```

## License

This project is
[licensed under the MIT license](https://github.com/vityafx/randomorg/blob/master/LICENSE).
