# rust-mraa [![Build Status][trav-ci-img]][trav-ci]

Bindings for libmraa in Rust

# Overview

rust-mraa is a library that aims to provide bindings for Intel’s libmraa in a idiomatic and abstract manner.

# Requirements

## Rust

We currently compile against the *Master* branch. I’d recommend using the
Nightly installer, as that has the greatest chance of working.

## libmraa

You may find the libmraa sources at https://github.com/intel-iot-devkit/mraa.


# Running on Intel Edison

The following steps are a short overview of how to create an Edison-compatible executable.

* Install the SDK from www.intel.com/support/edison/sb/CS-035180.htm
* Install an i686 version of rust (i.e. via ```multirust update nightly-i686 --installer https://static.rust-lang.org/dist/rust-nightly-i686-unknown-linux-gnu.tar.gz```)
* (Enable the i686 rust version for the current project, i.e. via ```multirust override nightly-i686```)
* Activate the SDK environment setup (i.e. ```source /opt/poky-edison/1.6.1/environment-setup-core2-32-poky-linux```)
* Compile an example 
* Copy example binary to Intel Edison and run there
