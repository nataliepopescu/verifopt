{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = [
      pkgs.python3
      pkgs.python311Packages.scrapy
    ];
}
