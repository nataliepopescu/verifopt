
ignore constants

recusive machinery
- when get to recursive call, put pin in it
- when return, check if pin type is same at rettype
- if not, redo analysis w more precise constraints until convergence (which we can assume since struct impls are not infinite)


function nsummmary
- only rnter func if dont hqave complete mapping

recursive limit (~50 or smthg, tunnable)
- after, fall back to type
- integers: widen to type

handle loops the same way
- a little harder b/c capture "env"
- need to see what is actually used
- what isn't decl in loop == loop arg
- record arg variable values/constraints after loop (those are your ret vals)

