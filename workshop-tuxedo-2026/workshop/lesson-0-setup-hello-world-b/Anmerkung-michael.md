# Anmerkung: Release-Profile und LTO vergleichen

Diese Notiz zeigt, wie man die Build-Zeit und die erzeugte Binary-Größe für
verschiedene Release-Profile vergleicht. Gemessen wird in diesem Projekt:

```bash
cd workshop/lesson-0-setup-hello-world-b
```

## Cargo-Profile

In der `Cargo.toml` sind zwei zusätzliche Profile definiert:

```toml
[profile.release-thin]
inherits = "release"
lto = "thin"

[profile.release-fat]
inherits = "release"
lto = "fat"
```

Beide Profile erben vom normalen `release`-Profil. Deshalb bleibt `opt-level = 3`
aktiv, auch wenn es nicht noch einmal explizit eingetragen ist.

- `release`: normales Cargo-Release-Profil ohne explizites LTO in diesem Projekt
- `release-thin`: Release-Build mit ThinLTO
- `release-fat`: Release-Build mit FatLTO

## Benchmark ausführen

Für einen vergleichbaren Cold Build wird vor jedem Profil `cargo clean`
ausgeführt. Cargo gibt die Build-Zeit am Ende selbst aus.

```bash
cargo clean
cargo build
stat -f "%z Bytes" target/debug/sprite_mover

cargo clean
cargo build --release
stat -f "%z Bytes" target/release/sprite_mover

cargo clean
cargo build --profile release-thin
stat -f "%z Bytes" target/release-thin/sprite_mover

cargo clean
cargo build --profile release-fat
stat -f "%z Bytes" target/release-fat/sprite_mover
```

Auf Linux kann statt `stat -f` meist diese Variante verwendet werden:

```bash
stat -c "%s Bytes" target/release/sprite_mover
```

## Ergebnis auf diesem Rechner

Gemessen am 2026-04-13 um 21:10 CEST auf einem `Mac Book Pro M4`.

- `rustc 1.94.0 (4a4ef493e 2026-03-02)`
- `cargo 1.94.0 (85eff7c80 2026-01-15)`

| Profil | Kommando | LTO | Build-Zeit | Binary-Größe |
| --- | --- | --- | ---: | ---: |
| dev | `cargo build` | kein LTO | 7m 17s | 136,058,224 Bytes (129.75 MiB) |
| release | `cargo build --release` | kein explizites LTO | 4m 28s | 90,674,880 Bytes (86.47 MiB) |
| release-thin | `cargo build --profile release-thin` | thin | 4m 41s | 92,266,224 Bytes (87.99 MiB) |
| release-fat | `cargo build --profile release-fat` | fat | 13m 46s | 75,108,176 Bytes (71.63 MiB) |

## Einordnung

Die Werte sind nicht allgemein gültig. Sie hängen stark vom Rechner, der
Rust-Version, dem Betriebssystem, dem aktuellen Cache-Zustand und der Projektgröße
ab.

In dieser Messung war `release-fat` deutlich langsamer, erzeugte aber die kleinste
Binary. `release-thin` war nur wenig langsamer als `release`, erzeugte hier aber
keine kleinere Binary. Das kann sich bei anderen Projekten oder Toolchains ändern.
