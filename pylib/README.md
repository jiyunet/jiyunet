# pylib

This module is basically just a C-like front-end for Jiyunet APIs, which are all
in Rust and use a lot of fancy high-level abstractions.  Since those are hard
for other languages to understand we need an interface with a bunch of things
tagged with `extern "C"`.

The goal of this is to provide some functionality to work with the cryptographic
systems and work with the DAG data structures from Python.  With that we'll end
up building a C API (as the Python code just calls that) and with it we'll be
able to build wrappers for whichever languages we want, like Crystal, D, etc.

Some languages like Java or Elixir that don't have seamless C interop we'd want
to build up the code from scratch, and it would probably be faster anyways.  So
this wouldn't be all that useful.
