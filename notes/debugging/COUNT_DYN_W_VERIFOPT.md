# Counting Dyn Dispatch using VerifOpt

## Mismatch between what Interp and Rewrite think are dyn dispatch

Interp:
- check via mir availability
- perhaps track/label calls somehow for rewrite so don't need to recompute this set?

Rewrite:
- if first arg is dyn (this is probably wrong)
