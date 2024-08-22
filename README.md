# Distal
## Exploration of Distributed Algorithms in Rust

Start with learning the [Actix](https://github.com/actix/actix) actor framework.
Implement some distributed message passing algorithms using Actix 

Then implement some other distributed algorithms using shared memory

Algorithms are pulled from [Distributed Algortihms, An Intuitive Approach](https://mitpress.mit.edu/9780262037662/distributed-algorithms/)

## TODO
- [ ] factor out ping actor into its own module
- [ ] add `started()` method
- [ ] add `stop()` method
- [ ] modify actor to run the collatz conjecture and return the number of steps
