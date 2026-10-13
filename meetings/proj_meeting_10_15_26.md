# meeting

## agenda

- circular dependencies / scopes -> rewrites
    - order of compilation: rewrites in dependencies
    - a solution: code motion, but CON code size

- finicky size/performance results
    - very sensitive to various code placement/exported fns/etc

- soundness
    - when we do not execute a function (stdlib stubs or return type fallbacks)
      we might be missing calls to functions that end up being rewritten, and
      thus possibly miss type constraints. rewriting without a fallback is thus
      unsound in these cases

## notes

