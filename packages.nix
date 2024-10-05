{ pkgs }: 
{
  default = pkgs.callPackage ./nix/pono.nix { };
  pono-local = pkgs.callPackage ./nix/pono-local.nix { };
}
