# meeting

## agenda

- still trying to sort out smaller examples

## notes

crontab
- longitudinal results
- noise: experiment + machine?

vtable rewrite
- just remove vtables after we check theyre never accessed

fatptr rewrite that also _optimizes_ woul dbe a much bigger lift

lldb
- watchpoint on loading mem - check vtable use
    - still use
    - who knows what optimizations use it downstream
- anything that isn't being rewritten - keep vtable

