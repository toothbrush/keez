let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override { extensions = [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ]; });
in
with nixpkgs;
stdenv.mkDerivation {
  name = "rust";
  buildInputs = [
    # Unsure if cmake is needed -- recommended by
    # https://nixos.wiki/wiki/Rust
    # cmake
    dbus
    nasm
    openssl
    pkg-config
    ruststable
    rustup
    zlib
  ];
}
