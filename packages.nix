{ pkgs }: 
{
  default = pkgs.callPackage ./nix/pono.nix { };
  nightly = pkgs.callPackage ./nix/pono-develop.nix { };
  pono-local = pkgs.callPackage ./nix/pono-local.nix { };
}
