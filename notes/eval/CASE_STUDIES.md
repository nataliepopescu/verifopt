# Case Studies

- [ ] ripgrep
    - file regex searcher
    - one last panic! (hopefully)

- [ ] dynolog cli
    - perf telemetry tool (facebook)
    - ~verifopt starts running but hits a panic at 2nd dynamic dispatch call~
    - another error

- [ ] zellij
    - terminal workspace
    - release build time almost 6 min
    - ~hits same panic as dynolog at its 14th dynamic dispatch site~
    - now stack overflow

- [ ] tock

- [ ] quixote
    - "Blazing-fast blockchain event indexer"
    - debug build time for unmodified Rustc was over 3 min
    - release build time > 4 min
    - *no entry fn*
