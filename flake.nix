{
  description = "meta-signal-herdr-nexus — generated meta herdr-nexus Signal contract";

  inputs = {
    base.url = "github:LiGoldragon/build-base";
    ethos-zero = {
      url = "github:LiGoldragon/ethos-zero";
      inputs.base.follows = "base";
    };
  };

  # Cargo.nix is generated from Cargo.lock by the build base's crate2nix and
  # committed, so evaluation imports no derivation:
  #   nix shell github:LiGoldragon/build-base#toolchain github:LiGoldragon/build-base#crate2nix -c crate2nix generate
  outputs =
    { self, base, ethos-zero }:
    base.lib.workspaceOutputs {
      cargoNix = ./Cargo.nix;

      # Every test runs with every feature on.
      memberChecks = { check, ... }: { meta-signal-herdr-nexus = check "meta-signal-herdr-nexus" [ "default" "datom" ]; };

      extraChecks =
        { pkgs, toolchain, check, doc, ... }:
        let
          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rust = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [ ./src ./tests ];
          };
        in
        {
          # The tests under the default feature set alone.
          test-default = check "meta-signal-herdr-nexus" [ "default" ];
          fmt = pkgs.runCommand "meta-signal-herdr-nexus-fmt" { nativeBuildInputs = [ toolchain ]; } ''
            rustfmt --check --edition ${manifest.package.edition} ${rust}/src/lib.rs ${rust}/tests/*.rs
            touch $out
          '';
          doc = doc "meta-signal-herdr-nexus" [ "default" "datom" ] [ "-D" "warnings" ];
          generated = pkgs.runCommand "meta-signal-herdr-nexus-generated" { generator = ethos-zero.packages.${pkgs.stdenv.hostPlatform.system}.default; declaration = ./signal.ethos; committed = ./src/lib.rs; } ''
            temporary="$(mktemp -d)"
            "$generator/bin/ethos-zero" "Generate.{ $declaration $temporary }" | grep -q '^Generated'
            cmp "$temporary/$(basename "$declaration" .ethos).rs" "$committed"
            touch "$out"
          '';
        };
    };
}
