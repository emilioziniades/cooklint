{
  description = "Cooklang recipe linter";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      forEachSystem =
        fn:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ] (system: fn nixpkgs.legacyPackages.${system});
    in
    {
      packages = forEachSystem (pkgs: {
        cooklint = pkgs.rustPlatform.buildRustPackage {
          name = "cooklint";
          src = ./.;
          cargoHash = "sha256-UFMb4i+F87wbpAxe2MSTm53k+UfWSz1sMA4oSbF1cko=";
        };
      });

      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            just
            cargo
            rustc
            clippy
            rustfmt
          ];
        };
      });
    };
}
