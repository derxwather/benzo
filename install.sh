#!/bin/bash

echo "🚀 Установка benzo [HMU]..."

# Проверяем есть ли Rust
if ! command -v rustc &> /dev/null; then
    echo "📦 Устанавливаем Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Создаём временную папку
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Скачиваем и собираем
echo "⚡ Скачиваем исходники..."
git clone https://github.com/derxwather/benzo .

echo "🔨 Собираем..."
cargo build --release

echo "📥 Устанавливаем..."
sudo cp target/release/benzo /usr/local/bin/
sudo chmod +x /usr/local/bin/benzo

# Чистим за собой
cd - > /dev/null
rm -rf "$TMP_DIR"

echo "✅ benzo установлен! Попробуй: benzo +h"

echo "Готово! Теперь можно использовать команду 'benzo' из любой директории" 