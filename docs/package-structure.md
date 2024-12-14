# Строение пакетов

Каждый пакет — это `tar`-архив, сжатый с помощью XZ. Архив содержит в себе 3 файла:

1. `package.toml` — метаданные пакета;
2. `files.txt` — список устанавливаемых в систему файлов (файлы из архива `pkgfiles.tar`)
3. `pkgfiles.tar` — несжатый `tar`-архив с файлами пакета.

Метаданные пакета имеют следующий вид:

```toml
# секция описания пакета
[package]
name = "имя пакета"
version = "версия пакета"
sumamry = "краткое описание пакета <= 80 симв."
maintainer = "информация о сборщике пакета"
arch = "архитектура пакета"
description = "полное описание пакета; есть поддержка многострочности"

# секция перечисления зависимостей пакета
[deps]
required = ["список", "необходимых", "зависимостей"] #, без которых пакет работать не будет
optional = ["список", "необязательных", "зависимостей"]
```

Список файлов `files.txt` — обычный ASCII-текст, в котором каждый файл указывается на отдельной строке (т.е. путь до каждого файла разделяется символом `\n`).