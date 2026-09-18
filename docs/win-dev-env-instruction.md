# Windows Development Environment

Эта инструкция описывает настройку Windows для разработки проекта `video-analytics`.

Используемый стек:

* Rust
* MSVC
* Windows SDK
* LLVM / Clang
* FFmpeg 8.1
* `ffmpeg-next = 8.1.0`

После настройки проект должен собираться и запускаться из обычного PowerShell:

```powershell
cargo check
cargo build
cargo run -p worker
```

Открывать `Developer PowerShell for VS 2022` вручную не требуется.

---

# Требования

ОС:

* Windows 10/11 x64

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

* MSVC v143 C++ build tools
* Windows 10/11 SDK
* C++ CMake tools for Windows

Для проекта используется MSVC toolchain.

После установки проверить наличие, например:

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
[Environment]::SetEnvironmentVariable(
    "LIBCLANG_PATH",
    "C:\Program Files\LLVM\bin",
    "User"
)
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

Скачать архив вида:

```text
ffmpeg-n8.1-latest-win64-gpl-shared-8.1.zip
```

Источник:

https://github.com/BtbN/FFmpeg-Builds/releases

Необходимо использовать именно **development build**, содержащий:

```text
bin/
include/
lib/
```

`include/` и `lib/` необходимы для сборки `ffmpeg-sys-next`.

---

# Размещение FFmpeg

FFmpeg устанавливается глобально.

Рекомендуемый путь:

```text
C:\ffmpeg\
├── bin\
├── include\
├── lib\
├── doc\
└── presets\
```

Например, после распаковки:

```text
C:\ffmpeg\ffmpeg-n8.1-latest-win64-gpl-shared-8.1\
```

содержимое этой директории необходимо разместить непосредственно в:

```text
C:\ffmpeg\
```

В результате должны существовать:

```text
C:\ffmpeg\bin\ffmpeg.exe
C:\ffmpeg\include\libavcodec\avcodec.h
C:\ffmpeg\lib
```

Проверить:

```powershell
Test-Path "C:\ffmpeg\bin\ffmpeg.exe"
```

```powershell
Test-Path "C:\ffmpeg\include\libavcodec\avcodec.h"
```

```powershell
Test-Path "C:\ffmpeg\lib"
```

Все команды должны вернуть:

```text
True
```

---

# Настройка PATH для FFmpeg

Добавить:

```text
C:\ffmpeg\bin
```

в пользовательский `PATH`.

В PowerShell выполнить:

```powershell
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($currentPath -notlike "*C:\ffmpeg\bin*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;C:\ffmpeg\bin",
        "User"
    )
}
```

Если команда не сработала, можно использовать `setx`:

```powershell
setx PATH "$([Environment]::GetEnvironmentVariable('Path', 'User'));C:\ffmpeg\bin"
```

После изменения **полностью закрыть PowerShell и открыть новый**.

Проверить:

```powershell
where.exe ffmpeg
```

Ожидается:

```text
C:\ffmpeg\bin\ffmpeg.exe
```

Если в системе уже установлен FFmpeg через Chocolatey и `where.exe ffmpeg` показывает несколько путей, `C:\ffmpeg\bin` должен находиться первым.

Проверить версию:

```powershell
ffmpeg -version
```

В выводе должна быть версия FFmpeg 8.1.x.


> Если ранее FFmpeg был установлен через Chocolatey, `where.exe ffmpeg` может показать несколько файлов. `C:\ffmpeg\bin` должен находиться раньше Chocolatey в `PATH`.

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
FFMPEG_DIR = "C:/ffmpeg"
```

`FFMPEG_DIR` указывает `ffmpeg-sys-next`, где находятся:

```text
C:\ffmpeg\include
C:\ffmpeg\lib
C:\ffmpeg\bin
```

Больше не требуется:

```powershell
$env:FFMPEG_DIR=...
```

Также больше не требуется хранить FFmpeg внутри Git-репозитория.

---

# Версия ffmpeg-next

В `worker/Cargo.toml` использовать фиксированную версию:

```toml
[dependencies]
ffmpeg-next = "=8.1.0"
```

Версия FFmpeg и версия Rust crate — разные вещи:

```text
FFmpeg          8.1
ffmpeg-next     8.1.0
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

