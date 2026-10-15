# meeting

## agenda

- circular dependencies / scopes -> rewrites
    - order of compilation: rewrites in dependencies
    - a solution: code motion, but CON code size
    - code size is already larger...

- finicky size/performance results
    - very sensitive to various code placement/exported fns/etc

- soundness
    - when we do not execute a function (stdlib stubs or return type fallbacks)
      we might be missing calls to functions that end up being rewritten, and
      thus possibly miss type constraints. rewriting without a fallback is thus
      unsound in these cases

## status

- ripgrep status

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

- tock status

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

    - _many_ of existing rewrites are a single target w dynamic fallback -
      i.e. analysis imprecision

- code size culprits

    - switch table chain
        - how many options in general?
    - inlining
    - fnptr comparisons keep targets in the binary even if they are never called
        - mae's suggestion from a few weeks ago: clean up at binary level

## notes

