{ rustPlatform
, pkg-config
, gtk3
, gtk-layer-shell
, lib
, lockFile
, cargo
, rustc
, ...
}:
let
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

  checkInputs = [ cargo rustc ];

  nativeBuildInputs = [
    pkg-config
    cargo
    rustc
  ];
  copyLibs = true;

  meta = with lib; {
    description = "A small program to handle external pluggable screens with hyprland and acpid";
    homepage = "https://github.com/DashieTM/hyprdock";
    changelog = "https://github.com/DashieTM/hyprdock/releases/tag/${version}";
    license = licenses.gpl3;
    maintainers = with maintainers; [ DashieTM ];
    mainProgram = "hyprdock";
  };
}
