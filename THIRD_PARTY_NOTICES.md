# Third-Party Notices

This software includes components from third-party open-source projects.
Below is a summary of the licenses for dependencies used in this project.

## License Summary

### Apache-2.0 OR MIT (Majority of dependencies)
The following libraries are dual-licensed under Apache-2.0 OR MIT:
- tokio, anyhow, ndarray, serde, serde_json, sha2, ed25519-dalek (partial)
- chrono, lazy_static, hex, rand, log, env_logger
- And 100+ other transitive dependencies

These licenses permit commercial and proprietary use.

### BSD-3-Clause
The following libraries use BSD-3-Clause license:
- curve25519-dalek
- ed25519-dalek
- subtle

BSD-3-Clause permits commercial and proprietary use with proper attribution.

### BSD-3-Clause-Clear
The following libraries use BSD-3-Clause-Clear license:
- concrete-core v1.0.2
- concrete-csprng v0.2.2

BSD-3-Clause-Clear is similar to BSD-3-Clause but explicitly excludes patent grants.
This license permits commercial and proprietary use with proper attribution.

### MIT OR Unlicense
The following libraries are dual-licensed under MIT OR Unlicense:
- aho-corasick, memchr, jiff

These licenses permit commercial and proprietary use.

### MIT
The following libraries use MIT license:
- aligned-vec, bytes, dyn-stack, generic-array, mio, tokio, zmij
- And several others

MIT license permits commercial and proprietary use with proper attribution.

## Full License Texts

For the full text of each license, please refer to:
- Apache License 2.0: https://www.apache.org/licenses/LICENSE-2.0
- MIT License: https://opensource.org/licenses/MIT
- BSD-3-Clause: https://opensource.org/licenses/BSD-3-Clause
- BSD-3-Clause-Clear: https://spdx.org/licenses/BSD-3-Clause-Clear.html
- Unlicense: https://unlicense.org/

## Verification

To regenerate this list, run:
```bash
cargo install cargo-license
cargo license --all-features
```

## Attribution Requirements

This software complies with the attribution requirements of all included
third-party libraries. The use of these open-source components does not
affect the proprietary nature of this software.

---
Last Updated: 2025-12-29
