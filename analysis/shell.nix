let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.11";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShell {
  packages = with pkgs; [
    python3
    rustup
    libgcc
    libllvm
    cmake
    ninja
    zlib
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="/nix/store/d7xmymiyy6qnkfycsqybj62f4bk2g013-llvm-21.1.7-dev/lib/:$LD_LIBRARY_PATH"
    export LD_LIBRARY_PATH="/nix/store/zv9shv0566mrq0lvrws448rs52gc56k1-zlib-1.3.1/lib/:$LD_LIBRARY_PATH"
    export LD_LIBRARY_PATH="/nix/store/xc0ga87wdclrx54qjaryahkkmkmqi9qz-gcc-15.2.0-lib/lib/:$LD_LIBRARY_PATH"
  '';
    #export LD_LIBRARY_PATH="/nix/store/qksd2mz9f5iasbsh398akdb58fx9kx6d-gcc-13.2.0-lib/lib/:$LD_LIBRARY_PATH"
}
