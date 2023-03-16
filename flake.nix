{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    {
      overlays.default = (final: prev:
        let naersk' = final.callPackage naersk { };
        in { git-mob = naersk'.buildPackage { src = ./.; }; });
    } // flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ self.overlays.default ];
        };
      in rec {
        defaultPackage = pkgs.git-mob;
        devShell =
          pkgs.mkShell { nativeBuildInputs = with pkgs; [ rustc cargo ]; };
      });
}
