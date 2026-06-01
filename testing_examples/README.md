# Testing Examples

## Statuses

### Compile

- [x] closures
- [x] fnptrs
- [x] generic
- [x] one_variant
- [x] rand_
- [x] shims
- [x] switchint
- [x] two_variants
- [x] two_variants_rand
- [x] two_variants_static
- [ ] two_variants_static_nonzst

### Analyze fully

- [x] closures
- [x] fnptrs
- [x] generic
    - Rect fields are not stored b/c specific to the Rect const allocation layout (maybe this is generalizable but have not yet thought about this)
- [x] one_variant
- [x] rand_
    - not really anything else to analyze
- [ ] shims
    - the candidate list of fns for the fnptr w the signature i32 -> i32 is too
      large, so we are currently falling back to return types
    - ideally we can narrow down this list a lot
- [x] switchint
- [x] two_variants
    - not really anything else to analyze b/c input cannot be statically known...
- [x] two_variants_rand
    - not really anything else to analyze b/c rand cannot be statically known...
- [x] two_variants_static
- [ ] two_variants_static_nonzst
