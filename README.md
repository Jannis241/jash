# jash

Ein kleiner Terminal-Emulator in Rust, den ich nebenbei gebaut hab, hauptsächlich um mal was mit [egui](https://github.com/emilk/egui) zu machen.

Man tippt einen Befehl ein, der wird über `sh -c` (bzw. `cmd /C` auf Windows) ausgeführt und die Ausgabe landet im Fenster. Kein richtiges PTY, also Sachen wie `vim` oder `htop` gehen (noch) nicht.

## Starten

```
cargo run --release
```

## Config

Momentan noch direkt im Code, in `main.rs` über `window::Config`. Da kann man z.B. Schriftgröße, Farben, Prompt, Transparenz und den Cursor (Block, Strich, Underline) einstellen.
