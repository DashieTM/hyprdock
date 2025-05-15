{
  description = "Docking program for Hyprland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {
        self',
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self'.packages;
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rust-analyzer
          ];
        };
        packages = let
          lockFile = ./Cargo.lock;
        in rec {
          hyprdock = pkgs.callPackage ./nix/default.nix {inherit inputs lockFile;};
          default = hyprdock;
        };
      };
      flake = _: rec {
        nixosModules.home-manager = homeManagerModules.default;
        homeManagerModules = rec {
          hyprdock = import ./nix/hm.nix inputs.self;
          default = hyprdock;
        };
      };
    };
}
