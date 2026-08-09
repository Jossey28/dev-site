{
  description = "Flake for `dev-site`";

  inputs = {
    nixpkgs.url = "github:/NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:/numtide/flake-utils";
    rust-overlay.url = "github:/oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "cargo"
            "clippy"
            "rustfmt"
            "rust-docs"
          ];
          targets = [
            "x86_64-unknown-linux-gnu"
            "wasm32-unknown-unknown"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            wasm-bindgen-cli
            cargo-leptos # Or `trunk`, depending on your build setup
            binaryen # For Wasm optimization (wasm-opt)
            dart-sass # Often used with Leptos for compiling SCSS
            leptosfmt # For formatting your Leptos code
            pkg-config
            openssl
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
