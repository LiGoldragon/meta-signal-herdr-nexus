{
  description = "meta signal for the herdr Nexus";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    ethos-zero.url = "github:LiGoldragon/ethos-zero";
  };
  outputs = { nixpkgs, flake-utils, rust-build, ethos-zero, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromPkgs pkgs;
        source = rust.cleanSource { root = ./.; };
        cargoArtifacts = rust.craneLib.buildDepsOnly { src = source; };
      in {
        checks = {
          test = rust.craneLib.cargoTest {
            src = source;
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--all-features";
          };
          test-default = rust.craneLib.cargoTest {
            src = source;
            inherit cargoArtifacts;
          };
          test-datom = rust.craneLib.cargoTest {
            src = source;
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--all-targets --features datom";
          };
          generated = pkgs.runCommand "meta-signal-herdr-generated" {
            generator = ethos-zero.packages.${system}.default;
            declaration = ./signal.ethos;
            committed = ./src/lib.rs;
          } ''
            temporary="$(mktemp -d)"
            "$generator/bin/ethos-zero" "Generate.{ $declaration $temporary }" | grep -q '^Generated'
            name="$(basename "$declaration" .ethos)"
            cmp "$temporary/$name.rs" "$committed"
            touch "$out"
          '';
        };
      });
}
