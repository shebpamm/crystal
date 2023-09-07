{ pkgs, ... }:

{
  packages = with pkgs; [];

  languages.rust.enable = true;
}
