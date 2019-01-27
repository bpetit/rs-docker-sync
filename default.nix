with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "rust-docker"; # Probably put a more meaningful name here
    buildInputs = [
    	rustup
    	binutils
    	gcc
    	pkgconfig
    	openssl
    ];
}