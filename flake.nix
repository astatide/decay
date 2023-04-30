{
  description = "A Nix-flake-based Rust + Node.js development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        (import rust-overlay)
        (self: super: rec {
          nodejs = super.nodejs-18_x;
          pnpm = super.nodePackages.pnpm;
          yarn = (super.yarn.override { inherit nodejs; });
          rustToolchain =
            let
              rust = super.rust-bin;
            in
            if builtins.pathExists ./rust-toolchain.toml then
              rust.fromRustupToolchainFile ./rust-toolchain.toml
            else if builtins.pathExists ./rust-toolchain then
              rust.fromRustupToolchainFile ./rust-toolchain
            else
              rust.stable.latest.default;
        })
      ];

      pkgs = import nixpkgs { inherit system overlays; };

      packages = with pkgs; [
        rustToolchain
        openssl
        pkg-config
        cargo-deny
        cargo-edit
        cargo-watch
        rust-analyzer
        node2nix
        nodejs
        pnpm
        yarn
        wasm-pack
    ];

    allSystems = [
      "aarch64-darwin"
      "aarch64-linux"
      "armv5tel-linux"
      "armv6l-linux"
      "armv7a-linux"
      "armv7l-linux"
      "i686-linux"
      # "mipsel-linux" # Missing `busybox`.
      "powerpc64le-linux"
      "riscv64-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];

    in
    {
      
      devShells = {

        default = pkgs.mkShell {
          packages = packages;
          shellHook = ''
            ${pkgs.rustToolchain}/bin/cargo --version
          '';
        };
        
        "x86_64-linux" = pkgs.mkShell {
          packages = packages // [pkgs.darwin.apple_sdk.frameworks.AppKit];
          shellHook = ''
            ${pkgs.rustToolchain}/bin/cargo --version
          '';
        };

      };

    });
}
