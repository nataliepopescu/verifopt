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

