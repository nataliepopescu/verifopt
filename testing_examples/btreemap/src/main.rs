// Minimal example exercising BTreeMap<K, Box<dyn Trait>>:
//   - BTreeMap::new()   -> stub_constructor
//   - .insert(k, v)     -> stub_insert (merges Dog/Cat's constraints into
//                          the synthetic element slot)
//   - .iter()           -> stub_read (hands back the element slot's
//                          constraints for the loop variable)
//   - animal.speak()    -> a dyn dispatch call whose FSA candidate set
//                          should resolve to {Dog, Cat} by reading straight
//                          through the BTreeMap's synthetic element slot,
//                          instead of the interpreter descending into
//                          BTreeMap's real (NodeRef/NonNull-based) body.

use std::collections::BTreeMap;

trait Speak {
    fn speak(&self) -> String;
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) -> String {
        "Meow".to_string()
    }
}

fn main() {
    let mut animals: BTreeMap<String, Box<dyn Speak>> = BTreeMap::new();
    animals.insert("dog".to_string(), Box::new(Dog));
    animals.insert("cat".to_string(), Box::new(Cat));

    for (name, animal) in animals.iter() {
        // Dynamic dispatch: without the stub, resolving this call requires
        // walking through BTreeMap's real internal node representation to
        // find out what's stored at this key. With the stub, FSA should
        // read the constraint straight off the map's synthetic element
        // slot and immediately know the candidate set is {Dog, Cat}.
        //println!("{name}: {}", animal.speak());
        std::hint::black_box(animal.speak());
    }
}
