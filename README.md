# Distal
## Exploration of Distributed Algorithms in Rust

Start with learning the [Actix](https://github.com/actix/actix) actor framework.
Implement some distributed message passing algorithms using Actix 

Then implement some other distributed algorithms using shared memory

Algorithms are pulled from [Distributed Algortihms, An Intuitive Approach](https://mitpress.mit.edu/9780262037662/distributed-algorithms/)

## TODO
- [x] factor out ping actor into its own module
- [x] implement collatz logic
- [ ] refactor CollatzActor to be able to handle multiple requests at the same time by spining up new actors
