{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
      in
        with pkgs; {
          # `nix develop`
          devShell = mkShell {
            nativeBuildInputs = [ rustc cargo cargo-edit ];
          };
        }
    );
}
