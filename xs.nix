{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rocmPackages.llvm.clang
    xorg.libxcb
    xorg.xcbutil
    xcb-util-cursor
    llvmPackages.libclang
  ];
  shellHook = with pkgs; ''
    export LD_LIBRARY_PATH=${llvmPackages.libclang.lib}/lib:$LD_LIBRARY_PATH
    export C_INCLUDE_PATH=${rocmPackages.llvm.clang}/lib/clang/17.0.0/include:${glibc.dev}/include:${xcb-util-cursor.dev}/include:${xorg.libxcb.dev}/include:$C_INCLUDE_PATH
  '';
}
