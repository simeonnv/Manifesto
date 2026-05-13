{
  description = "Simeon's Manifesto";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    import-tree.url = "github:vic/import-tree";

    naersk.url = "github:nix-community/naersk";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        (inputs.import-tree ./nixModules)
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      # perSystem = {system, ...}: let
      #   pkgs = import inputs.nixpkgs {
      #     inherit system;
      #     overlays = [(import inputs.rust-overlay)];
      #   };

      #   rustToolchain = pkgs.rust-bin.stable.latest.default.override {
      #     extensions = ["rust-src" "rust-analyzer"];
      #   };

      #   naersk' = pkgs.callPackage inputs.naersk {
      #     cargo = rustToolchain;
      #     rustc = rustToolchain;
      #   };
      # in {
      #   devShells.default = pkgs.lib.mkForce (pkgs.mkShell {
      #     buildInputs = [
      #       rustToolchain
      #     ];
      #   });

      #   packages.default = naersk'.buildPackage {
      #     src = ./.;
      #   };
      # };
    };
}
