# Установка lpkg

## 1. Сборка из исходного кода

> **Внимание:** подразумевается, что вы собираете `lpkg` из исходного кода на хост-ПК на архитектуре x86_64 для ПК на архитектуре AArch64.

### 1.1. Настройка окружения

> **Внимание:** все действия выполняются от имени пользователя `lfa`.

Поскольку `lpkg` написан на языке Rust, вам нужно установить инструментарий для его работы:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Установите стабильную версию языка:

```bash
rustup default stable
```

Добавьте поддержку архитектуры `$LFA_TGT`:

```bash
rustup target add aarch64-unknown-linux-musl
```

Объявите переменную окружения `$CARGO_BUILD` с аргументами для системы сборки `cargo`:

```bash
export CARGO_BUILD="build --release --target aarch64-unknown-linux-musl"
echo "CARGO_BUILD=\"$CARGO_BUILD\"" >> ~/.bashrc

mkdir -v ~/.cargo
cat > ~/.cargo/config.toml << EOF
[target.aarch64-unknown-linux-musl]
linker = "$LFA_TGT-gcc"
ar = "$LFA_TGT-ar"
EOF
```

> Подразумевается, что у вас уже собран кросс-компилятор для нужной вам архитектуры (как в этом примере AArch64).

### 1.2. Сборка

```bash
cargo $CARGO_BUILD
```

### 1.3. Установка

Создайте необходимые системные каталоги:

```bash
mkdir -pv $LFA_SYS/var/cache/lpkg/{archives,extracted}
mkdir -pv $LFA_SYS/var/db/lpkg/
```

Скопируйте собранные файлы в ОС:

```bash
cp -v ./target/release/lpkg $LFA_SYS/usr/bin/
```

## 2. Установка предварительно собранного пакета

`lpkg` имеет готовые сборки для архитектуры AArch64. Скачайте архив `lpkg-0.1-bin-aarch64.txz` и распакуйте его в директорию `/home/lfa/src/`. Далее скопируйте файлы из распакованной директории в собираемую ОС:

```bash
cp -rv {usr,var} $LFA_SYS/
```
