{ pkgs, ... }:

{
  packages = with pkgs; [
    openssl.dev
    pkg-config
    postgresql
    sqlite
    mysql80
    terraform
    azure-cli
    gdb
  ];

  languages.rust.enable = true;
  dotenv.enable = true;
}
