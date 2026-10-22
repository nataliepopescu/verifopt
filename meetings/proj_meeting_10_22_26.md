# meeting

## agenda

- moved imix deps into single crate

- for ripgrep, added a "partitioning" change that, based on the analysis
  results, marks different targets as "external-to-CGU" so that rewrites
  outside of said CGU can refer to those targets
    - can change the CGU partitioning results, so additional size baseline
      controls for this change
    - interestingly, the partitioning change often reduces the binary size,
      after which the rewrite inflates it again

    - doesn't apply to tock since tock's CGU == 1

- https://github.com/nataliepopescu/verifopt/blob/ripgrep/STATUS.md#tock-1

- FSA soundness issues makes the no-dyn-fallback switch statement unsound
    - currently using a "CHAmax" baseline: checks which instances are actually
      _compiled_ and uses those as an upper bound of what functions could
      feasibly be called
    - is able to "prove" when there is only a single possible call (from a
      single concrete candidate)

    - can only safely rewrite into no-dyn-fallback switch statements when FSA is
      sound

    - we will likely not get to a point where FSA is provably sound, but at least
      want to get to a point where the obvious soundness errors are taken care of

- new CHAmax baseline
    - kind of being used as an FSA check / fallback
    - i've only been testing with max candidates == 1 (i.e.
      no switch statement rewrites, only direct call swaps)
    - more candidates (i.e., writing a switch statement) needs assume/reason
      about multiple copies of the same function (due to inlining/different 
      CGU copies)

    - ripgrep: mostly benefits from FSA
        - CHAmax is really large due to generic library code

    - tock: mostly benefits from CHAmax

- addressed soundness issues:
    - memoized hits (exact fn calls) weren't replaying callee writes (side
      effects)
    - writes through raw pointers and *mut args
    - statics were not being written
    - writing an array element replaced the whole array
    - smaller bugs...

- remaining soundness issues:
    - pointers stored in fields are not aliased, they are copied
    - missing values read as "nothing" instead of "unknown"
    - ptrs returned into statics are not aliased
    - indirect calls drop return aliases
    - smaller bugs...

- soundness issues in short:
    - skipped-fn side effects, ptrs/aliasing, and statics

## notes


