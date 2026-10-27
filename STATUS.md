# STATUS (thursday's meeting 10/15/26)

## ripgrep

```sh
verifopt_results/verifopt_mir_dumps/anyhow-c489250b6359e3d5.txt: 1 dumped, 1 actually rewritten
verifopt_results/verifopt_mir_dumps/globset-a61f3f012c52ecf2.txt: 8 dumped, 8 actually rewritten
verifopt_results/verifopt_mir_dumps/grep_printer-689eb3b3cd87e62c.txt: 1 dumped, 1 actually rewritten
verifopt_results/verifopt_mir_dumps/grep_regex-8ea328d318a1a895.txt: 1 dumped, 1 actually rewritten
verifopt_results/verifopt_mir_dumps/grep_searcher-061e54d8de2dd8b6.txt: 1 dumped, 0 actually rewritten
verifopt_results/verifopt_mir_dumps/ignore-0d77df7f3e2fe840.txt: 1 dumped, 1 actually rewritten
verifopt_results/verifopt_mir_dumps/log-e908a7ec82829b0e.txt: 1 dumped, 1 actually rewritten
verifopt_results/verifopt_mir_dumps/regex_automata-39ea3a309a67b31a.txt: 2 dumped, 1 actually rewritten
verifopt_results/verifopt_mir_dumps/regex_syntax-645eeec1352d19c1.txt: 2 dumped, 2 actually rewritten
verifopt_results/verifopt_mir_dumps/rg-967863c7843c55ed.txt: 2 dumped, 2 actually rewritten
verifopt_results/verifopt_mir_dumps/termcolor-e114b84a11e267c0.txt: 1 dumped, 0 actually rewritten

[nix-shell:~/hack/verifopt-examples/ripgrep]$ size plain_target/release/rg verifopt_target/release/rg
   text    data     bss     dec     hex filename
4854524  384257     432 5239213  4ff1ad plain_target/release/rg
4862936  384265     432 5247633  501291 verifopt_target/release/rg
```

- missing rewrites need stdlib built from scratch (for verifopt analysis)

## tock

```sh
verifopt_results_verifopt_target/verifopt_mir_dumps/capsules_core-20b305e55f59d557.txt: 10 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/capsules_extra-5377bd100f392d5d.txt: 5 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/components-a90c10349b538f27.txt: 1 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/imix-ac9b027b0546dbb3.txt: 15 dumped, 15 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/kernel-a82edd5bbd98d363.txt: 15 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/sam4l-3716898326bb9396.txt: 12 dumped, 1 actually rewritten

[nix-shell:~/hack/verifopt-examples/tock/boards/imix]$ size plain_target/thumbv7em-none-eabi/release/imix verifopt_target/thumbv7em-none-eabi/release/imix
   text    data     bss     dec     hex filename
 168396       0   31628  200024   30d58 plain_target/thumbv7em-none-eabi/release/imix
 169564       0   31628  201192   311e8 verifopt_target/thumbv7em-none-eabi/release/imix
```

- missing rewrites need some (code-motion?) change b/c targets do not exist
  in the crate being compiled
    - ignore tock structure
    - stick everything into one big crate

- _many_ of existing rewrites are a single target w dynamic fallback -
  i.e. analysis imprecision

- code size culprits:
    - switch table chain
        - how many options in general?
    - inlining
    - fnptr comparisons keep targets in the binary even if they are never called
        - mae's suggestion from a few weeks ago: clean up at binary level

# NEW BUILD (10/16/26)

FSA fallback to CHAmax
- CHAmax considers which functions are _actually_ compiled, while CHA considers
  which functions _could_ be compiled
- thus CHAmax puts an upperbound on the set of functions we'd want to fall back
  to
- in the below results, FSA sometimes finds more candidates than CHAmax, which
  points to a potential precision bug
- in theory, CHAmax is the maximum set of candidates we could actually
  construct a rewrite/switch-case statement for

also fixed a bug in the stdlib stubs impl
- in stubs, widen instead of interpreting

also added logging for tracking number of candidates, if a fn was able to be
interpreted or not, etc

and added max-candidates knob (previously was using 4)

## ripgrep

max candidates == 10:

