{
  description = "AN (安装) - Unified Package Manager for Linux";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;

        an-installer = pkgs.rustPlatform.buildRustPackage rec {
          pname = "an-installer";
          version = "0.1.1";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          # テストは実行しない（Flatpak等のシステム依存があるため）
          doCheck = false;

          meta = with pkgs.lib; {
            description = "AN (安装) - Unified Package Manager for Linux";
            homepage = "https://github.com/clearclown/AN";
            license = licenses.mit;
            maintainers = [ ];
            platforms = platforms.linux;
            mainProgram = "an";
          };
        };
      in
      {
        packages = {
          default = an-installer;
          an-installer = an-installer;
        };

        apps = {
          default = {
            type = "app";
            program = "${an-installer}/bin/an";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            git
          ];

          shellHook = ''
            echo "AN development environment"
            echo "Rust version: $(rustc --version)"
          '';
        };
      }
    );
}
