# AGENTS.md

## Cel projektu

Projekt jest przygotowywany pod plytke ESP32-C3 DevKit opisana w `docs/plytka/`. Kod ma byc profesjonalny, przewidywalny w utrzymaniu i zoptymalizowany pod ograniczenia mikrokontrolera.

## Zasady pracy w repo

- Nie dodawaj ani nie zachowuj redundantnych sciezek kompatybilnosci wstecznej. Przy zmianie wewnetrznego API, konfiguracji, flagi lub kontraktu UI migruj aktualnych callerow i usuwaj stare wejscia, galezie, stale, komentarze oraz fallbacki w tej samej zmianie.
- Preferuj male, jednoznaczne moduly z jasna odpowiedzialnoscia. Unikaj globalnego stanu, chyba ze jest wymagany przez sterownik, przerwanie lub runtime.
- Wprowadzaj zaleznosci ostroznie. Dla kazdej nowej zaleznosci sprawdz koszt flash/RAM, aktywowane feature flags i czy biblioteka dziala na docelowym toolchainie ESP.
- Utrzymuj konfiguracje projektu w repo: `Cargo.toml`, `Cargo.lock`, `.cargo/config.toml`, `rust-toolchain.toml`, `sdkconfig.defaults` lub rownowazne pliki powinny byc wersjonowane, jesli sa czescia builda.
- Nie commituj sekretow, tokenow Wi-Fi, certyfikatow prywatnych ani lokalnych plikow srodowiskowych.

## Rust dla ESP32-C3

- Wybierz jeden glowny stos i trzymaj sie go konsekwentnie:
  - `esp-hal` / `no_std` dla niskiego narzutu, pelnej kontroli i prostych firmware.
  - `esp-idf-hal` / `esp-idf-svc` / `std` gdy potrzebne sa Wi-Fi, TLS, uslugi IDF lub latwiejsza integracja sieciowa.
- Dla ESP32-C3 pamietaj, ze target jest RISC-V. Nie kopiuj konfiguracji z ESP32/ESP32-S3 bez sprawdzenia targetu, feature flags i peryferiow.
- Preferuj statyczna alokacje, bufory o znanym rozmiarze i typy stackowe. Uzywaj heapu tylko tam, gdzie realnie upraszcza kod i nie tworzy ryzyka fragmentacji.
- Unikaj `unwrap()` i `expect()` poza kodem inicjalizacji, testami oraz miejscami, gdzie blad jest faktycznie niemozliwy i opisany komentarzem. W logice runtime propaguj bledy przez `Result`.
- Projektuj bledy jako konkretne enumy lub lekkie typy domenowe. Nie ukrywaj bledow sprzetowych w ogolnym `anyhow::Error` w kodzie niskopoziomowym.
- Nie blokuj petli glownej bez potrzeby. Dlugie operacje dziel na kroki, dodawaj timeouty i karm watchdog, jesli runtime tego wymaga.
- Trzymaj przerwania krotkie. W ISR ustawiaj flagi, wysylaj zdarzenia lub zapisuj minimalne dane; ciezsza prace wykonuj w tasku/petli glownej.
- Dla komunikacji miedzy taskami uzywaj jawnych kolejek, kanalow lub atomikow. Dokumentuj wlascicielstwo danych wspoldzielonych.
- Logowanie powinno byc lekkie i kontrolowane poziomem. W goracych sciezkach unikaj formatowania stringow i nadmiernych logow.
- Kazda konfiguracja pinow, zegarow, UART/I2C/SPI/PWM/ADC powinna miec nazwy odpowiadajace schematowi lub fizycznemu opisowi plytki.
- Kod inicjalizacji sprzetu trzymaj oddzielnie od logiki aplikacyjnej, aby latwiej testowac logike bez hardware.

## Jakosc i optymalizacja

- Przed uznaniem zmiany za gotowa uruchom, gdy projekt juz istnieje:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo build --release`
- Mierz rozmiar firmware po wiekszych zmianach. Jesli rozmiar rosnie, wskaz powod albo ogranicz feature flags.
- Uzywaj `defmt` albo `log` konsekwentnie, nie mieszaj systemow logowania bez powodu.
- Preferuj typy jednostek i stale nazwane zamiast magicznych liczb, szczegolnie dla czasow, czestotliwosci, pinow i rozmiarow buforow.
- Dla kodu zaleznego od czasu uzywaj monotonicznych zegarow/timerow zamiast recznego liczenia petli.
- Optymalizuj dopiero po zidentyfikowaniu kosztu. Najpierw popraw algorytm, alokacje i czestotliwosc pracy peryferiow; mikrooptymalizacje zostaw na koniec.
- Jesli potrzebne sa profile release, ustaw je jawnie w `Cargo.toml`, np. `opt-level = "s"` dla rozmiaru albo `opt-level = 3` dla krytycznych sciezek wydajnosciowych.

## VS Code

- Rekomendowane rozszerzenia:
  - `rust-lang.rust-analyzer`
  - `vadimcn.vscode-lldb`
  - `tamasfe.even-better-toml`
  - rozszerzenia Espressif tylko wtedy, gdy projekt korzysta z ESP-IDF.
- Wspoldziel w repo tylko ustawienia przydatne dla wszystkich, np. `.vscode/settings.json`, `.vscode/tasks.json`, `.vscode/extensions.json` i `.vscode/launch.json`.
- Nie zapisuj w repo lokalnych sciezek do toolchainow, portow szeregowych ani prywatnych ustawien uzytkownika, jesli nie da sie ich opisac przenosnie.
- `rust-analyzer` powinien sprawdzac ten sam target i feature flags, ktorych uzywa firmware. Nie ignoruj bledow analizatora jako "problemow IDE" bez sprawdzenia konfiguracji Cargo.
- Zadania VS Code powinny wywolywac te same komendy, ktore dzialaja w terminalu: build, flash, monitor, fmt i clippy.

## Dokumentacja

- Aktualizuj `README.md` lub dokumentacje projektu przy kazdej zmianie wymagajacej nowej komendy builda, flashowania, pinoutu lub konfiguracji sprzetowej.
- Decyzje sprzetowe zapisuj konkretnie: wersja plytki, uzyte piny, napiecia, predkosci magistral, tryb zasilania i ograniczenia.
- Zdjecia, modele obudowy i skrypty generujace obudowe trzymaj w osobnych katalogach, zgodnie z aktualna struktura `docs/`.
