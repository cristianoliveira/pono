{
  description = "slot - Symbolic link organizer tool";
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
        devShells.default = pkgs.mkShell {
          buildInputs = [ 
            pkgs.cargo
            cpkgs.funzzy

            ## liconv
            pkgs.libiconv
          ];
        };
    });
}
