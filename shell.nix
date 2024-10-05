# Add channel for cnixpkgs
{ pkgs ? import <nixpkgs> {}, cpkgs ? import <cnixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [ 
    # Build deps
    pkgs.cargo
    pkgs.rustfmt

    ## System deps
    pkgs.libiconv

    ## Dev deps
    cpkgs.funzzy
    pkgs.gnused
  ];
}
