let
  nixpkgs = fetchTarball
    "https://github.com/NixOS/nixpkgs/archive/b43c397f6c213918d6cfe6e3550abfe79b5d1c51.tar.gz";
  pkgs = import nixpkgs {
    config = { };
    overlays = [ ];
  };

in
pkgs.mkShell {
  packages = with pkgs; [ dioxus-cli clippy caddy ];
  nativeBuildInputs = with pkgs; [
    xorg.libX11
    pkg-config
    gobject-introspection
    cargo
    nodejs
  ];

  buildInputs = with pkgs; [
    at-spi2-atk
    atkmm
    cairo
    cargo
    cmake
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    openssl
    pango
    rust-analyzer
    rustc
    sqlite
    wasm-bindgen-cli
    webkitgtk_4_1
    xdotool
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
