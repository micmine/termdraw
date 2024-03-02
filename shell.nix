{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [
      rustup
      zig
      ninja
      gettext
      libtool
      autoconf
      automake
      gnumake
      cmake
      gcc
      doxygen
      # libs
      openssl
      openssl.dev
      pkgconfig
  ];
}
