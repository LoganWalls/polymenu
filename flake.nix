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
        toolchain = with fenixPkgs;
          combine [
            complete.rustc
            complete.cargo
            complete.rust-src
            complete.rustfmt
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              cargo-tauri
              trunk
              # esbuild
              fenixPkgs.rust-analyzer
            ]
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
