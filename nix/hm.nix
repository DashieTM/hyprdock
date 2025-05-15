self: {
  config,
  pkgs,
  lib,
  ...
}: let
  cfg = config.programs.hyprdock;
  defaultPackage = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
in {
  meta.maintainers = with lib.maintainers; [dashietm];
  options.programs.hyprdock = with lib; {
    enable = mkEnableOption "hyprdock";

    package = mkOption {
      type = with types; nullOr package;
      default = defaultPackage;
      defaultText = lib.literalExpression ''
        hyprdock.packages.''${pkgs.stdenv.hostPlatform.system}.default
      '';
      description = mdDoc ''
        Package to run
      '';
    };

    settings = lib.mkOption {
      default = null;
      example = {
        monitor_name = "eDP-1";
      };
      type = with lib.types; nullOr (attrsOf anything);
      description = ''
        See https://github.com/Xetibo/hyprdock/blob/main/example_config.toml for more options.
      '';
    };
  };
  config = lib.mkIf cfg.enable {
    home.packages = lib.optional (cfg.package != null) cfg.package;
    xdg.configFile."hyprdock/config.toml" = lib.mkIf (cfg.settings != null) {
      source =
        (pkgs.formats.toml {}).generate "config" cfg.settings;
    };
  };
}
