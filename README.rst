JuicyJ
======

JuicyJ Unabashedly Is a Compiler for Your JOOS.

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
