{ pkgs ? import <nixpkgs> {} }:

let
  # 1. Import a newer version of nixpkgs (e.g., the unstable channel)
  unstablePkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {};


in
unstablePkgs.mkShell {
  # Tools needed for development
  buildInputs = with unstablePkgs; [
    # Rust tools
    rustc
    rust-analyzer
    cargo
  ];

  
  # Optional: Set RUST_SRC_PATH for better rust-analyzer integration
}
