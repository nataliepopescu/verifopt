# meeting

## agenda

- full ripgrep (interp) run
    - plan: looking at top-worst performers and going from there

        - currently: StmtNewConstraintsFromConvert
            - converting an MIR Rvalue into a Verifopt Rval

    - what kind of performance should we even be targeting?
        - obviously faster is better, but how to prioritize this?

- running verifopt on more binaries
    - trying dynolog/cli + zellij -> more error-whack-a-mole there

- tracking too many variables - checked and usually only in the hundreds (max)
  per scope, but of course this could blow up with many scoped, so still 
  potential optimization opportunity

## notes

bitmap of traits (instead of list of traits); variants
- should def do this

how many copies of the enivronment we have at any given time
- easiest way to do this: static int
- wrap immut map in struct that incs/decs that static
- can get a high-watermark

perf
- much more important to understand _why_ something performs as it does (vs
  getting it to a certain point)

currently: could run w the overnight release build (time)

need rewrites to determine how much more we want to sacrifice (perf vs
precision)
- i.e. better perf w/out sacrificing precision
- dont want to decide until we know effect on binary

development iteration speed
- get claude to "minimize" examples for faster iteration

_scaling_ performance also important (linear vs exponential/more)

opus? fable?

add branch-awareness
- when take a branhc, keep around what we learned from the conditional

fn summaries that preserve all conditions == chunky, so..
maybe also summary specialization
- are any invocations (params) partially-reused

maybe double check loops?
- but also just be guided by where the tool freaks out/is wrong


conclusion:
- improve perf where we're doing dumb things
- wait to make precision tradeoffs until we know how itll impact the final
  binary performance (of the code we're optimizing)
- functionality: address when we hit a panic/incorrect result (once we
  eventually test the rewritten binaries)
- kinda need to start codegening
