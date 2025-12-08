{ nixpkgs ? import <nixpkgs> {} }:

nixpkgs.mkShell {
    nativeBuildInputs = [
        nixpkgs.deno
    ];
}
