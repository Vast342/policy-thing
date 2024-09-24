# Policy-Thing

This is the repository of my currently-unnamed policy network trainer made in Rust. Currently, it only supports (768->1)x384, which isn't even a network, but in the future that will change.

Todo List:
- [x] Barebones Functionality
- [x] Train a first policy "net"
- [ ] Optimisation
    - [ ] Simplify Box<T> shenanigans to reduce time wasted on copying 
    - [ ] `pop_lsb()` loops instead of `0..64` when looping over board
    - [ ] Anything else that I can find
- [ ] Better Data Format
    - [ ] N moves in the format (more than 32) and only train positions where legal move count <= N
    - Note: Needs to be done inside Anura's data generation as well
- [ ] Actual Network Support
    - [ ] Research arch considerations and activation functions generally used
    - [ ] Partial Derivatives (wooooooo)
    - [ ] Implementation
- [ ] More Features
    - [ ] Extra LR scheduling (just non-linear at least)
    - [ ] Adam / AdamW
    - [ ] More layers?
    - [ ] Alternate Activation Functions
    - [ ] Maybe support more data formats in the distant future
