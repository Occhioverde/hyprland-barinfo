{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs { inherit system; };
      in
        {
          devShell = pkgs.mkShell {
            name = "hyprland-barinfo-dev";
            packages = [
              pkgs.rustc
              pkgs.cargo
              pkgs.rust-analyzer
            ];
          };
          defaultPackage = pkgs.rustPlatform.buildRustPackage {
            pname = "hyprland-barinfo";
            version = "0.1.0";
            src = ./.;
            cargoSha256 = "sha256-vZE0OhJaYNM9XIKxz6PD+Z0yLS9mYO/YWyqkJAEHEHQ=";
          };
        }
    );
}
