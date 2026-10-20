{
  description = "文章のまとめ方の skill と、それを支える lint";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAll = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAll (pkgs: rec {
        # cargo test runs in checkPhase, so the package build is also the test run.
        osekkai = pkgs.rustPlatform.buildRustPackage {
          pname = "osekkai";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          meta.mainProgram = "osekkai";
        };
        default = osekkai;
      });

      checks = forAll (pkgs: {
        osekkai = self.packages.${pkgs.stdenv.hostPlatform.system}.osekkai;
        # The skill text must keep the four step anchors in the binary's order.
        skill =
          pkgs.runCommand "osekkai-skill"
            { nativeBuildInputs = [ self.packages.${pkgs.stdenv.hostPlatform.system}.osekkai ]; }
            ''
              osekkai steps --check ${./skills/osekkai/SKILL.md}
              touch $out
            '';
        fmt =
          pkgs.runCommand "osekkai-fmt" { nativeBuildInputs = [ pkgs.rustfmt ]; }
            ''
              rustfmt --check --edition 2021 ${./.}/src/*.rs ${./.}/tests/*.rs
              touch $out
            '';
      });

      overlays.default = final: prev: {
        osekkai = self.packages.${prev.stdenv.hostPlatform.system}.osekkai;
      };

      devShells = forAll (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer
          ];
        };
      });
    };
}
