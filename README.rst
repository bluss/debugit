
debugit
=======

Debug-print any value without trait bounds using specialization (Rust nightly
channel). Requires Rust nightly.

Please read the `API documentation here`__

__ https://docs.rs/debugit

|build_status|_ |crates|_

.. |crates| image:: https://img.shields.io/crates/v/debugit.svg
   :alt: debugit at crates.io
.. _crates: https://crates.io/crates/debugit

Recent Changes
--------------

- 0.2.0

  - Make debugit nightly-only and require ``unsafe`` to call due
    to soundness issues with specialization.

- 0.1.2

  - Use ``version_matches`` for version checking (compiles faster)

- 0.1.1

  - Edit docs
  - Fix repository link

License
-------

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.



