{pkgs, ...}: {
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
    librsvg
    glib
    gsettings-desktop-schemas
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
    # clippy.enable = true;
  };

  scripts = {
    "acc-session-reader:desktop".exec = "cd acc-session-reader && cargo tauri dev";
    "acc-session-reader:desktop".description = "Run Tauri desktop app in dev mode";

    "acc-session-reader:web".exec = "trunk serve --config acc-session-reader/Trunk.toml";
    "acc-session-reader:web".description = "Run trunk web server";

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
      exec = "trunk build --config acc-session-reader/Trunk.toml";
      description = "Build WASM frontend";
      after = ["build:css"];
    };

    "build:desktop" = {
      exec = "cd acc-session-reader && cargo tauri build";
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

  env.GSETTINGS_SCHEMA_DIR = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}/glib-2.0/schemas";
}
