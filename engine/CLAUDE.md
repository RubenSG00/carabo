# Motor de audio (`carabo-engine`)

Lee `docs/spec/06-motor-de-audio.md` §1 a §7 antes de tocar esta carpeta.

- En `src/rt/` y `src/dsp/`, y en las funciones de `src/io/` que llama `cpal`, rigen las reglas
  RT-01 a RT-10 (06 §4): sin reservar ni liberar memoria, sin cerrojos, sin archivos, sin registro
  y sin pánicos.
- Las fórmulas de 06 son exactas. Las pruebas comparan muestra a muestra.
- El motor no depende de Tauri ni escribe registro: comunica eventos (06 §18.2).
- Comprobar: `cargo test -p carabo-engine` y `cargo test -p carabo-engine --release -- --ignored perf_`.
