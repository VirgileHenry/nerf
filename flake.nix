{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=release-21.11";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in with pkgs; rec {
        devShell = mkShell rec {
          buildInputs = [
            libxkbcommon
            libGL

            # WINIT_UNIX_BACKEND=wayland
            wayland
          ];
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          RUST_BACKTRACE=1;
        };
      });
}