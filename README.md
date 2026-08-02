<div align="center">
  <h1>search-t</h1>
  <p>A fast, parallel file-content search tool written in Rust.</p>

  <p>
    <a href="https://github.com/SomeFlyingThing/search-t/actions/workflows/rust.yml">CI</a>
    ·
    <a href="https://github.com/SomeFlyingThing/search-t/releases">Releases</a>
    ·
    <a href="LICENSE">Apache-2.0 License</a>
  </p>
</div>

<hr>

<h2>About</h2>

<p>
  <code>sf</code> recursively searches files below the current directory
  for a byte pattern. Work is distributed across threads, and every result
  includes its relative path, zero-based line and column, and a highlighted
  match.
</p>

<p>
  When the current directory contains a <code>.gitignore</code>, ignored paths
  are skipped automatically. Searches also work in directories without one.
</p>

<h2>Usage</h2>

<p>Run the command from the directory you want to search:</p>

<pre><code>sf "needle"</code></pre>

<p>Example output:</p>

<pre><code>src/main.rs: 12: 18:     let needle = parse();</code></pre>

<h2>Installation</h2>

<h3>Prebuilt binary</h3>

<p>
  Linux x86-64 archives and SHA-256 checksums are available on the
  <a href="https://github.com/SomeFlyingThing/search-t/releases">releases page</a>.
</p>

<pre><code>tar -xzf sf-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
install -m 755 sf ~/.local/bin/sf</code></pre>

<h3>Build from source</h3>

<p>
  Install <a href="https://rustup.rs/">rustup</a>, then run the commands below.
  The repository automatically selects Rust nightly and installs the required
  Clippy and rustfmt components.
</p>

<pre><code>git clone https://github.com/SomeFlyingThing/search-t.git
cd search-t
cargo build --release --locked
install -m 755 target/release/sf ~/.local/bin/sf</code></pre>

<h2>Development</h2>

<p>Run the same checks used by CI:</p>

<pre><code>cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo build --locked --verbose
cargo test --locked</code></pre>

<h2>Publishing a release</h2>

<ol>
  <li>Update the package version in <code>Cargo.toml</code>.</li>
  <li>Commit the version change.</li>
  <li>Push a matching tag, such as <code>v0.1.0</code>.</li>
</ol>

<p>
  GitHub Actions builds the optimized Linux binary and publishes a compressed
  archive and checksum. A release is rejected when its tag does not match the
  package version.
</p>

<h2>License</h2>

<p>
  Licensed under the <a href="LICENSE">Apache License 2.0</a>.
</p>
