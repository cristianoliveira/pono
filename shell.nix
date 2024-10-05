# Add channel for cnixpkgs
{ pkgs ? import <nixpkgs> {}, cpkgs ? import <cnixpkgs> {}, sourcepkgs ? import ./packages.nix }:
pkgs.mkShell {
  buildInputs = [ 
    sourcepkgs.pono-local

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