```sh
verifopt_results_verifopt_target/verifopt_mir_dumps/aho_corasick-1f3b7cec0eca0c09.txt: 12 dumped, 6 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/alloc-136ab265c0b3285d.txt: 2 dumped, 2 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/anyhow-3aea57882015ec1d.txt: 2 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/bstr-bfe789c50cad4960.txt: 3 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/core-c24a5d053bae200d.txt: 8 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/gimli-913b67e7bfcd3a83.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/globset-e18fcc239900d421.txt: 13 dumped, 10 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/grep_cli-0b1d7238e2d71c60.txt: 3 dumped, 3 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/grep_printer-5829ce5c16c41cc6.txt: 3 dumped, 3 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/grep_regex-834f97686c168d7f.txt: 3 dumped, 3 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/grep_searcher-e40ea5566a2656b1.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/ignore-c1d98d8019f0f284.txt: 6 dumped, 5 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/lexopt-e8ff7f4c4b5937df.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/log-3103b24831d08714.txt: 1 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/object-96f931f946639d35.txt: 2 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/proc_macro-6a314f8d1330e9bc.txt: 4 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/regex_automata-70131cc4b7a5b422.txt: 22 dumped, 13 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/regex_syntax-ec82b9fc3448b694.txt: 7 dumped, 7 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/rg-4789f286e4aa02f2.txt: 12 dumped, 7 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/serde-080c5c30ad7d7ed2.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/serde_core-de0e583ea931987e.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/serde_json-c9305652766a0326.txt: 3 dumped, 3 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/std-b4270c738501939e.txt: 15 dumped, 14 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/termcolor-99cb7f7307757c48.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/walkdir-629828d1005faf9c.txt: 2 dumped, 0 actually rewritten

      2 pointers n=1
      1 pointers n=10 of=11
      8 pointers n=2
     11 pointers n=3
      2 pointers n=4
     10 pointers n=7
      9 pointers n=8
     43 single
      1 single exact
      3 skipped too_many_targets n=101
     60 skipped too_many_targets n=104
     18 skipped too_many_targets n=12
      4 skipped too_many_targets n=14
      3 skipped too_many_targets n=27
      3 skipped too_many_targets n=34
      1 skipped too_many_targets n=37
      1 skipped too_many_targets n=95

[nix-shell:~/hack/verifopt-examples/ripgrep]$ size plain_target/x86_64-unknown-linux-gnu/release/rg verifopt_target/x86_64-unknown-linux-gnu/release/rg
   text    data     bss     dec     hex filename
4509880  347905     440 4858225  4a2171 plain_target/x86_64-unknown-linux-gnu/release/rg
4543884  348409     424 4892717  4aa82d verifopt_target/x86_64-unknown-linux-gnu/release/rg
```

- still 2 pointers (switch statements w fallbacks) where only one concrete case
  - check CHAmax here

max candidates == 1:

