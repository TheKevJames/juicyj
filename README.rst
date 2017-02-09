JuicyJ
======

JuicyJ Unabashedly Is a Compiler for Your JOOS. `JuicyJ Docs`_.

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

Running
~~~~~~~

::

    RUST_LOG=juicyj=debug cargo run  # args...

.. _`Course Site`: https://www.student.cs.uwaterloo.ca/~cs444/
.. _`JuicyJ Docs`: https://circleci.com/api/v1/project/TheKevJames/juicyj/latest/artifacts/0/$CIRCLE_ARTIFACTS/docs/juicyj/index.html?branch=master&filter=successful
