use dst_explore::SeedState;
use rand::prelude::*;

fn main() {
    let mut rng = SmallRng::from_rng(&mut rand::rng());
    #[derive(SeedState, Debug)]
    enum State {
        StateOne,
        StateTwo,
        StateThree,
    }

    State::seed(rng.random_range(0..100));
}

pub trait SeedState {
    fn seed(seed: usize);
}
