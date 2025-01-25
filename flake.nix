{
  description = "A development environment with nightly Rust, trunk, just, and irust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-nightly = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "rustfmt" ];
          targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            rust-nightly
            pkgs.trunk
            pkgs.just
            pkgs.irust
          ];

          shellHook = ''
            echo "Welcome to the Rust development environment!"
            echo "Rust version: $(rustc --version)"
            echo "Trunk version: $(trunk --version)"
            echo "Just version: $(just --version)"
            echo "IRust version: $(irust --version)"
          '';
        };
      }
    );
}
