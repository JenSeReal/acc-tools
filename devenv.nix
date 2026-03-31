{pkgs, ...}: {
  packages = with pkgs; [
    cargo-binstall
    cargo-tauri
    trunk
    # Tauri dependencies for Linux
    webkitgtk_4_1
    librsvg
    glib
    gsettings-desktop-schemas
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.94.0";
    targets = ["wasm32-unknown-unknown"];
  };

  # git-hooks.hooks = {
  #   rustfmt.enable = true;
  #   clippy.enable = true;
  # };

  env.GSETTINGS_SCHEMA_DIR = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}/glib-2.0/schemas";
}
