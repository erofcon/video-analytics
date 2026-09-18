# Windows Development Environment


Эта инструкция описывает настройку Windows для разработки проекта `video-analytics`.

Используемый стек:

- Rust
- MSVC
- Windows SDK
- LLVM / Clang
- FFmpeg 8.1
- `ffmpeg-next = 8.1.0`

После настройки проект должен собираться из **обычного PowerShell**:

```powershell
cargo check
cargo build
```

Открывать `Developer PowerShell for VS 2022` вручную не требуется.

---

# Требования

ОС:

- Windows 10/11 x64

Необходимые компоненты:

1. Rust
2. Visual Studio 2022 + C++ Build Tools
3. Windows SDK
4. LLVM / Clang
5. FFmpeg 8.1 development build

---

# Установка Rust

Установить Rust через `rustup`.

Официальный сайт:

https://www.rust-lang.org/tools/install

Или через `winget`:

```powershell
winget install Rustlang.Rustup
```

После установки открыть новый PowerShell.

Проверить:

```powershell
rustc --version
cargo --version
rustup show
```

Должен использоваться MSVC toolchain:

```text
stable-x86_64-pc-windows-msvc
```

При необходимости:

```powershell
rustup default stable-x86_64-pc-windows-msvc
```

---

# Установка Visual Studio 2022

Необходимо установить Visual Studio 2022 Community или Build Tools.

Скачать:

https://visualstudio.microsoft.com/downloads/

При установке выбрать workload:

> Desktop development with C++

Убедиться, что установлены:

- MSVC v143 C++ build tools
- Windows 10/11 SDK
- C++ CMake tools for Windows

Для проекта необходим именно MSVC toolchain.

После установки проверить наличие:

```text
C:\Program Files\Microsoft Visual Studio\2022\Community\
```

Путь может отличаться, если используется другая редакция Visual Studio.

---

# Установка LLVM / Clang

`ffmpeg-next` использует `ffmpeg-sys-next`, который генерирует Rust bindings для FFmpeg через `bindgen`.

`bindgen` требует LLVM/Clang и `libclang`.

Установить LLVM:

```powershell
winget install LLVM.LLVM
```

Обычно LLVM устанавливается в:

```text
C:\Program Files\LLVM
```

Проверить:

```powershell
Test-Path "C:\Program Files\LLVM\bin\libclang.dll"
```

Ожидаемый результат:

```text
True
```

---

# Настройка LIBCLANG_PATH

`LIBCLANG_PATH` должен указывать на директорию, содержащую `libclang.dll`.

Один раз выполнить:

```powershell
[Environment]::SetEnvironmentVariable( "LIBCLANG_PATH", "C:\Program Files\LLVM\bin", "User" )
```

Закрыть PowerShell и открыть новый.

Проверить:

```powershell
$env:LIBCLANG_PATH
```

Ожидаемый результат:

```text
C:\Program Files\LLVM\bin
```

Проверить сам файл:

```powershell
Test-Path "$env:LIBCLANG_PATH\libclang.dll"
```

Ожидаемый результат:

```text
True
```

---

# Установка FFmpeg

Для проекта используется:

```text
FFmpeg 8.1
```

Используется Windows x86_64 shared development build.

Скачать необходимо архив вида:

```text
ffmpeg-n8.1-latest-win64-gpl-shared-8.1.zip
```

Источник:

https://github.com/BtbN/FFmpeg-Builds/releases

Не использовать только runtime-версию FFmpeg, в которой отсутствуют:

```text
include/
lib/
```

Для сборки Rust bindings необходимы FFmpeg headers и libraries.

---

# Размещение FFmpeg в проекте

После распаковки FFmpeg должен находиться внутри проекта:

```text
video-analytics/
└── vendor/
    └── ffmpeg/
        ├── bin/
        ├── include/
        ├── lib/
        ├── doc/
        └── presets/
```

Например, если архив был распакован в:

```text
C:\ffmpeg\ffmpeg-n8.1-latest-win64-gpl-shared-8.1
```

из корня проекта выполнить:

```powershell
New-Item -ItemType Directory -Force vendor

Copy-Item `
    "C:\ffmpeg\ffmpeg-n8.1-latest-win64-gpl-shared-8.1" `
    ".\vendor\ffmpeg" `
    -Recurse
