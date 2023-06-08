{}:
let
  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
  pkgs = (import <nixpkgs> {
    overlays = [ rust-overlay ];
  });
in
pkgs.mkShell {
  buildInputs = [
    
    (pkgs.rust-bin.stable.latest.rust.override {
      extensions = ["rust-src" "clippy"];
    })

    pkgs.openssl
    pkgs.pkg-config
    #pkgs.nodejs-16_x
  ];
}
