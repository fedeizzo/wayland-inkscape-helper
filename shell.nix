{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cmake
    cargo
    pkg-config
    udev
    libinput
  ];
}
