## benzo [HMU]
># Hardcore Modular Utils - генератор проектов для задротов
 support only linux (stable on fedora)

## 🚀 Установка

### Быстрая установка (одной командой)
```bash
curl -sSL https://raw.githubusercontent.com/derxwather/benzo/main/install.sh | bash
```

### Установка из исходников
```bash
# Ставим Rust (если еще нет)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Клонируем репу
git clone https://github.com/derxwather/benzo
cd benzo

# Собираем и ставим
cargo build --release && sudo cp target/release/benzo /usr/local/bin/
```
