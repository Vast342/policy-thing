# Policy-Thing

This is the repository of my currently-unnamed policy network trainer made in Rust. Currently, it only supports (768->1)x384, which isn't even a network, but in the future that will change.

Todo List:
- [x] Barebones Functionality
- [x] Train a first policy "net"
- [ ] Optimisation
    - [ ] Simplify Box<T> shenanigans to reduce time wasted on copying network
    - [ ] During Datapoint->mailbox conversion, count how many moves there are and use that instead of 0..32 loops all over the place
    - [ ] Anything else that I can find
- [ ] Better Data Format
    - [ ] Mine is currently bad and inefficient,
    - [ ] All Moves represented
    - [ ] or at least random selection of moves
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
