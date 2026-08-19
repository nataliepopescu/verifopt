# Case Studies

- [ ] ripgrep
    - file regex searcher
    - verifopt runs quite far but keeps hitting various panics + trying to
      diagnose performance slowness

- [ ] dynolog cli
    - perf telemetry tool (facebook)
    - verifopt starts running but hits a panic at 2nd dynamic dispatch call

- [ ] zellij
    - terminal workspace
    - release build time almost 6 min
    - hits same panic as dynolog at its 14th dynamic dispatch site

- [ ] tock

- [ ] quixote
    - "Blazing-fast blockchain event indexer"
    - debug build time for unmodified Rustc was over 3 min
    - release build time > 4 min
    - *no entry fn*
