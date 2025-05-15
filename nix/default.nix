{
  cargo,
  gtk-layer-shell,
  gtk3,
  lib,
  lockFile,
  pkg-config,
  rustPlatform,
  rustc,
  ...
}: let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
  rustPlatform.buildRustPackage rec {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    src = ../.;

    buildInputs = [
      pkg-config
      gtk3
      gtk-layer-shell
    ];

    cargoLock = {
      inherit lockFile;
    };

    nativeBuildInputs = [
      pkg-config
      cargo
      rustc
    ];

    meta = with lib; {
      description = "Docking program for Hyprland";
      homepage = "https://github.com/Xetibo/hyprdock";
      changelog = "https://github.com/Xetibo/hyprdock/releases/tag/${version}";
      license = licenses.gpl3;
      maintainers = with maintainers; [dashietm];
      mainProgram = "hyprdock";
    };
  }
