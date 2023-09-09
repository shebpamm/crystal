{ pkgs, ... }:

{
  packages = with pkgs; [
    openssl.dev
    pkg-config
    postgresql
    sqlite
    mysql80
  ];

  languages.rust.enable = true;
}
