{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    let
      overlay =
        (final: prev:
          let
            naersk' = final.callPackage naersk {};
          in {
              git-mob = naersk'.buildPackage {src = ./.;};
            });
    in
      flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ overlay ];
        };
      in rec {
        defaultPackage = pkgs.git-mob;
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo ];
        };
      }
    );
}