```sh
verifopt_results_verifopt_target/verifopt_mir_dumps/aho_corasick-1f3b7cec0eca0c09.txt: 6 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/alloc-136ab265c0b3285d.txt: 2 dumped, 2 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/anyhow-3aea57882015ec1d.txt: 1 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/bstr-bfe789c50cad4960.txt: 2 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/core-c24a5d053bae200d.txt: 7 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/gimli-913b67e7bfcd3a83.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/globset-e18fcc239900d421.txt: 2 dumped, 2 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/grep_cli-0b1d7238e2d71c60.txt: 3 dumped, 3 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/grep_printer-5829ce5c16c41cc6.txt: 3 dumped, 3 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/grep_regex-834f97686c168d7f.txt: 2 dumped, 2 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/grep_searcher-e40ea5566a2656b1.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/ignore-c1d98d8019f0f284.txt: 2 dumped, 2 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/lexopt-e8ff7f4c4b5937df.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/log-3103b24831d08714.txt: 1 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/object-96f931f946639d35.txt: 2 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/proc_macro-6a314f8d1330e9bc.txt: 1 dumped, 1 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/regex_automata-70131cc4b7a5b422.txt: 9 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/regex_syntax-ec82b9fc3448b694.txt: 7 dumped, 7 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/rg-4789f286e4aa02f2.txt: 2 dumped, 2 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/serde-080c5c30ad7d7ed2.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/serde_core-de0e583ea931987e.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/serde_json-c9305652766a0326.txt: 3 dumped, 3 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/std-b4270c738501939e.txt: 14 dumped, 13 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/termcolor-99cb7f7307757c48.txt: 1 dumped, 0 actually rewritten
verifopt_results_verifopt_target/verifopt_mir_dumps/walkdir-629828d1005faf9c.txt: 2 dumped, 0 actually rewritten

     43 single
      1 single exact
      3 skipped too_many_targets n=101
     60 skipped too_many_targets n=104
     18 skipped too_many_targets n=12
      4 skipped too_many_targets n=14
      1 skipped too_many_targets n=2
      3 skipped too_many_targets n=27
      2 skipped too_many_targets n=3
      3 skipped too_many_targets n=34
      1 skipped too_many_targets n=37
      7 skipped too_many_targets n=4
     10 skipped too_many_targets n=5
      5 skipped too_many_targets n=6
      1 skipped too_many_targets n=7
     12 skipped too_many_targets n=8
     19 skipped too_many_targets n=9
      1 skipped too_many_targets n=95

[nix-shell:~/hack/verifopt-examples/ripgrep]$ size plain_target/x86_64-unknown-linux-gnu/release/rg verifopt_target/x86_64-unknown-linux-gnu/release/rg
   text    data     bss     dec     hex filename
4509880  347905     440 4858225  4a2171 plain_target/x86_64-unknown-linux-gnu/release/rg
4511080  348289     424 4859793  4a2791 verifopt_target/x86_64-unknown-linux-gnu/release/rg
```

- size is closer but still worse in ripgrep, inlining due to rewrites
    - tested, _not_ the partitioning change

    - however, the partitioning hook _does_ change layouts, so it should be used
      at the baseline, not a build without it, to isolate the effects of the
      rewrites
        - note: need a store present for the hook to run properly

## tock

after putting dependencies w rewrites in imix crate

max candidates == 4:

```sh
verifopt_results_verifopt_target/verifopt_mir_dumps/imix-a7028f869d0b08c6.txt: 48 dumped, 43 actually rewritten

      5 pointers n=2
      1 pointers n=4
      7 single
     99 single exact
      2 skipped too_many_targets n=13
      1 skipped too_many_targets n=14
      1 skipped too_many_targets n=21
      1 skipped too_many_targets n=39
      2 skipped too_many_targets n=7
      2 skipped too_many_targets n=81
      3 skipped too_many_targets n=9

[nix-shell:~/hack/verifopt-examples/tock/boards/imix]$ size plain_target/thumbv7em-none-eabi/release/imix verifopt_target/thumbv7em-none-eabi/release/imix
   text    data     bss     dec     hex filename
 167200       0   31324  198524   3077c plain_target/thumbv7em-none-eabi/release/imix
 167236       0   31324  198560   307a0 verifopt_target/thumbv7em-none-eabi/release/imix
```

max candidates == 1:

```sh
verifopt_results_verifopt_target/verifopt_mir_dumps/imix-a7028f869d0b08c6.txt: 45 dumped, 41 actually rewritten

      7 single
     99 single exact
      2 skipped too_many_targets n=13
      1 skipped too_many_targets n=14
      6 skipped too_many_targets n=2
      1 skipped too_many_targets n=21
      1 skipped too_many_targets n=39
      1 skipped too_many_targets n=4
      2 skipped too_many_targets n=7
      2 skipped too_many_targets n=81
      3 skipped too_many_targets n=9

[nix-shell:~/hack/verifopt-examples/tock/boards/imix]$ size plain_target/thumbv7em-none-eabi/release/imix verifopt_target/thumbv7em-none-eabi/release/imix
   text    data     bss     dec     hex filename
 167200       0   31324  198524   3077c plain_target/thumbv7em-none-eabi/release/imix
 166940       0   31324  198264   30678 verifopt_target/thumbv7em-none-eabi/release/imix
```

- size win if no switch statements and only single-candidate (exact) rewrites!

# NEW BUILD (10/17/26)

fix target creation when CHA finds no candidates

fix rewrite unsoundness due to fnptr fallbacks
- plus try to recover some fnptr precision

closure captures fix
- big impact on analysis time b/c state starts to grow again, working on alleviating this

## ripgrep

