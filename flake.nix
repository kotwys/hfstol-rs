{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils/main;
    rust-overlay.url = github:oxalica/rust-overlay/master;
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            [ pkgs.clang ]
            ++ (pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.mold ]);
          buildInputs = [
            rust
            pkgs.rust-analyzer-unwrapped
          ];
          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
        };
      }
    );
}