Visual Studio устанавливает:

```text
cl.exe
link.exe
Windows SDK
```

Обычно необходимые переменные окружения активируются через Developer PowerShell.

Чтобы не запускать его вручную, используется:

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

if (-not (Test-Path "C:\Program Files\LLVM\bin\libclang.dll")) {
    throw "libclang.dll not found"
}

$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"

if (-not (Test-Path "C:\ffmpeg\bin\ffmpeg.exe")) {
    throw "FFmpeg not found: C:\ffmpeg\bin\ffmpeg.exe"
}

Write-Host ""
Write-Host "Development environment initialized." -ForegroundColor Green
Write-Host ""
Write-Host "FFmpeg: C:\ffmpeg"
Write-Host "LLVM:   C:\Program Files\LLVM\bin"
Write-Host "Target: x86_64-pc-windows-msvc"
```

Путь к Visual Studio необходимо изменить, если используется:

* Professional
* Enterprise
* Build Tools
* другой каталог установки.

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

После этого новый PowerShell автоматически получает окружение Visual Studio и LLVM.

> Не добавлять абсолютный путь к `setup-dev.ps1` в Git. PowerShell profile является локальной настройкой конкретного компьютера.

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

Также:

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

# Проверка FFmpeg

Проверить расположение:

```powershell
where.exe ffmpeg
```

Ожидается:

```text
C:\ffmpeg\bin\ffmpeg.exe
```

Проверить версию:

```powershell
ffmpeg -version
```

Проверить development-файлы:

```powershell
Test-Path "C:\ffmpeg\include\libavcodec\avcodec.h"
```

```powershell
Test-Path "C:\ffmpeg\lib"
```

Все должны вернуть:

```text
True
```

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

Запуск:

```powershell
cargo run -p worker
```

---

# Структура проекта

FFmpeg больше не хранится внутри проекта:

```text
video-analytics/
│
├── .cargo/
│   └── config.toml
│
├── scripts/
│   └── setup-dev.ps1
│
├── worker/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
│
├── Cargo.toml
└── ...
```

FFmpeg находится отдельно:

```text
C:\ffmpeg\
├── bin\
├── include\
└── lib\
```

---

# Важное замечание для нового разработчика

После клонирования репозитория необходимо установить системные зависимости:

1. Rust
2. Visual Studio 2022 C++ workload
3. Windows SDK
4. LLVM / Clang
5. FFmpeg 8.1 development build

FFmpeg устанавливается в:

```text
C:\ffmpeg
```

`ffmpeg-next` получает путь к FFmpeg через:

```text
.cargo/config.toml
```

```toml
[env]
FFMPEG_DIR = "C:/ffmpeg"
```

А Windows находит FFmpeg и его DLL через:

```text
C:\ffmpeg\bin
```

в системном `PATH`.

После настройки проект собирается обычной командой:

```powershell
cargo build
```

и запускается:

```powershell
cargo run -p worker
```

---

# Текущая версия toolchain

На момент создания инструкции проект использует:

```text
OS:                Windows x64
Compiler:          MSVC
Visual Studio:     2022
Rust:              stable
Target:            x86_64-pc-windows-msvc
LLVM/Clang:        установлен системно
FFmpeg:            8.1
FFmpeg location:   C:\ffmpeg
ffmpeg-next:       8.1.0
ffmpeg-sys-next:   8.1.0
```

При изменении версии FFmpeg необходимо проверить совместимость:

```text
FFmpeg
ffmpeg-next
ffmpeg-sys-next
```

и обновить эту инструкцию.
