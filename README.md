# esp_rs

Starter Rust `no_std` dla plytki `ESP32-C3-DevKit-RUST-1 v1.2a`.

Projekt bazuje na aktualnym generatorze `esp-generate` i stosie `esp-hal`.
Domyslny program miga dioda uzytkownika na GPIO7, co odpowiada wersji RUST-1 tej plytki.

## Co jest w repo

- `src/bin/main.rs` - pierwszy program: blink diody uzytkownika na GPIO7.
- `src/bin/hello.rs` - drugi program: logowanie komunikatu co sekunde.
- `.cargo/config.toml` - target `riscv32imc-unknown-none-elf` i runner przez `espflash`.
- `.vscode/` - ustawienia, rekomendowane rozszerzenia i zadania VS Code.
- `docs/plytka/` - zdjecia i opis konkretnej wersji plytki.
- `docs/obudowa/` - gotowe pliki obudowy.
- `docs/skrypty_obudowy/` - skrypty generujace obudowe.

## Srodowisko

Na tym komputerze zostalo zainstalowane:

- Rust `1.96.1`
- target `riscv32imc-unknown-none-elf`
- `espflash 4.4.0`
- `esp-generate 1.3.0`

Po otwarciu nowego terminala upewnij sie, ze Cargo jest w `PATH`:

```bash
source "$HOME/.cargo/env"
```

Na nowym komputerze uruchom:

```bash
./scripts/setup_macos.sh
```

## Pierwszy build

```bash
cargo build
```

Pelniejsza kontrola:

```bash
./scripts/check.sh
```

## Flashowanie plytki

Podlacz plytke przez USB-C i sprawdz porty:

```bash
espflash list-ports
```

Wgraj domyslny blink:

```bash
cargo run
```

Wgraj przyklad `hello`:

```bash
cargo run --bin hello
```

Runner w `.cargo/config.toml` uzywa:

```bash
espflash flash --monitor --chip esp32c3
```

Dlatego po `cargo run` program zostanie wgrany i od razu otworzy sie monitor szeregowy.

## VS Code

Zainstaluj rekomendowane rozszerzenia z `.vscode/extensions.json`.

Najwazniejsze zadania w VS Code:

- `cargo: build`
- `cargo: clippy`
- `cargo: fmt`
- `espflash: list ports`
- `esp32c3: flash blink`
- `esp32c3: flash hello`

## Notatki sprzetowe

- Ta plytka to `ESP32-C3-DevKit-RUST-1 v1.2a 04/2022`.
- Dioda uzytkownika RUST-1 jest na GPIO7.
- W RUST-2 dioda zostala przeniesiona na GPIO10, wiec nie kopiuj pinoutu z RUST-2 bez sprawdzenia rewizji.
- ESP32-C3 jest RISC-V, dlatego podstawowy target Rust to `riscv32imc-unknown-none-elf`.

## Przydatne zrodla

- Rust on ESP Book: https://docs.espressif.com/projects/rust/book/
- esp-generate: https://github.com/esp-rs/esp-generate
- esp-hal: https://github.com/esp-rs/esp-hal
- espflash: https://github.com/esp-rs/espflash
- ESP32-C3-DevKit-RUST board: https://github.com/esp-rs/esp-rust-board
- Dokumentacja RUST-2 z informacja o zmianie GPIO LED: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c3/esp32-c3-devkit-rust-2/user_guide.html
