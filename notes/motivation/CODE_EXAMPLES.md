# Motivating Code Examples

## Tock capsule client callbacks

trait: `dyn time::AlarmClient`
- define `alarm()`
    - callback for when alarm fires

exemplifies a general pattern in tock
- multiple steps of registering clients
- different "valid" sets of clients at each step (in particular, the lowest chip
  step should only ever have a single client at runtime, but other steps may
  have many, so merely collecting the total set of clients used/registered would
  be insufficient)

