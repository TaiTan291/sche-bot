{
  description = "Rust Discord Bot (serenity, poise, sqlx) development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Build Inputs
        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
          sqlx-cli # DBマイグレーション用
        ];

        buildInputs = with pkgs; [
          openssl
          sqlite
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          # 環境変数の設定
          shellHook = ''
            export DATABASE_URL="sqlite://scheduler.db"
            echo "🦀 Rust Dev Shell Loaded"
            echo "Database URL: $DATABASE_URL"
          '';

          # pkg-configがOpenSSLを見つけられるように設定
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      }
    );
}
