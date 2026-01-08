mod M1 {
    use builtin::*;

    // FIXME does not compile, but example copied from
    // https://verus-lang.github.io/verus/guide/proof_functions.html
    pub closed spec fn min(x: int, y: int) -> int {
        if x <= y {
            x
        } else {
            y
        }
    }

    pub proof fn lemma_min(x: int, y: int)
        ensures
            min(x, y) <= x,
            min(x, y) <= y,
            min(x, y) == x || min(x, y) == y,
    {
    }
}

mod M2 {
    use builtin::*;
    use crate::M1::*;

    proof fn test() {
        lemma_min(10, 20);
        assert(min(10, 20) == 10); // succeeds
        assert(min(100, 200) == 100); // FAILS
    }
}
