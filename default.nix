{ pkgs ? import <nixpkgs> { } }:
pkgs.stdenv.mkDerivation {
  name = "nscripts";
  version = "0.1.0";
  src = ./.;
  buildInputs = [ pkgs.rustc pkgs.cargo ];
  cargoSha256 = pkgs.lib.fakeSha256;
  buildPhase = ''
    cargo build --release
  '';
  installPhase = ''
    mkdir -p $out/bin
    cp target/release/closewindow $out/bin/
  '';
  meta = with pkgs.lib; {
    description = "My commands and scripts project";
    license = licenses.mit;
    maintainers = [ "zinc" ];
  };
}
