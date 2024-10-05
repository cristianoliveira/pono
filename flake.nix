{
  description = "pono - pack and organize symlinks once";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    copkgs.url = "github:cristianoliveira/nixpkgs";
  };
  outputs = { self, nixpkgs, utils, copkgs, ... }: 
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        sourcepkgs = import ./packages.nix { inherit pkgs; };
        cpkgs = import copkgs { inherit pkgs; };
      in {
        packages = sourcepkgs;

        devShells.default = pkgs.callPackage ./shell.nix { inherit pkgs cpkgs sourcepkgs; };
    });
}
