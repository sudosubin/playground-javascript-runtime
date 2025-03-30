{
  description = "sudosubin/playground-javascript-runtime";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      inherit (nixpkgs.lib) genAttrs platforms;
      forAllSystems = f: genAttrs platforms.unix (system: f (import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      }));

    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.stable."1.62.1".default.override {
              extensions = [ "rust-src" ];
            })
          ];
        };
      });
    };
}
