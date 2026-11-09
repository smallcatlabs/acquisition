**Range `4d3ab4a4^..d9d882a7`, TLS-stack work only (4d3ab4a4, 46b80bfb, N52's sentence in d9d882a7): the done-when holds in form but not in substance.** The claim (N52), the ruling (C169), the manifest comment and a pin all exist. The pin holds less than C169's sentence and its own comments state, and the owner's ruling on C169's scope is a blanket assent with one clause still awaiting his re-read. Gate and breaker results: not verified.

I staged every "confirmed" case below in a `git archive` copy of d9d882a7 under my scratchpad and ran `tools/arch-check.sh` there; the repository itself was not touched.

## Findings, most severe first

**1. `tools/arch-check.sh:231-235` and `crates/acquisition-daemon/Cargo.toml:56` — shapes 7, 2. A feature dropped from the transport's reqwest line passes the check.**
- **Fault:** the reader takes the `--all-features` resolve, which is the union over every edge, including the daemon's dev-dependency copy of the same reqwest line. Anything the transport drops, the dev edge puts back.
- **Confirmed inputs, both exit 0 with `ok dependencies … the TLS stack as named`:**
  - transport `features = ["json"]` (production `acqd` has no TLS at all);
  - transport `features = ["json", "rustls-tls-webpki-roots-no-provider"]` (production reqwest has no ring provider of its own and takes whatever provider the process installs, or panics).
- **Sibling sentence:** `crates/acquisition-transport/Cargo.toml:20-22` says "A feature added or dropped here … check 5 reads the resolved set". "Dropped" is false.
- **Reverse direction, confirmed:** adding `http2` to the daemon's dev line alone refuses, although the production client is unchanged. The refusal text then tells the reader the registration's stack changed.
- **Smallest fix:** besides the resolve, hold the declared edge. Read `.packages[] | select(.name=="acquisition-transport") | .dependencies[] | select(.name=="reqwest")` and require its `features`, `uses_default_features` and `req` to be the named ones, and the daemon's dev line to equal it. Add a breaker for the dropped feature.

**2. `decisions/network.md:27`, `tools/arch-check.sh:425-431` — shape 1. C169's Pinned line claims more than the graph can hold.**
The sentence names backend, provider, root store, offered protocol and release line. The check's own comment at `:225` says "as far as the graph decides it"; the entry and the refusal text do not carry that qualifier. Changes to the ClientHello or root store that pass the check:

- **a. Builder options in Rust (by reading).** `crates/acquisition-transport/src/lib.rs:157-161` builds the client with `user_agent` and `redirect` only. Adding any of these changes what is presented with no feature change:
  - `.min_tls_version(…)` / `.max_tls_version(…)` (the supported-versions extension);
  - `.tls_sni(false)`;
  - `.use_preconfigured_tls(…)`;
  - `.add_root_certificate(…)` / `.tls_built_in_root_certs(false)` (root store).

  No test, grep or checklist row holds the builder. The network row of `REVIEW-CHECKLIST.md` covers reqwest in the public API and timeouts only.
- **b. Process-default crypto provider (by reading).** reqwest 0.12.28 `src/async_impl/client.rs:763-771` uses `CryptoProvider::get_default()` first and falls back to ring only if none is installed. Any crate in the `acqd` process that calls `install_default` (a ring provider with reordered suites or groups, or a third-party provider) changes the hello. Confirmed: adding `rustls = { version = "0.23", default-features = false, features = ["ring","std","tls12"] }` to the daemon or the transport passes. Nothing calls `install_default` today.
- **c. A patched fork (confirmed).** `[patch.crates-io] rustls = { path = <copy of 0.23.46> }` passes; the resolved package has `source: null`. The reader selects by name and never reads `.source`, so a fingerprint-altering fork of rustls or reqwest at the same version line is invisible. Smallest fix: put the source in the line and require the crates.io registry.
- **d. Crates between and beneath (by reading).** Unheld, as d9d882a7's message concedes:
  - ring 0.17 (the actual suite, group and signature-algorithm implementations);
  - hyper-rustls 0.27 and tokio-rustls 0.26;
  - rustls-webpki 0.103;
  - webpki-roots 1.0, which is the root store's content;
  - hyper, for the HTTP/1.1 header shape Cloudflare also reads.

  A ring 0.18 or webpki-roots 2.0 under unchanged rustls and reqwest lines passes. "Root store" is held only as reqwest's choice of the webpki feature.
- **e. Patch releases.** They float by the entry's own words, so the check matches the sentence here. But the lock moved rustls 0.23.43 → .45 → .46 across the ledger's rows, so the stack held is a release line, not a fixed hello. Whether any 0.23.x patch changed the ring provider's hello: not verified.
- **Held correctly:**
  - a `cfg(windows)`-only `native-tls` edge refuses (confirmed);
  - ALPN is feature-gated in reqwest (`client.rs:825-840`), so "HTTP/1.1 alone" follows from the absence of `http2`.
- **Smallest fix:** word the Pinned line as what is held ("the release lines and resolved features of reqwest and rustls"), and add a checklist row for the client builder and for `install_default`. Or add a grep over the transport and daemon sources for the builder's TLS methods and `install_default`.

**3. `docs/design/network-ground-truth.md:1007-1011`, `decisions/network.md:27`, `tools/arch-check.sh:396,425` — shape 3. The sentence d9d882a7 fixed has siblings left standing.**
- N52 still says "nothing else has been presented under the continuous user-agent". The next sentence, as amended, says the C++ app's stack under the same registration is not read.
- Invariant 4 makes the user-agent continuous with the C++ app's registration, so the first sentence asserts what the second disclaims.
- The same unread inference stands in C169's title ("the one GGG has seen from this registration"), the refusal text ("the one the registration has presented") and the comment at `:425` ("the stack GGG has seen under the continuous user-agent").
- **Smallest fix:** scope each to "the Rust daemon".

**4. `docs/design/network-ground-truth.md:1004-1006` — shapes 1, 8. "`Cargo.lock` resolves it to rustls 0.23 with `ring`, `std` and `tls12`, and reqwest without `http2`" is not readable from `Cargo.lock`.**
- A lockfile records no features. `std` and `tls12` come from `cargo metadata`'s resolve.
- The lock's reqwest entry lists `quinn` (`Cargo.lock:2802`), an HTTP/3 dependency that the metadata resolve does not contain, so the lock alone would mislead on offered protocols.
- What the lock does show: rustls depends on `ring`, and there is no `h2`, `aws-lc-rs` or `native-tls` package.
- **Smallest fix:** cite `cargo metadata --all-features`' resolve, which is the reader the pin uses.

**5. `docs/design/network-ground-truth.md:1000` — the evidence tag is weaker-fitting than its neighbours.**
- Every other `OBSERVED` claim (N34–N50) reports what GGG answered. N50 is the one offline precedent, and it reads stored GGG responses.
- N52 reads manifests and infers what was sent; no ClientHello was captured. The document's own table has `CODE` ("inference from what the current client does") for that.
- "High … no request made" is honest, but the tag is not used this way elsewhere. `OBSERVED` itself is absent from the tag table (it lists `OBS`); that predates this range.

**6. `tools/arch-check-breakers.sh:204-217` — shape 2. The two cases prove one thing: a reqwest feature added on the transport line refuses.**
- Both match the same pattern string, so neither shows which line differed.
- Case 2 is named "takes the native TLS backend" and the header says "in rustls's place". In fact rustls stays resolved through the dev edge (the comment at `:205-206` admits this), and the case is green only because `native-tls` and `default-tls` are added.
- **Missing cases:**
  - a dropped feature (fails today, finding 1);
  - a rustls-line change, e.g. the `aws-lc-rs` provider;
  - a second rustls in the graph, which the commit message claims refuses but nothing stages;
  - a release-line change;
  - a root-store change (`rustls-tls-native-roots`);
  - a patched source (passes today, finding 2c);
  - a shadowed `tls_stack` reader, as the suite does for the table reader.

**7. `decisions/network.md:27` — entry rule.**
- **Followed:** form, one *Why:*, glosses on N52, invariant 4 and N2, no history markers, 736 bytes against the 800 limit.
- **Rule 3:** the Pinned line names a check that holds less than the property (finding 2).
- **Rule 7:** the quoted word is "Yes, let's fix the first four now", on a triage recommendation. The clause "a patch release is a dependency bump" is, by 4d3ab4a4's own message, the session's reading "for his re-read". That part of the entry is not yet the owner's word.

**8. N52's history sentences — supported.**
- `8bece72d` is dated 2026-08-20 and changes the reqwest line from `["json"]` to `["json", "rustls-tls"]` with no default features.
- Every later manifest line (`5a82e761`, `7cf7ac12`, `53b50fed`, `24052df0`) is that line.
- Both lockfiles (the spike's and the root's) show reqwest 0.12.28, rustls 0.23.x, ring 0.17.14, and no `h2`, `aws-lc-rs` or `native-tls`, throughout.
- The builder never carried a TLS option at `8bece72d`, `412c840e`, `d985dd72`, `e4019380` or `709f23e0`.
- The ledger's rows start 2026-08-22, after `8bece72d`.
- "The real provider's first commit" is loose: `8bece72d` lands a "previously uncommitted" provider mode, so sends from the uncommitted tree before it are unrecorded either way.

**9. Mechanics of the reader — otherwise sound.**
- jq's `sort` is codepoint order and independent of locale.
- The comparison is exact and fails closed on an absent crate or an empty answer; jq failure exits 1.
- A duplicated reqwest or rustls is an extra line and refuses.
- Dropping `__` features loses nothing: `rustls-tls` implies `__rustls-ring`, and rustls's `ring` feature shows it.

## Not checked, and why
- The gate, the breaker suite's "110 ok", and any build: execution, not verified.
- What a real ClientHello contains, and whether any rustls 0.23.x patch changed it: no capture exists and only 0.23.46 is in the local registry.
- Whether `hyper-rustls` contributes to the client config in reqwest 0.12: not read.
- Issue #204's text beyond the quote supplied.
- `tools/docs-check.sh` and `bothy check` on the entry: not run.

## Nitpicks, unlikely in practice
- Finding 2c (a patched fork) and 2b (a third-party provider installed by the daemon) need a deliberate act.
- The dev-line false refusal in finding 1.
- A second rustls brought by an unrelated crate refuses with a message about the registration's stack.
- A reqwest patch release that adds an implied public feature would refuse a Dependabot round with a "ruling, then live run" message, against the entry's "a patch release is a dependency bump".
- Findings 4 and 5.

## About the brief and this prompt
- With no brief file, the done-when's "the owner has ruled" can only be judged from a commit message quoting an assent to a list; the ruling's scope is not on record anywhere I could read.
- REVIEW-PROMPT.md says "do not build or run anything", while the task allowed `cargo metadata` and `arch-check.sh`. I used a scratch copy for the staged cases and would name that allowance in the prompt.
