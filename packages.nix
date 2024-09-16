{ pkgs, system }: 
{
  slot = pkgs.callPackage ./nix/slot.nix { };
  slotNigthly = pkgs.callPackage ./nix/slot-nightly.nix { };
}
