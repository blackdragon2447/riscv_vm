{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      name = "riscv_vm";
      src = ./.;
      pkgs = import nixpkgs {
        inherit system;
      };
    in {
      devShells.default = pkgs.mkShell {
        shellHook = ''
          export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib:${pkgs.libGL}/lib
        '';
        buildInputs = with pkgs.pkgsCross.riscv64-embedded.buildPackages; [gcc clang lld llvm just autoconf cmake];
        nativeBuildInputs = with pkgs; [
          libxkbcommon
          libGL
          wayland
        ];
      };
    });
}
