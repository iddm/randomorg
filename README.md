# random.org
[![Build status](https://travis-ci.org/iddm/randomorg.svg?branch=master)](https://travis-ci.org/iddm/randomorg)
[![Crates](https://img.shields.io/crates/v/randomorg.svg)](https://crates.io/crates/randomorg)
[![Docs](https://docs.rs/randomorg/badge.svg)](https://docs.rs/randomorg)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


A https://random.org client library. The randomness comes from atmospheric noise, which
for many purposes is better than the pseudo-random number algorithms typically used in computer
programs.

## Status
Everything is implemented. Note, that the [`random.org`](https://random.org) service
API is at beta stage of development, however the library will try to be up-to-date.

## The documentation
The [documentation](https://docs.rs/randomorg) which may help you using this library.

## Implementation
- Immutable interface, no need to synchronize,
thread-safe ([`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) and
[`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html)).
- No unsafe blocks.
- [`reqwest`](https://crates.io/crates/reqwest) crate is used for performing requests.
- [`chrono`](https://crates.io/crates/chrono) for dates.
- [`serde`](https://crates.io/crates/serde) for serialization and deserialization.

## Features
- `rand` feature which provides the
[`rand_core::RngCore`](https://rust-random.github.io/rand/rand_core/trait.RngCore.html)
trait implementation for the [`Random`](https://docs.rs/randomorg/0.5.0/randomorg/struct.Random.html)
struct and adds new `FallibleRandom<T: rand_core::RngCore>` structure
for better random generation UX.

## Usage
Start by creating `Random` instance and perform needed operations after.

```rust
extern crate randomorg;

fn main() {
    use randomorg::Random;
    let r = Random::new("API KEY HERE");
    // A method-call way:
    println!("Result: {:?}", r.generate_integers(-100, 100, 15, true));
    // A lazy request builder way:
    let random_data = r.request_integers().min(0).max(100).limit(5).collect::<Vec<i32>>();
    println!("Random integers: {:?}", random_data);
}
```

With the `rand` feature you can also use it that way:

```rust
extern crate randomorg;

fn main() {
    use rand_core::RngCore;
    use randomorg::Random;
   
    let mut random = Random::new("API KEY HERE");
    let mut key = [0u8; 16];
    random.fill_bytes(&mut key);
    let random_u64 = random.next_u64();
}
```

## License

This project is
[licensed under the MIT license](https://github.com/iddm/randomorg/blob/master/LICENSE).
