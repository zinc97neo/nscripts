{
  description = "This project builds some commands and scripts";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
    in
    {
      packages.x86_64-linux.default = pkgs.stdenv.mkDerivation {
        pname = "nscripts";
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
          maintainers = [ maintainers.yourName ];
        };
      };
    };
}
