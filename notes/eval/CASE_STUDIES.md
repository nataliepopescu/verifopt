# Case Studies

- [ ] ripgrep
    - file regex searcher
    - one last panic! (hopefully)

- [ ] dynolog cli
    - perf telemetry tool (facebook)
    - ~verifopt starts running but hits a panic at 2nd dynamic dispatch call~
    - ~another error~
    - now stack overflow

- [ ] zellij
    - terminal workspace
    - release build time: almost 6 min
    - ~hits same panic as dynolog at its 14th dynamic dispatch site~
    - now stack overflow

- [ ] tock
    - embedded kernel!
    - testing on boards/imix
    - `make` runs in: ~45s
        - tockloader installed
        - where is cargo invoked?
    - release build (via `cargo`) time: ~17s

----

- [ ] eza
    - ls written in Rust
    - smaller binary
    - release build time: 38s
    - *no entry fn*

- [ ] quixote
    - "Blazing-fast blockchain event indexer"
    - debug build time for unmodified Rustc was over 3 min
    - release build time: >4 min
    - *no entry fn*
