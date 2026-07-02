# esp_rs

Starter Rust `no_std` dla plytki `ESP32-C3-DevKit-RUST-1 v1.2a`.

Projekt bazuje na aktualnym generatorze `esp-generate` i stosie `esp-hal`.
Domyslny program steruje dioda RGB z akcelerometru, a dane z SHTC3 wysyla do monitora szeregowego co 5 sekund.

## Co jest w repo

- `src/bin/main.rs` - aplikacja glowna: IMU steruje jasnoscia RGB, SHTC3 loguje temperature i wilgotnosc.
- `src/bin/hello.rs` - drugi program: logowanie komunikatu co sekunde.
- `src/bin/rgb_test.rs` - bezpieczny test samej RGB: bardzo slabe R/G/B/off.
- `src/bin/safe_off.rs` - awaryjny program trzymajacy GPIO2 i GPIO7 w stanie niskim.
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

Wgraj domyslna aplikacje RGB + czujniki:

```bash
cargo run
```

Wgraj przyklad `hello`:

```bash
cargo run --bin hello
```

Wgraj bardzo przygaszony test RGB bez czujnikow:

```bash
cargo run --bin rgb_test
```

Awaryjnie zgas RGB i mala czerwona LED:

```bash
cargo run --bin safe_off
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
- `esp32c3: flash sensors-rgb`
- `esp32c3: flash hello`

## Notatki sprzetowe

- Ta plytka to `ESP32-C3-DevKit-RUST-1 v1.2a 04/2022`.
- Dioda uzytkownika RUST-1 jest na GPIO7.
- Aktualna aplikacja trzyma GPIO7 w stanie niskim, wiec mala czerwona LED nie miga.
- RGB WS2812/SK6812 jest na GPIO2.
- Magistrala I2C: SDA GPIO10, SCL GPIO8.
- IMU ICM42670-P jest pod adresem `0x68`.
- Czujnik temperatury i wilgotnosci SHTC3 jest pod adresem `0x70`.
- W RUST-2 dioda zostala przeniesiona na GPIO10, wiec nie kopiuj pinoutu z RUST-2 bez sprawdzenia rewizji.
- ESP32-C3 jest RISC-V, dlatego podstawowy target Rust to `riscv32imc-unknown-none-elf`.

## Przydatne zrodla

- Rust on ESP Book: https://docs.espressif.com/projects/rust/book/
- esp-generate: https://github.com/esp-rs/esp-generate
- esp-hal: https://github.com/esp-rs/esp-hal
- oficjalny przyklad RGB dla `esp-hal-smartled`: https://github.com/esp-rs/esp-hal-community/blob/main/esp-hal-smartled/examples/hello_rgb.rs
- espflash: https://github.com/esp-rs/espflash
- ESP32-C3-DevKit-RUST board: https://github.com/esp-rs/esp-rust-board
- Dokumentacja RUST-2 z informacja o zmianie GPIO LED: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c3/esp32-c3-devkit-rust-2/user_guide.html
