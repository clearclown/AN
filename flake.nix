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
      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "an";
            version = "0.1.0";
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

            meta = with pkgs.lib; {
              description = "AN (安装) - Unified Package Manager for Linux";
              homepage = "https://github.com/clearclown/AN";
              license = licenses.mit;
              maintainers = [ ];
              platforms = platforms.linux;
            };
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