max candidates == 1

```sh
      9 pointers n=1
      2 single exact
      1 skipped too_many_targets n=10
      2 skipped too_many_targets n=100
      4 skipped too_many_targets n=101
     60 skipped too_many_targets n=104
     18 skipped too_many_targets n=12
      3 skipped too_many_targets n=15
     75 skipped too_many_targets n=16
      3 skipped too_many_targets n=17
     13 skipped too_many_targets n=2
      4 skipped too_many_targets n=256
      8 skipped too_many_targets n=3
      4 skipped too_many_targets n=3149
      1 skipped too_many_targets n=3150
      1 skipped too_many_targets n=3151
      5 skipped too_many_targets n=4
      5 skipped too_many_targets n=5
      5 skipped too_many_targets n=6
      1 skipped too_many_targets n=7
     11 skipped too_many_targets n=8
     22 skipped too_many_targets n=9
      1 skipped too_many_targets n=95

[nix-shell:~/hack/verifopt-examples/ripgrep]$ size baseline_target/x86_64-unknown-linux-gnu/release/rg partitioning_baseline_target/x86_64-unknown-linux-gnu/release/rg clean_target/x86_64-unknown-linux-gnu/release/rg
   text    data     bss     dec     hex filename
4509880  347905     440 4858225  4a2171 baseline_target/x86_64-unknown-linux-gnu/release/rg
4502756  348145     424 4851325  4a067d partitioning_baseline_target/x86_64-unknown-linux-gnu/release/rg
4504612  348145     424 4853181  4a0dbd clean_target/x86_64-unknown-linux-gnu/release/rg
```

- we added a partitioning change that marks some functions external that weren't
  before, so that rewrites that need crate items from other CGUswithin the same
  crate can actually access them
- turns out this change is actually responsible for a healthy size reduction,
  apart from the rewrites themselves


# NEW BUILD (10/20)

- fix memo bug
    - when hit memo limit, interp w concrete args but store in widened args -
      incorrect!
    - was also ignoring side effects (doesn't register writes through arg ref,
      statics, interior mutability, raw ptrs, etc) / callee writes

- fixed memory leak that was affecting ripgrep

- fixed performance bug that was stalling the tock run


## ripgrep

analysis still over 3 hours long

```sh
      9 pointers n=1
      2 single exact
      1 skipped too_many_targets n=10
      2 skipped too_many_targets n=100
      4 skipped too_many_targets n=101
     60 skipped too_many_targets n=104
     18 skipped too_many_targets n=12
      3 skipped too_many_targets n=15
     75 skipped too_many_targets n=16
      3 skipped too_many_targets n=17
     13 skipped too_many_targets n=2
      4 skipped too_many_targets n=256
      8 skipped too_many_targets n=3
      4 skipped too_many_targets n=3149
      1 skipped too_many_targets n=3150
      1 skipped too_many_targets n=3151
      5 skipped too_many_targets n=4
      5 skipped too_many_targets n=5
      5 skipped too_many_targets n=6
      1 skipped too_many_targets n=7
     11 skipped too_many_targets n=8
     22 skipped too_many_targets n=9
      1 skipped too_many_targets n=95

[nix-shell:~/hack/verifopt-examples/ripgrep]$ size plain_target/x86_64-unknown-linux-gnu/release/rg hook_target/x86_64-unknown-linux-gnu/release/rg timing_target/x86_64-unknown-linux-gnu/release/rg
   text    data     bss     dec     hex filename
4509880  347905     440 4858225  4a2171 plain_target/x86_64-unknown-linux-gnu/release/rg
4502756  348145     424 4851325  4a067d hook_target/x86_64-unknown-linux-gnu/release/rg
4504612  348145     424 4853181  4a0dbd timing_target/x86_64-unknown-linux-gnu/release/rg
```

- partitioning hook reduces binary size
- rewrite adds size back

## tock

analysis down from 12 hours to 42 minutes

