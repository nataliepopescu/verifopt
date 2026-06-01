# Testing Examples

## Statuses

### Compile

- [x] closures
- [x] fnptrs
- [ ] generic
- [ ] one_variant
- [x] rand_
- [x] shims
- [x] switchint
- [ ] two_variants
- [ ] two_variants_rand
- [ ] two_variants_static
- [ ] two_variants_static_nonzst

### Analyze fully

- [x] closures
- [x] fnptrs
- [ ] generic
- [ ] one_variant
- [ ] rand_
    - not really anything else to analyze
- [ ] shims
    - the candidate list of fns for the fnptr w the signature i32 -> i32 is too
      large, so we are currently falling back to return types
    - ideally we can narrow down this list a lot
- [x] switchint
- [ ] two_variants
- [ ] two_variants_rand
- [ ] two_variants_static
- [ ] two_variants_static_nonzst
