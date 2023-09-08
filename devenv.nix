{ pkgs, ... }:

{
  packages = with pkgs; [
    openssl.dev
    pkg-config
  ];

  languages.rust.enable = true;
}
