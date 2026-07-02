# ESP32-C3-DevKit-RUST-1

- Status identyfikacji: potwierdzony.
- Wersja fizycznej plytki: `ESP32-C3-DevKit-RUST-1 v1.2a 04/2022`.
- Zdjecia: `20260701_230940.jpg` (gora), `20260702_000431_back.jpg` (dol).
- Widoczne oznaczenia: `ESP32-C3`, `ESP-RS`, `BOOT`, `RST`, `BAT+`, `BAT-`, `Charger`, `0x68`, `0x70`, `github.com/esp-rs`.

## Zrodla

- Sklep/model: https://kamami.pl/esp32/1186940-esp32-c3-devkit-rust-1-plytka-deweloperska-z-modulem-wifi-esp32-c3-5906623469901.html
- Repozytorium KiCad RUST-1: https://github.com/esp-rs/esp-rust-board
- Dokumentacja RUST-2 podana referencyjnie, nie jako baza wymiarow tej sztuki: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c3/esp32-c3-devkit-rust-2/index.html

## Wymiary mechaniczne RUST-1

Wartosci wyciagniete z pliku KiCad `hardware/esp-rust-board/esp-rust-board.kicad_pcb` z repozytorium `esp-rs/esp-rust-board`.

| Parametr | Wartosc |
|---|---:|
| Obrys PCB, dlugosc | 63.500 mm |
| Obrys PCB, szerokosc | 22.860 mm |
| Grubosc PCB | 1.6 mm |
| Promien zaokraglenia naroznikow PCB | 0.508 mm |
| Srednica 4 otworow montazowych | 3.048 mm |
| Rozstaw osi otworow montazowych, dlugosc | 58.420 mm |
| Rozstaw osi otworow montazowych, szerokosc | 17.780 mm |
| Offset osi otworow od krotszych krawedzi PCB | 2.540 mm |
| Offset osi otworow od dluzszych krawedzi PCB | 2.540 mm |
| Rozstaw rzedow pinow | 20.320 mm |
| Raster pinow w rzedach | 2.540 mm |
| Srednica wiercen pinow | 0.762 mm |
| Srednica padow pinow | 1.524 mm |

Uklad wspolrzednych do modelowania: przyjmij `x=0..22.860 mm` po szerokosci i `y=0..63.500 mm` po dlugosci PCB. Osie otworow montazowych sa w punktach:

- `(2.540, 2.540)`
- `(20.320, 2.540)`
- `(2.540, 60.960)`
- `(20.320, 60.960)`

## Uwagi do obudowy

- USB-C jest na jednej krotkiej krawedzi; wyciecie musi uwzglednic realny wtyk, nie tylko metal gniazda.
- Przyciski `RST` i `BOOT` sa na gornej stronie plytki; obudowa powinna miec otwory serwisowe albo drukowane popychacze.
- Nie zakrywac metalowej anteny modulu ESP32-C3 pelna scianka zbyt blisko anteny.
- Piny baterii `B+`/`B-` sa na dole plytki; decyzja o ich wystawieniu zalezy od tego, czy obudowa ma obslugiwac akumulator.
- Piny boczne moga byc calkowicie dostepne przez dlugie szczeliny albo zamkniete, jesli plytka ma pracowac bez przewodow wpinanych od boku.

