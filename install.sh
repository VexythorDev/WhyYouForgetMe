#!/bin/bash

set -e

APP_DIR="$HOME/.local/share/whyyouforgetme"
BIN_DIR="$HOME/.local/bin"
DESKTOP_DIR="$HOME/.local/share/applications"

echo "🌱 Instalando Planta..."

mkdir -p "$APP_DIR/assets/sprites"
mkdir -p "$BIN_DIR"
mkdir -p "$DESKTOP_DIR"

# Copia binário e assets pro mesmo lugar
cp target/release/plant "$APP_DIR/plant"
chmod +x "$APP_DIR/plant"
cp assets/banco.md      "$APP_DIR/assets/"
cp assets/config.toml   "$APP_DIR/assets/"
cp assets/sprites/*     "$APP_DIR/assets/sprites/"

# Cria wrapper no bin
cat > "$BIN_DIR/plant" << 'EOF'
#!/bin/bash
cd "$HOME/.local/share/whyyouforgetme"
exec "$HOME/.local/share/whyyouforgetme/plant" "$@"
EOF
chmod +x "$BIN_DIR/plant"

# Cria .desktop
cat > "$DESKTOP_DIR/whyyouforgetme.desktop" << EOF
[Desktop Entry]
Name=Planta
Comment=Uma planta cínica que te xinga por não regar ela
Exec=$BIN_DIR/plant
Icon=$APP_DIR/assets/sprites/icon.png
Terminal=false
Type=Application
Categories=Game;
EOF

echo "✅ Instalado! Execute com: plant"
