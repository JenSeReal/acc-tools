{pkgs, ...}: let
  # Compile GTK3 + desktop schemas into a single gschemas.compiled so GLib can
  # find org.gtk.Settings.FileChooser (required by the GTK file-chooser dialog).
  compiledSchemas =
    pkgs.runCommand "gsettings-compiled" {
      nativeBuildInputs = [pkgs.glib.dev];
    } ''
      mkdir -p $out/share/glib-2.0/schemas
      cp ${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}/glib-2.0/schemas/*.gschema.xml \
         $out/share/glib-2.0/schemas/
      cp ${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}/glib-2.0/schemas/*.gschema.xml \
         $out/share/glib-2.0/schemas/
      glib-compile-schemas $out/share/glib-2.0/schemas/
    '';
in {
  packages = with pkgs; [
    yaml-language-server
    vscode-css-languageserver
    tailwindcss-language-server
    tailwindcss_4

    cargo-binstall
    cargo-tauri
    trunk
    # Tauri dependencies for Linux
    webkitgtk_4_1
    gtk3
    librsvg
    glib
    gsettings-desktop-schemas
    # GStreamer plugins required by WebKitGTK for audio/video (splash screen sound)
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
  ];

  languages = {
    javascript = {
      enable = true;
      npm.enable = true;
      pnpm.enable = true;
    };
    rust = {
      enable = true;
      toolchainFile = ./rust-toolchain.toml;
    };
  };

  enterShell = ''
    if ! command -v ui &> /dev/null; then
      echo "Installing rust-ui CLI..."
      cargo binstall ui-cli --no-confirm
    fi

    # Install npm dependencies if node_modules doesn't exist
    if [ ! -d "node_modules" ]; then
      echo "Installing npm dependencies..."
      npm install
    fi
  '';

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy = {
      enable = true;
      entry = pkgs.lib.mkForce (toString (pkgs.writeShellScript "clippy-pre-commit" ''
        export CARGO_TARGET_DIR="target/pre-commit"
        exec cargo clippy \
          --workspace \
          --exclude acc-tools-ui-tauri \
          --all-targets \
          --all-features \
          -- -D warnings
      ''));
      pass_filenames = false;
    };
  };

  scripts = {
    "acc-tools-ui:desktop".exec = "cd acc-tools-ui && cargo tauri dev";
    "acc-tools-ui:desktop".description = "Run ACC Tools desktop app in dev mode";

    "acc-tools-ui:web".exec = "trunk serve --config acc-tools-ui/Trunk.toml";
    "acc-tools-ui:web".description = "Run ACC Tools web server";

    "rust-ui:init".exec = "ui init";
    "rust-ui:init".description = "Initialize rust-ui in project";

    "rust-ui:add".exec = "ui add";
    "rust-ui:add".description = "Add rust-ui components interactively";

    "rust-ui:list".exec = "ui list";
    "rust-ui:list".description = "List available rust-ui components";

    "tailwind:build".exec = "npm run build:css";
    "tailwind:build".description = "Build Tailwind CSS";

    "tailwind:watch".exec = "npm run watch:css";
    "tailwind:watch".description = "Watch and rebuild Tailwind CSS";
  };

  tasks = {
    "build:css" = {
      exec = "npm run build:css";
      description = "Build Tailwind CSS";
    };

    "build:web" = {
      exec = "trunk build --config acc-tools-ui/Trunk.toml";
      description = "Build WASM frontend";
      after = ["build:css"];
    };

    "build:desktop" = {
      exec = "cd acc-tools-ui && cargo tauri build";
      description = "Build Tauri desktop app";
    };

    "build:all" = {
      description = "Build both web and desktop";
      after = ["build:web" "build:desktop"];
    };

    "cargo:test" = {
      exec = "cargo test";
      description = "Run all tests";
    };

    "cargo:lint" = {
      exec = "cargo clippy --all-targets --all-features";
      description = "Run clippy lints";
    };

    "cargo:fmt" = {
      exec = "cargo fmt";
      description = "Format code";
    };

    "cargo:check" = {
      description = "Run fmt and lint";
      after = ["cargo:fmt" "cargo:lint"];
    };
  };

  env.GSETTINGS_SCHEMA_DIR = "${compiledSchemas}/share/glib-2.0/schemas";
}