```sh
      2 pointers n=1
    134 single exact
      1 skipped too_many_targets n=11
      2 skipped too_many_targets n=13
      1 skipped too_many_targets n=14
      1 skipped too_many_targets n=1968
      1 skipped too_many_targets n=1969
      7 skipped too_many_targets n=2
      1 skipped too_many_targets n=22
      1 skipped too_many_targets n=39
      2 skipped too_many_targets n=4
      2 skipped too_many_targets n=7
      4 skipped too_many_targets n=8
      2 skipped too_many_targets n=82
     14 skipped too_many_targets n=9

[nix-shell:~/hack/verifopt-examples/tock/boards/imix]$ size plain_target/thumbv7em-none-eabi/release/imix hook_target/thumbv7em-none-eabi/release/imix timing_target/thumbv7em-none-eabi/release/imix
   text    data     bss     dec     hex filename
 167200       0   31324  198524   3077c plain_target/thumbv7em-none-eabi/release/imix
 167200       0   31324  198524   3077c hook_target/thumbv7em-none-eabi/release/imix
 166676       0   31324  198000   30570 timing_target/thumbv7em-none-eabi/release/imix
 ```

- partitioning hook does not change binary size (CGU already == 1)
- rewrite reduces size

# NEW BUILD (10/21)

- fallback to CHAmax (candidate's compiled instances)

- better pointer model
    - write aliasing instead of copying when *mut args, raw ptrs, interior mutability, etc

- actually write statics

## ripgrep

## tock

```sh
      2 pointers n=1
    270 single exact
     14 skipped too_many_targets n=10
      2 skipped too_many_targets n=100
      1 skipped too_many_targets n=11
      2 skipped too_many_targets n=13
      1 skipped too_many_targets n=14
      1 skipped too_many_targets n=1968
      1 skipped too_many_targets n=1970
     11 skipped too_many_targets n=2
      2 skipped too_many_targets n=33
      1 skipped too_many_targets n=39
      5 skipped too_many_targets n=4
      1 skipped too_many_targets n=40
      2 skipped too_many_targets n=7
      8 skipped too_many_targets n=9

[nix-shell:~/hack/verifopt-examples/tock/boards/imix]$ size plain_target/thumbv7em-none-eabi/release/imix statics_target/thumbv7em-none-eabi/release/imix
   text    data     bss     dec     hex filename
 167200       0   31324  198524   3077c plain_target/thumbv7em-none-eabi/release/imix
 166104       0   31324  197428   30334 statics_target/thumbv7em-none-eabi/release/imix
```

binary size is 1,096 B smaller!

- no vtables eliminated (two diff numbers, based on a heuristic scan):
    - 120 vtables (2,896 B)
    - all vtables together are only about 3.1 KB (132 vtables in plain)
        - the later number
    - 158? (3.6 KB)

- rewritten calls get inlined into their callers

  ┌──────────────────────────────────────────────────────────────────────┬───────┬─────────────────────────────────────────────────────┐
  │                                Trait                                 │ Sites │           Target (the only compiled impl)           │
  ├──────────────────────────────────────────────────────────────────────┼───────┼─────────────────────────────────────────────────────┤
  │ Process                                                              │ 212   │ ProcessStandard<Sam4l<…>, ProcessStandardDebugFull> │
  ├──────────────────────────────────────────────────────────────────────┼───────┼─────────────────────────────────────────────────────┤
  │ usb Client                                                           │ 10    │ usbc_client::Client<Usbc>                           │
  ├──────────────────────────────────────────────────────────────────────┼───────┼─────────────────────────────────────────────────────┤
  │ SchedulerTimer                                                       │ 8     │ SysTick                                             │
  ├──────────────────────────────────────────────────────────────────────┼───────┼─────────────────────────────────────────────────────┤
  │ flash Client<FLASHCALW> + NonvolatileStorage                         │ 10    │ NonvolatileToPages<FLASHCALW>                       │
  ├──────────────────────────────────────────────────────────────────────┼───────┼─────────────────────────────────────────────────────┤
  │ DebugWriter                                                          │ 5     │ UartDebugWriter                                     │
  ├──────────────────────────────────────────────────────────────────────┼───────┼─────────────────────────────────────────────────────┤
  │ NineDof                                                              │ 4     │ Fxos8700cq                                          │
  ├──────────────────────────────────────────────────────────────────────┼───────┼─────────────────────────────────────────────────────┤
  │ analog-comparator / adc / crc Client                                 │ 6     │ the one driver each                                 │
  ├──────────────────────────────────────────────────────────────────────┼───────┼─────────────────────────────────────────────────────┤
  │ about 15 others (I2C, USART, GPIO Interrupt, Sixlowpan, UDP, Rng, …) │ 17    │                                                     │
  └──────────────────────────────────────────────────────────────────────┴───────┴─────────────────────────────────────────────────────┘
  
  - Mostly Process. Process::processid (85 sites) and Process::enter_grant (62) alone are over half of all rewrites.
  - Where the sites are:
    - 120 are in the many instances of ProcessGrant::access_grant_with_allocator, one per capsule.
    - 20 are in Kernel::do_process and 14 in memop.
    - The rest are scattered through the syscall path and the chip drivers' interrupt closures.
  - The 2 guarded sites are USART as uart::Transmit and USART as uart::Receive.
  - What's left untouched: the 52 too_many_targets sites keep their dyn calls. They include SyscallDriver dispatch (about 1,970 candidates) and
    Ast::handle_interrupt. 

inlining itself removes instructions:

  plain:      ldr r2, [r6, #0xc]   ; vtable slot of Process::processid                                                                                
              mov r0, sp                                                                                                                              
              mov r1, r4                                                                                                                              
              blx r2                                                                                                                                  
  rewritten:  ldr r6, [sp, #0xc]   ; processid inlined: a single field load

vs VFE (LLVM's virtual function elimination): (`-Zvirtual-function-elimination`)
- only works w LTO
    - is LTO already enabled in the non-VFE build? yes
- also, VFE adds extra trait tags to vtables + dyn calls

Enabling LLVM's existing virtual-function elimination (VFE) saves far more than our rewrite: −5,400 B of .text on its own, against −1,096 B for the
  rewrite. With both, it's −5,640 B, so on top of VFE the rewrite adds only −240 B. Whole vtables still don't disappear, even with both.

  ┌─────────────────┬─────────┬──────────┬─────────────────────────┐
  │      Build      │  .text  │ vs plain │ Indirect calls (blx rN) │
  ├─────────────────┼─────────┼──────────┼─────────────────────────┤
  │ plain           │ 164,068 │          │ 731                     │
  ├─────────────────┼─────────┼──────────┼─────────────────────────┤
  │ rewritten       │ 162,972 │ −1,096   │ 526                     │
  ├─────────────────┼─────────┼──────────┼─────────────────────────┤
  │ plain + VFE     │ 158,668 │ −5,400   │ 250                     │
  ├─────────────────┼─────────┼──────────┼─────────────────────────┤
  │ rewritten + VFE │ 158,428 │ −5,640   │ 249                     │
  └─────────────────┴─────────┴──────────┴─────────────────────────┘

- Where the rewrite still adds on top of VFE:
  - In ProcessStandard's Process vtable, 15 of 60 slots become null instead of 9.
  - A few more usb, flash and Sixlowpan slots are emptied.
  - 13 more functions are removed (328 B).

- no vtables are eliminated in _any_ of the four builds

- VFE has already emptied the slots that matter, so
  dropping whole vtables would add at most a few hundred bytes.

- the difference between rewritten and rewritten+VFE could be the vtable-removal
  that verifopt currently lacks

a bounded gain, a.k.a. "speed of light"
- what is the theoretical maximum here?

  What this means for the evaluation. For tock's size, the baseline to compare against is plain + VFE, not plain. Against it, verifopt saves 240 B: it
  removes some vtable slots LLVM can't prove dead, but its devirtualization adds little. Where verifopt can still win over VFE is at sites with more
  than one implementation that FSA narrows down. In tock that's rare (FSA falls back to CHA at 332 of 369 sites), but ripgrep is the case where FSA
  matters. A ripgrep + VFE comparison is worth doing once its run finishes. That one is a performance comparison, since LLVM's devirtualization helps
  speed too.

224 dyn dispatch locations (338 calls) were never recorded by the analysis
- interpreter currently ignores drops (it shouldn't b/c these are just like any
  other dyn call)
  drops)
    - 108 calls
- capsule code
    - analysis skips CHA calls w generic params (including lifetime params) b/c
      they're not correctly compared
    - 230 calls

-> more coverage


# NEW BUILD (10/23)

WAIT FOR RIPGREP

- analyze drop glue

- fix CHA generic params mismatch issue



