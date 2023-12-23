let
  unstable = import (fetchTarball "https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz") { };
in
{ nixpkgs ? import <nixpkgs> {} }:
with nixpkgs; mkShell {
  buildInputs = [
    cargo
    rust-analyzer
    rustc
    rustfmt

    cmake
    fontconfig
    pkg-config
  ];

  OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
  OPENSSL_LIB_DIR = "${openssl.out}/lib";
}
