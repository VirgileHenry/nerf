{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    libxkbcommon
    libGL
    # WINIT_UNIX_BACKEND=wayland
    wayland
  ];
  LD_LIBRARY_PATH="${pkgs.libxkbcommon}/lib:${pkgs.libGL}/lib:${pkgs.wayland}/lib";
}