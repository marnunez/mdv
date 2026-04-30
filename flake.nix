{
  description = "mdv - Zathura for markdown";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        gtk = pkgs.gtk4;

        desktopItem = pkgs.makeDesktopItem {
          name = "mdv";
          desktopName = "mdv";
          genericName = "Markdown Viewer";
          comment = "Minimal keyboard-driven markdown viewer";
          exec = "mdv %f";
          terminal = false;
          startupNotify = true;
          categories = [ "Office" "Viewer" "GTK" ];
          mimeTypes = [ "text/markdown" "text/x-markdown" ];
        };

        mdv = pkgs.rustPlatform.buildRustPackage {
          pname = "mdv";
          version = "0.1.0";
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
            copyDesktopItems
            makeBinaryWrapper
          ];

          buildInputs = [ gtk ];

          desktopItems = [ desktopItem ];

          postFixup = ''
            wrapProgram $out/bin/mdv \
              --prefix XDG_DATA_DIRS : ${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/gsettings-desktop-schemas-${pkgs.gsettings-desktop-schemas.version}:${gtk}/share/gsettings-schemas/gtk4-${gtk.version} \
              --prefix XDG_DATA_DIRS : $out/share
          '';

          meta = with pkgs.lib; {
            description = "Minimal keyboard-driven markdown viewer with vim keybindings";
            homepage = "https://github.com/marnunez/mdv";
            mainProgram = "mdv";
            platforms = platforms.linux;
          };
        };
      in {
        packages.default = mdv;
        packages.mdv = mdv;

        apps.default = flake-utils.lib.mkApp {
          drv = mdv;
        };

        checks.default = mdv;

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            pkg-config
            rust-analyzer
            rustup
            gtk
          ];
        };
      });
}
