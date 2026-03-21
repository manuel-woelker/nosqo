{ config, pkgs, ... }:

{
  env = {
    HOME = config.env.DEVENV_STATE + "/home";
    XDG_CACHE_HOME = config.env.HOME + "/.cache";
    XDG_CONFIG_HOME = config.env.HOME + "/.config";
    XDG_DATA_HOME = config.env.HOME + "/.local/share";
    CARGO_HOME = config.env.DEVENV_STATE + "/cargo-home";
    CARGO_TARGET_DIR = config.env.DEVENV_STATE + "/cargo-target";
    CARGO_INSTALL_ROOT = config.env.DEVENV_STATE + "/cargo-install";
  };

  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.93.1";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-src"
      "rust-analyzer"
    ];
  };

  packages = [
    pkgs.cargo-nextest
    pkgs.git
    pkgs.openssl
    pkgs.pkg-config
  ];

  enterShell = ''
    mkdir -p \
      "$HOME" \
      "$XDG_CACHE_HOME" \
      "$XDG_CONFIG_HOME" \
      "$XDG_DATA_HOME" \
      "$CARGO_HOME" \
      "$CARGO_TARGET_DIR" \
      "$CARGO_INSTALL_ROOT"

    echo "devenv ready"
    echo "HOME=$HOME"
    rustc --version
    cargo --version
  '';
}
