{
  description = "Vampa: a toy programming language compiled with LLVM";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      crane,
      flake-utils,
      treefmt-nix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        llvm = pkgs.llvmPackages_22.llvm;

        commonArgs = {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type: (pkgs.lib.hasSuffix ".snap" path) || (craneLib.filterCargoSources path type);
          };
          strictDeps = true;

          nativeBuildInputs = [ llvm.dev ];
          buildInputs = with pkgs; [
            libffi
            libxml2
            ncurses
            zlib
          ];

          "LLVM_SYS_221_PREFIX" = llvm.dev;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        vamc = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

        treefmt = treefmt-nix.lib.evalModule pkgs {
          projectRootFile = "flake.nix";
          programs.rustfmt.enable = true;
          programs.nixfmt.enable = true;
          programs.prettier.enable = true;
        };
      in
      {
        formatter = treefmt.config.build.wrapper;
        checks.formatting = treefmt.config.build.check self;

        checks.vamc = vamc;
        packages.default = vamc;
        apps.default = flake-utils.lib.mkApp {
          drv = vamc;
          name = "vamc";
        };

        devShells.default = craneLib.devShell {
          packages = commonArgs.buildInputs ++ [ llvm.dev ];
          "LLVM_SYS_221_PREFIX" = llvm.dev;
        };
      }
    );
}
