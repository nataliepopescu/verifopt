# Case Studies

- [ ] ripgrep
    - file regex searcher
    - one last panic! (hopefully)
    - now runs in 65 min? sus idk what happened
        - also, worst-performers has slightly changed
        - TermInterpStaticCall is #4 now, but idk whats in there
    - changes?
        - rewrite caps variants at 7

        - weird but not crucial to look at right now


- [ ] dynolog cli
    - perf telemetry tool (facebook)
    - ~verifopt starts running but hits a panic at 2nd dynamic dispatch call~
    - ~another error~
    - now stack overflow

- [ ] zellij
    - terminal workspace
    - non-verifopt release build time: almost 6 min
    - ~hits same panic as dynolog at its 14th dynamic dispatch site~
    - now stack overflow


----

- [ ] tock
    - embedded kernel!
    - testing on boards/imix
    - `make` runs in: ~45s
        - tockloader installed
        - where is cargo invoked?
    - non-verifopt release build (via `cargo`) time: ~17s
    - *no entry fn*
    - `--entry-func main`

- [ ] eza
    - ls written in Rust
    - smaller binary
    - non-verifopt release build time: 38s
    - *no entry fn*

- [ ] quixote
    - "Blazing-fast blockchain event indexer"
    - debug build time for unmodified Rustc was over 3 min
    - non-verifopt release build time: >4 min
    - *no entry fn*
