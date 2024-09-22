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
            pkgs.rustfmt

            ## System deps
            pkgs.libiconv

            ## Dev deps
            cpkgs.funzzy
          ];
        };

        devShells.cross-build = pkgs.mkShell {
          buildInputs = [ 
            # Build deps
            pkgs.rustup
            pkgs.cargo
            pkgs.rustfmt

            ## System deps
            pkgs.libiconv

            ## Cross build deps
            pkgs.docker
            pkgs.cargo-cross

            ## Dev deps
            cpkgs.funzzy
          ];

          shellHook = ''
            rustup default stable
            rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin
          '';
        };
    });
}
