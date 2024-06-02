{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
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
          defaultPackage = craneLib.buildPackage {
            pname = "hyprland-barinfo";
            version = "0.3.0";
            src = craneLib.cleanCargoSource (craneLib.path ./.);
          };
        }
    );
}
