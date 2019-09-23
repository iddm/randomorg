extern crate randomorg;

fn main() {
    use randomorg::Random;
    use std::env;

    let r = Random::new(env::var("RANDOM_ORG_API_KEY").unwrap());
    println!(
        "Result: {:?}",
        r.generate_integers(-100, 100, 15, true).unwrap()
    );
}
