{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [ pkgs.rustup pkgs.python3 ];

  shellHook = ''
    # Make sure the nightly toolchain is present (no-op if already installed)
    rustup toolchain install nightly-2026-01-13-x86_64-unknown-linux-gnu --profile minimal 2>/dev/null || true

    export LD_LIBRARY_PATH="$(rustc +nightly-2026-01-13-x86_64-unknown-linux-gnu --print target-libdir):$LD_LIBRARY_PATH"
  '';
}
