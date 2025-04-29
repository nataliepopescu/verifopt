# mae meeting

backtracking for conditionals
- if exploring both paths, need to discard the things you learned from the true
  branch when exploring the false branch

apparently almost always need to backtrack
- copy the hashmap (mem) state every time you do this

hashtree?
- adding forks in the path

immutable hashmap; mod returns new hashmap (mut?)
- tree ends up being a tree of maps (compound)
- could also use a list of pairs

lambdas turn into function pointers?

regular functions tend to not be stored in variables
- if stored in variable

eventually, interp will spit out new version of the program
- 3 things returned
    - store
    - expression returned
    - string of program text
- ^ don't worry about this yet

maybe write out a simple parser (for own ease of writing tests, etc)
