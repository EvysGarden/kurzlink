{
  description = "kurzlink - a static site generator for your shortlinks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk/master";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs@{ self
    , nixpkgs
    , utils
    , naersk
    , ...
    }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };

      buildInputs = with pkgs; [
      ];

      mkRustProject = (conf: ((pkgs.callPackage naersk { }).buildPackage ({
        src = ./.;
        inherit buildInputs;
      } // conf)));
    in
    {
      packages.default = mkRustProject { };
      packages.clippy = mkRustProject { mode = "clippy"; };
      packages.check = mkRustProject { mode = "check"; };

      devShell = with pkgs; mkShell {
        buildInputs = buildInputs ++ [
          rustc
          cargo
          rust-analyzer
          rustfmt
          clippy
        ];
        LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
      };
    });
}
