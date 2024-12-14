# lpkg — Linux for ARM Package Manager

`lpkg` — пакетный менеджер для систем, собираемых по руководству [LFA](https://linux-for-arm.github.io). Обладает простейшим базовым функционалом для дальнейшего упрощения разработки и сопровождения решений по LFA.

## Функционал

- [ ] Установка пакетов;
- [ ] Удаление пакетов;
- [ ] Просмотр информации об установленном ПО;

## Стек технологий

- **ЯП:** Rust
- **ОС:** ARM Linux musl (LFA)
- **Архитектура:** рекомендуется ARM-v8

## Зависимости

Для сборки:

- `rustc`
- `cargo`
- `git`
- `tar`
- `xz`

Для использования:

- `xz`
- `tar`
- `ash` (из BusyBox)

## Ограничения

1. На данный момент пакетный менеджер **не выполняет** проверку на наличие конфликтов перед установкой пакета. Это, может быть, будет добавлено в будущем.
2. Мы не используем никакого более-менее нормального хранилища данных об установленном в систему ПО. Эта информация хранится в ряде текстовых файлов. Это сделано потому, что для работы с текстом не нужно тащить в программу сторонних библиотек.

## Использование

**Установка пакетов в систему:**

```
# lpkg install package<1>.txz package<2>.txz ... package<n>.txz
```

По умолчанию пакеты устанавливаются в `/`, однако вы можете изменить префикс установки (на свой страх и риск!) с помощью ключа `-p`, `--prefix`:

```
# lpkg intall package<1>.txz ... package<n>.txz --prefix=/usr/local/
```

**Удаление пакетов из системы:**

```
# lpkg remove package<1> package<2> ... package<n>
```

**Просмотр информации об установленном пакете:**

```
$ lpkg meta package
```

**Просмотр списка установленных пакетов:**

```
$ lpkg list
```

Будут выведены только имена пакетов. Для того, чтобы выводить вместе с этим их версии, архитектуры и размеры на диске, используйте ключи:

- `-v`, `--version`
- `-a`, `--arch`
- `-s`, `--size`

**Встроенная справка**

О работе с дополнительными опциями смотрите справку:

```bash
lpkg --help
```

Вы также можете просмотреть справку по использованию основных команд пакетного менеджера (`install`, `remove`, `meta`):

```bash
lpkg [install|remove|meta|list] --help
```
