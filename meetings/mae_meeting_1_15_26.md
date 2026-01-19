# meeting

## agenda

- stuck getting MIR (in particular, func defs) of dependent crates
    1. technical issues getting Callback-based crate to compile w our custom
       rustc
    2. even if we _were_ to get it to compile, would it still be possible to get
       all dep MIR in this way? according to a bjorn comment in the latest zulip
       thread it might not be? (comment confused me)

## notes

more worried about MIR not being emitted than a proper API not being there
- suggests: scanning the DefId space to try to get MIR for anything you can
- if can get MIR for random DefIds, then great, can probably string together
  some weird calls
- if can't, then that's the next/first thing we should figure out

