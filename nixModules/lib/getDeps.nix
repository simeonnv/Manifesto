{
  inputs,
  lib,
  ...
}: {
  flake.lib.getDeps = pkgs: lib: {
    nativeBuildInputs = with pkgs; [
      stdenv.cc
      clang
      lld
      pkg-config
      cmake
      gcc
      perl
      automake
      llvm
      libclang
      nodejs
    ];

    buildInputs = with pkgs; [
      cargo
      rustc
      rustfmt
      clippy
      rust-analyzer
      curl
      libclang
      llvm
      lld
      nodejs
    ];

    devTools = with pkgs; [
      cargo
      rustc
      rustfmt
      clippy
      rust-analyzer
      libclang
      llvm
      lld
      nodejs
    ];
  };
}
