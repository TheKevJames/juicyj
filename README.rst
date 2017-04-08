JuicyJ
======

JuicyJ Unabashedly Is a Compiler for Your JOOS.

This project was written for the CS 444 "Compilers" course at the University of
Waterloo in Winter 2017. It is meant to compile JOOS 1W, a confined subset of
Java 1.3.

This compiler does not quite pass all the associated test cases, mostly due to
time constraints. I may continue work on this at some point in the future.

FAQ
---

`Course Site`_

Rust Installation
~~~~~~~~~~~~~~~~~

::

    curl -sSf https://sh.rustup.rs | sh
    # complete installation process
    source ~/.cargo/env
    rustup toolchain install 1.14.0
    rustup default 1.14.0
    rustc --version  # should show "rustc 1.14.0 (e8a012324 2016-12-16)"

    cargo install --force rustfmt  # to ensure you have an up-to-date formatter

Documentation
~~~~~~~~~~~~~

Documentation can be built with

::

    cargo doc

and will be located in the compiled documentation folder
(:code:`target/doc/juicyj/index/html`).

Running
~~~~~~~

::

    RUST_LOG=juicyj=debug cargo run  # args...

.. _`Course Site`: https://www.student.cs.uwaterloo.ca/~cs444/
