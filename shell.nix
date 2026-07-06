let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
  ];
  nativeBuildInputs = with pkgs; [
    libxcb
    dbus
    pkg-config
  ];
  dbus = pkgs.dbus;
}