```

Проверить:

```powershell
Test-Path ".\vendor\ffmpeg\bin\ffmpeg.exe"
```

```powershell
Test-Path ".\vendor\ffmpeg\include\libavcodec\avcodec.h"
```

```powershell
Test-Path ".\vendor\ffmpeg\lib"
```

Все команды должны вернуть:

```text
True
```

---

# Настройка Cargo

В корне проекта создать:

```text
.cargo/
└── config.toml
```

Содержимое:

```toml
[env]
FFMPEG_DIR = { value = "vendor/ffmpeg", relative = true }
```

Это позволяет Cargo автоматически устанавливать:

```text
FFMPEG_DIR
```

относительно корня проекта.

Поэтому больше не требуется каждый раз выполнять:

```powershell
$env:FFMPEG_DIR=...
```

---

# Версия ffmpeg-next

В `worker/Cargo.toml` использовать фиксированную версию:

```toml
[dependencies]
ffmpeg-next = "=8.1.0"
```

Версия FFmpeg и версия Rust crate — разные вещи:

```text
FFmpeg        8.1
ffmpeg-next   8.1.0
ffmpeg-sys-next 8.1.0
```

Проверить:

```powershell
cargo tree -p worker | findstr ffmpeg
```

Ожидается:

```text
ffmpeg-next v8.1.0
ffmpeg-sys-next v8.1.0
```

---

# MSVC в обычном PowerShell

Visual Studio устанавливает необходимые инструменты:

```text
cl.exe
link.exe
Windows SDK
INCLUDE
LIB
PATH
```

Обычно они активируются через Developer PowerShell.

В этом проекте Developer PowerShell не должен требоваться для каждой сборки.

Для автоматизации используется:

```text
scripts/
└── setup-dev.ps1
```

Создать файл:

```powershell
$ErrorActionPreference = "Stop"

$vsDevShell = "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\Launch-VsDevShell.ps1"

if (-not (Test-Path $vsDevShell)) {
    throw "Visual Studio Developer PowerShell not found: $vsDevShell"
}

& $vsDevShell -Arch amd64 -HostArch amd64 -SkipAutomaticLocation

$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"

Write-Host ""
Write-Host "Development environment initialized." -ForegroundColor Green
```

Путь к Visual Studio необходимо изменить, если используется:

- Professional
- Enterprise
- Build Tools
- другой каталог установки

---

# Автоматический запуск setup-dev.ps1

Чтобы не запускать скрипт вручную при каждом открытии PowerShell, можно добавить его в PowerShell profile.

Узнать расположение profile:

```powershell
$PROFILE
```

Открыть:

```powershell
notepad $PROFILE
```

Добавить:

```powershell
& "C:\Users\<USERNAME>\Documents\video-analytics\scripts\setup-dev.ps1"
```

Заменить `<USERNAME>` на имя пользователя Windows.

После этого новый PowerShell автоматически получает окружение Visual Studio.

> Не добавлять этот абсолютный путь в Git. PowerShell profile является локальной настройкой конкретного компьютера.

---

# Проверка MSVC

Открыть новый обычный PowerShell.

Проверить:

```powershell
cl
```

Если MSVC настроен правильно, будет показана информация о Microsoft C/C++ Compiler.

Проверить:

```powershell
where.exe cl
```

Также проверить:

```powershell
where.exe link
```

---

# Проверка LLVM

```powershell
$env:LIBCLANG_PATH
```

Должно быть:

```text
C:\Program Files\LLVM\bin
```

Проверить:

```powershell
Test-Path "$env:LIBCLANG_PATH\libclang.dll"
```

Результат:

```text
True
```

---

#  Проверка FFmpeg

Из корня проекта:

```powershell
Test-Path ".\vendor\ffmpeg\bin\ffmpeg.exe"
```

```powershell
Test-Path ".\vendor\ffmpeg\include\libavcodec\avcodec.h"
```

```powershell
Test-Path ".\vendor\ffmpeg\lib"
```

Все должны вернуть:

```text
True
```

Дополнительно:

```powershell
.\vendor\ffmpeg\bin\ffmpeg.exe -version
```

В выводе должна быть версия FFmpeg 8.1.x.

---

# Проверка Rust + FFmpeg

Из корня проекта:

```powershell
cargo check
```

Если проверка прошла:

```powershell
cargo build
```

Для worker:

```powershell
cargo build -p worker
```


# Важное замечание для нового разработчика

После клонирования репозитория необходимо установить системные зависимости:

1. Rust
2. Visual Studio 2022 C++ workload
3. Windows SDK
4. LLVM

После этого FFmpeg берётся из:

```text
vendor/ffmpeg/
```

а Cargo автоматически использует его через:

```text
.cargo/config.toml
```

После настройки окружения проект собирается обычной командой:

```powershell
cargo build
```

---

# Текущая версия toolchain

На момент создания инструкции проект использует:

```text
OS:              Windows x64
Compiler:        MSVC
Visual Studio:   2022
Rust:            stable
Target:          x86_64-pc-windows-msvc
LLVM/Clang:      установлен системно
FFmpeg:          8.1
ffmpeg-next:     8.1.0
ffmpeg-sys-next: 8.1.0
```

При изменении версии FFmpeg необходимо одновременно проверить совместимость:

```text
FFmpeg
ffmpeg-next
ffmpeg-sys-next
```

и обновить эту инструкцию.