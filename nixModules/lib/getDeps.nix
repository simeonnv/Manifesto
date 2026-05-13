{
  inputs,
  lib,
  ...
}: {
  flake.lib.getDeps = pkgs: lib: {
    nativeBuildInputs = with pkgs; [
      stdenv.cc
      clang
      pkg-config
      cmake
      gcc
      perl
      automake
      llvm
      libclang
      dioxus-cli
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
      dioxus-cli
    ];

    devTools = with pkgs; [
      cargo
      rustc
      rustfmt
      clippy
      rust-analyzer
      libclang
      llvm
      dioxus-cli
    ];
  };
}
