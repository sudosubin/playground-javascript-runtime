{
  description = "sudosubin/playground-javascript-runtime";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs.lib) genAttrs platforms;
      forAllSystems = f: genAttrs platforms.unix (system: f (import nixpkgs { inherit system; }));

    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            libiconv
            rustfmt
          ];
        };
      });
    };
}
