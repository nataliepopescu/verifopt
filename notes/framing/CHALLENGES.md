# Challenges

## Technical

- getting dependent crate MIR did not have an exposed API
    - solution: 
        - [x] iterate through all possible DefIds
        - [ ] identify invalid DefIds (TODO; another challenge)
        - [x] stop @ invalid DefIds

