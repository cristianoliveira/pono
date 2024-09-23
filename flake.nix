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
        cpkgs = import copkgs { inherit pkgs; };
      in {
        packages = import ./packages.nix { inherit pkgs system; };

        devShells.default = pkgs.mkShell {
          buildInputs = [ 
            # Build deps
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt

            ## System deps
            pkgs.libiconv

            ## Dev deps
            cpkgs.funzzy

            ## Github deps
            pkgs.gh
          ];
        };

        devShells.minimal = pkgs.mkShell {
          buildInputs = [ 
            # Build deps
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
          ];
        };
    });
}
