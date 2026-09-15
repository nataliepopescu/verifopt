use visitor_use::{SpeakBetterDogs, get_animal, wrap_dyn_call};
use rand::Rng;

fn main() {
    let num = rand::rng().random_range(..2);
    //let num = 4;

    let a = get_animal(num);
    let dc = &SpeakBetterDogs {};

    wrap_dyn_call(a, dc);
}
