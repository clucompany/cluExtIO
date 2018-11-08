# cluExtIO

[![Build Status](https://travis-ci.org/clucompany/cluExtIO.svg?branch=master)](https://travis-ci.org/clucompany/cluExtIO)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluextio)](https://crates.io/crates/cluextio)
[![Documentation](https://docs.rs/cluextio/badge.svg)](https://docs.rs/cluextio)

Syntactic sugar extends I/O capabilities.


# Capabilities:
1. EmptyWrite - Empty 'Write' that does nothing.
2. UnionWrite - Possibility to combine several 'Write' into one record.
3. MutexWrite - Combining Mutex and Write for multi-threaded access.
4. ExtWrite - The trait extends the capabilities of the standard Write, adds lock methods.
5. FlushWrite - An implementation of "Trait Write", which calls the flush () method on drop. 
...



# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the MIT License
