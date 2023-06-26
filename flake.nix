{
  description = "A customizable & cross platform bridge between your CLI and GUI.";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , fenix
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        inherit (pkgs) lib stdenv;
        fenixPkgs = fenix.packages.${system};
        baseToolchain = fenixPkgs.complete;
        toolchain = with baseToolchain;
          with fenixPkgs;
          combine [
            rustc
            cargo
            rust-src
            rustfmt
            clippy
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
        rustPlatform = pkgs.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              cargo-tauri
              trunk
              fenixPkgs.rust-analyzer
            ]
            ++ lib.optionals stdenv.isLinux (with pkgs; [
              pkg-config
              webkitgtk
            ])
            ++ lib.optionals stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
              wasm-bindgen-cli
              AppKit
              WebKit
              pkgs.libiconv
            ]);
          nativeBuildInputs = [ toolchain ];
        };
      }
    );
}
