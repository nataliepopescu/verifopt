let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.11";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    rustup
    gcc
    zlib
    libxml2
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib ]}:$LD_LIBRARY_PATH"
    export RUSTFLAGS="-L native=${pkgs.libllvm.lib}/lib"
    export LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.zlib pkgs.libxml2 ]}:$LIBRARY_PATH"
  '';
}
