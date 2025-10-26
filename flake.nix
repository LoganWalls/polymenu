{
  description = "A hyper customizable cross-platform tool for building ephemeral widgets";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
  };
  outputs = {
    nixpkgs,
    rust-overlay,
    crane,
    ...
  }: let
    inherit (nixpkgs) lib;
    withSystem = f:
      lib.fold lib.recursiveUpdate {}
      (map f ["x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin"]);
  in
    withSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
        inherit (pkgs) stdenv lib;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        buildDeps = with pkgs; (
          [nodePackages.pnpm nodejs]
          ++ lib.optionals stdenv.isLinux [
            pkg-config
            mesa
            webkitgtk_4_1
            libxkbcommon
            xorg.libX11
            xorg.libXcursor
            xorg.libxcb
            xorg.libXi
          ]
          ++ lib.optionals stdenv.isDarwin [
            libiconv
          ]
        );
        crate = craneLib.buildPackage {
          pname = "polymenu";
          version = "0.1.0";
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          nativeBuildInputs = buildDeps;
          buildInputs = lib.optionals stdenv.isLinux (
            with pkgs; [
              glib
              gtk3
              gtk-layer-shell
              libsoup_3
              pkg-config
              webkitgtk_4_1
            ]
          );
        };
      in {
        apps.${system}.default = let
          name = crate.pname or crate.name;
          exe = crate.passthru.exePath or "/bin/${name}";
        in {
          type = "app";
          program = "${crate}${exe}";
        };
        packages.${system}.default = crate;
        checks.${system} = {inherit crate;};
        devShells.${system}.default = pkgs.mkShell {
          packages = with pkgs;
            [
              toolchain
              svelte-language-server
            ]
            ++ buildDeps;
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
