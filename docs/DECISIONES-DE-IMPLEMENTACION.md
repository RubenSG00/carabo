# Decisiones de implementación

Registro de los huecos y contradicciones de la especificación que un agente ha resuelto durante una
tarea (`docs/spec/00-indice.md` §6). Cada entrada se añade al final con esta plantilla:

```markdown
## DI-<número consecutivo> · <fecha AAAA-MM-DD> · <tarea T-nnn>
- **Hueco o contradicción:** <qué faltaba o qué chocaba, citando documento y sección>
- **Decisión tomada:** <qué has implementado>
- **Por qué:** <principio PD-nn o regla en la que te apoyas>
- **Archivos afectados:** <rutas>
```

---

## DI-1 · 2026-11-16 · T-001
- **Hueco o contradicción:** 07 §2.3 pide las últimas versiones estables de `eslint` y de `eslint-plugin-jsx-a11y`. La última de `eslint` es la 10.15.0, pero la última de `eslint-plugin-jsx-a11y` (6.10.2) solo admite `eslint` hasta la línea 9 (`peerDependencies`: `^3 || … || ^9`), así que las dos últimas versiones no se pueden instalar juntas.
- **Decisión tomada:** se instala `eslint` 9.39.5, la última estable de la línea 9, que es compatible con todos los complementos de 07 §2.3 (`typescript-eslint`, `eslint-plugin-react-hooks` y `eslint-plugin-jsx-a11y`). Según 07 §2.1, la línea 9 queda fijada y no se cambia.
- **Por qué:** es lo más simple que cumple 07 §5.4 sin forzar dependencias incompatibles ni añadir configuración para saltarse la comprobación de `npm` (00 §6, paso 2).
- **Archivos afectados:** `package.json`, `package-lock.json`.

## DI-2 · 2026-11-16 · T-001
- **Hueco o contradicción:** 07 §11.1 pide poner en `LICENSE` el nombre de `git config user.name`, pero la sesión del agente tiene su propio `user.name`, distinto del autor del proyecto.
- **Decisión tomada:** `LICENSE` usa el nombre del autor del primer commit del repositorio (`RubenSG00`), que es el `user.name` de la máquina del autor (09 §1.1).
- **Por qué:** la licencia pertenece al autor del producto (DEC-24, DEC-25), no al agente.
- **Archivos afectados:** `LICENSE`.

## DI-3 · 2026-11-16 · T-002
- **Hueco o contradicción:** 07 §2.2 pide anotar la versión estable de Rust instalada con `rustup update stable` y fijarla en `rust-toolchain.toml`.
- **Decisión tomada:** `rust-toolchain.toml` fija `channel = "1.100.0"`, la versión que da `rustc --version` tras `rustup update stable` el 2026-11-16 (cumple «1.99 o posterior»).
- **Por qué:** 07 §2.2 y §5.5; T-002, criterio 2.
- **Archivos afectados:** `rust-toolchain.toml`.

## DI-4 · 2026-11-16 · T-002
- **Hueco o contradicción:** 07 §5.5 no dice qué versión llevan los paquetes `carabo` y `carabo-engine` en sus `Cargo.toml`, y 07 §5.1 dice que el campo `version` de `package.json` es la única fuente de la versión del programa.
- **Decisión tomada:** los dos `Cargo.toml` no declaran `version` (Cargo usa `0.0.0`). La versión del programa la toma Tauri de `package.json` a través de `"version": "../package.json"` en `tauri.conf.json`.
- **Por qué:** una sola fuente de la versión (07 §5.1); es lo más simple y no añade un número que haya que mantener a mano (PD-06).
- **Archivos afectados:** `engine/Cargo.toml`, `src-tauri/Cargo.toml`.

## DI-5 · 2026-11-17 · T-003
- **Hueco o contradicción:** 07 §13.3, paso 3, dice cómo se evalúan `OR` (o `/`), `AND`, los paréntesis y `WITH`, pero no qué operador manda en una expresión que mezcla `AND` y `OR` sin paréntesis, como `MIT OR GPL-3.0 AND LGPL-2.1`.
- **Decisión tomada:** `tools/check-licenses.mjs` sigue la regla de las expresiones SPDX: `WITH` se une primero a su licencia, después `AND` y por último `OR` (y `/`, que equivale a `OR`). La expresión anterior se lee `MIT OR (GPL-3.0 AND LGPL-2.1)` y se admite.
- **Por qué:** es el significado estándar de las expresiones de licencia que declaran los paquetes de Cargo y npm; no añade ninguna regla propia (00 §6, paso 2).
- **Archivos afectados:** `tools/check-licenses.mjs`, `tools/check-licenses.test.mjs`.

## DI-6 · 2026-11-17 · T-004
- **Hueco o contradicción:** 08 §7 pide que las pruebas de Rust de `engine/src/constants.rs` y `engine/src/time.rs` lean `fixtures/constants.json` y `fixtures/formulas/time.json`, pero el motor aún no tiene ninguna dependencia para leer JSON y T-004 no incluye `engine/Cargo.toml` entre sus archivos (09 §4).
- **Decisión tomada:** se añade `serde_json = "1"` a `engine/Cargo.toml` solo como `[dev-dependencies]`. `Cargo.lock` ya tenía `serde_json` 1.0.154 (por Tauri) y solo cambia la lista de dependencias de `carabo-engine`. Cuando la instantánea (T-005) necesite `serde` y `serde_json` en el propio motor, se pasan a `[dependencies]`.
- **Por qué:** `serde_json` es la dependencia que 07 §2.4 asigna al motor para JSON; usarla solo en pruebas es lo mínimo que permite cumplir 08 §7 sin escribir un lector de JSON propio (00 §6, paso 2).
- **Archivos afectados:** `engine/Cargo.toml`, `Cargo.lock`.

## DI-7 · 2026-11-17 · T-004
- **Hueco o contradicción:** 08 §7 dice que `engine/src/constants.rs` contiene «las constantes que usa el motor», sin decir cuáles son.
- **Decisión tomada:** `constants.rs` contiene las constantes de 00 §4 que el documento 06 usa, por su nombre o por su valor, y las que necesita `time.rs`: `SAMPLE_RATE`, `PPQ`, `TICKS_PER_STEP`, `TEMPO_MIN`, `TEMPO_MAX`, `TIME_SIGNATURES`, `STEPS_PER_BAR`, `MAX_TRACKS`, `MAX_SONG_FRAMES`, `MAX_PATTERNS_PER_TRACK`, `PATTERN_BARS`, `KIT_ROWS`, `MICROFADE_FRAMES`, `COUNT_IN_BARS`, `FADER_MIN_DB`, `FADER_MAX_DB`, `ROW_GAIN_MIN_DB`, `ROW_GAIN_MAX_DB`, `EQ_LOW_HZ`, `EQ_MID_HZ`, `EQ_HIGH_HZ`, `LEVEL_GAINS`, `MAX_VOICES` y `EXPORT_TAIL_FRAMES`. Las que solo usan la interfaz o el programa nativo (por ejemplo `TELEMETRY_HZ` o `PROJECT_FILE_NAME`) no se escriben en Rust; la tarea que necesite una en Rust la añade a `constants.rs` y a su prueba, que la compara con `fixtures/constants.json`. `STEPS_PER_BAR` es una tabla de pares (nombre del compás, pasos) en el orden de `TIME_SIGNATURES`.
- **Por qué:** 08 §7 y 07 §8.1 («nada fuera de la especificación»): no se escribe código que nadie usa, y `constants.rs` sigue siendo el único sitio de Rust con esos valores (07 §7).
- **Archivos afectados:** `engine/src/constants.rs`.

## DI-8 · 2026-11-17 · T-004
- **Hueco o contradicción:** RF-113 y 05 §2.2 dicen que la posición se muestra como `minutos:segundos.décimas` a partir de `tickToFrame(tick) / 48000` segundos, pero no dicen si las décimas se redondean o se truncan.
- **Decisión tomada:** `formatPosition` muestra las décimas ya alcanzadas, sin redondear: `floor(frames · 10 / SAMPLE_RATE)` décimas. A 120 negras por minuto, el tick 19 160 (9,979 s) se muestra `0:09.9`, no `0:10.0`. Los segundos van con dos cifras y los minutos sin ceros a la izquierda (`0:09.5`, `10:00.0`).
- **Por qué:** es la misma regla que el compás y el pulso de la misma fórmula, que usan `floor`, y la de cualquier reloj: no se muestra un instante al que la reproducción aún no ha llegado. Es lo más simple (00 §6, paso 2).
- **Archivos afectados:** `src/core/time.ts`, `src/core/time.test.ts`.

## DI-9 · 2026-11-17 · T-004
- **Hueco o contradicción:** RNF-17 dice que los números se formatean según el idioma (en español, con coma decimal), mientras que RF-113 y la barra de reproducción de 03 muestran la posición como `5.3` y `0:09.5`, con punto.
- **Decisión tomada:** `formatPosition` escribe los dos textos con punto en los dos idiomas (`5.1` y `0:08.0`), exactamente como RF-113.
- **Por qué:** para lo que hace el programa manda el documento 02 (00 §5), y RF-113 es el requisito concreto de esta presentación; `compás.pulso` y `min:seg.décimas` son notaciones de posición, no números decimales.
- **Archivos afectados:** `src/core/time.ts`.

## DI-10 · 2026-11-17 · T-004
- **Hueco o contradicción:** 08 §7 describe `fixtures/formulas/time.json` como una lista de casos `{ fn, args, result }` con las entradas «0, 1, 959, 960, 100 000 y el máximo que cabe en la canción», pero no dice cómo se escriben los argumentos ni cuál es ese máximo para cada función, y `barToTick` y `barToStep` no dependen del tempo.
- **Decisión tomada:** `args` es una lista en el orden de los parámetros: `[entrada, tempoBpm]` para `tickToFrame`, `frameToTick` y `stepToFrame`; `[compás, "4/4"]` para `barToTick` y `barToStep`; `[tempoBpm]` para `maxSongTick`. El máximo es `maxSongTick(tempo)` para ticks, `MAX_SONG_FRAMES` para frames, `floor(maxSongTick(tempo) / TICKS_PER_STEP)` para pasos y, para compases, el número de compases enteros que caben a 240 negras por minuto, el tempo con el que más caben (600 en 4/4 y 800 en 3/4 y 6/8). Los resultados se calcularon con aritmética entera exacta a partir de las fórmulas de 05 §2.2, sin usar `time.ts` ni `time.rs` (08 §1, principio 5).
- **Por qué:** es la forma más simple que cubre lo que pide 08 §7 y la misma para los dos lenguajes (00 §6, paso 2).
- **Archivos afectados:** `fixtures/formulas/time.json`.

## DI-11 · 2026-11-17 · T-005
- **Hueco o contradicción:** T-005 (09 §4) no incluye `engine/Cargo.toml` ni `engine/src/constants.rs` entre sus archivos, pero la instantánea y la carga de kits necesitan `serde`, `serde_json`, `thiserror` y `hound` (07 §2.4), y la validación de 06 §5.5 necesita `EQ_GAIN_MIN_DB` y `EQ_GAIN_MAX_DB`, que no estaban en `constants.rs` (DI-7). Sin ellas, `snapshot.rs` repetiría los valores −12 y 12, cosa que prohíben 00 §4 y T-004.
- **Decisión tomada:** `engine/Cargo.toml` tiene en `[dependencies]` `hound = "3.5"`, `serde` con `derive`, `serde_json` (que deja de estar solo en `[dev-dependencies]`, como preveía DI-6) y `thiserror = "2"`, y en `[dev-dependencies]` `tempfile = "3"`. `Cargo.lock` cambia solo en esas dependencias: `hound` 3.5.1, `tempfile` 3.27.0 y `thiserror` 2.0.21. `constants.rs` añade `EQ_GAIN_MIN_DB` y `EQ_GAIN_MAX_DB`, y su prueba las compara con `fixtures/constants.json`, tal como dice DI-7.
- **Por qué:** son las dependencias que 07 §2.4 asigna al motor para estos usos, y DI-7 pide añadir así una constante que el motor necesita. Es lo mínimo con lo que la tarea se puede hacer (00 §6, paso 2).
- **Archivos afectados:** `engine/Cargo.toml`, `Cargo.lock`, `engine/src/constants.rs`.

## DI-12 · 2026-11-17 · T-005
- **Hueco o contradicción:** 06 §2 dice que solo `files/` usa `hound` y que `files/wav.rs` lee los WAV. Pero según 06 §8.4 y T-005, `kits.rs` carga los WAV de los kits, y `files/wav.rs` es un archivo de T-044 (09 §8).
- **Decisión tomada:** `engine/src/kits.rs` lee los WAV de los kits con `hound` directamente, en su función privada `read_sound`. No se crea `files/`.
- **Por qué:** para el alcance de cada tarea manda 09 (00 §5): T-005 no crea archivos de T-044. Es la opción más simple y el autor la aprobó al revisar el plan de la tarea. `kits.rs` es la única excepción a la regla de 06 §2.
- **Archivos afectados:** `engine/src/kits.rs`.

## DI-13 · 2026-11-17 · T-005
- **Hueco o contradicción:** 06 §5.5 pide rechazar con `INVALID_SNAPSHOT` valores que los tipos de Rust más naturales ni siquiera admitirían: un tempo no entero, un `timeSignature` desconocido o un `phaseFrames` negativo. Con esos tipos, la instantánea fallaría al deserializarse y no llegaría a `set_project`. Además, 06 §5.5 exige `phaseFrames ≥ 0` y `regionStartFrame ≥ 0`, pero no dice nada de `startStep`, `startFrame` ni `phaseSteps`.
- **Decisión tomada:** los tipos de `engine/src/snapshot.rs` conservan los tipos JSON de la interfaz:
  - `f64` para `tempoBpm`, los dB, `pan`, `amount` y `size`;
  - `i64` para versión, pasos, frames, niveles y canales;
  - `String` para `timeSignature` y `kitId`.

  Así, todo valor que la interfaz puede enviar se deserializa, y `EngineSnapshot::validate` comprueba cada condición de 06 §5.5. También rechaza un `startStep`, un `startFrame` o un `phaseSteps` negativos.
- **Por qué:** una sola puerta de entrada, la validación, da siempre `INVALID_SNAPSHOT`, y el motor sigue con la instantánea anterior (06 §5.5). Las posiciones de la canción empiezan en 0 (00 §3.5, 05 §10.3), y aceptar una negativa obligaría al motor a manejar un caso que la interfaz nunca produce (00 §6, paso 2).
- **Archivos afectados:** `engine/src/snapshot.rs`.

## DI-14 · 2026-11-17 · T-005
- **Hueco o contradicción:** 06 §5.5 exige que `kitId` sea «uno de los kits cargados», pero `render_offline` valida la instantánea antes de cargar los kits (06 §16.1, pasos 1 y 2).
- **Decisión tomada:** la validación comprueba que `kitId` es uno de los tres identificadores de `KIT_IDS` (`acoustic`, `electronic` y `percussion`), sin consultar los kits cargados.
- **Por qué:** los kits cargados son siempre esos tres, porque `Engine::start` falla si falta alguno (06 §8.4). Con esta regla, la misma validación sirve a `set_project` y a `render_offline` sin depender de los kits (00 §6, paso 2).
- **Archivos afectados:** `engine/src/snapshot.rs`, `engine/src/kits.rs`.

## DI-15 · 2026-11-17 · T-005
- **Hueco o contradicción:** las recetas de 05 §8.4 dejan sin fijar varios detalles de los sonidos sustitutos:
  - qué valor es «el anterior» del primer frame en el ruido agudo;
  - la fase de los senos y de los barridos;
  - la forma de la onda cuadrada;
  - cómo se combinan las ráfagas y la cola de las palmadas;
  - cómo se aplica la «subida lineal» del shaker;
  - el orden entre normalizar y fundir;
  - la forma exacta del fundido;
  - cómo se pasa a 16 bits.

  Además, el script necesita `SAMPLE_RATE`, que no puede repetirse (00 §4).
- **Decisión tomada:** `tools/gen-placeholder-kits.mjs`:
  - **Ruido:** el generador empieza con `estado = semilla`, y el valor de la semilla es «el anterior» del primer frame. El ruido agudo del frame `i` es `valor(estado_(i+1)) − valor(estado_i)`.
  - **Fase:** senos y barridos empiezan en fase 0. La fase del barrido es la integral exacta de su frecuencia: `2π · (fFin · t + (fInicio − fFin) · τf · (1 − e^(−t/τf)))`.
  - **Onda cuadrada:** vale +1 en la primera mitad de cada periodo y −1 en la segunda (`(f · i) mod 48000 < 24000`).
  - **Palmadas:** las tres ráfagas y la cola empiezan con amplitud 1 y se suman.
  - **Shaker:** la subida lineal de 10 ms multiplica a la envolvente `e^(−t/τa)`.
  - **Orden:** se aplica primero el fundido de salida, `min(1, (N − 1 − i) / 240)`, de modo que el último frame vale 0, y después se normaliza el pico a 0,8.
  - **16 bits:** cada muestra se escribe como `round(x · 32767)`, igual que 06 §16.2 hace con 24 bits.
  - **Constantes:** el script importa `SAMPLE_RATE` de `src/core/constants.ts`. Node 24 ejecuta TypeScript eliminando los tipos.
- **Por qué:** en cada punto es la lectura más directa del texto de la receta. Normalizar al final garantiza que el pico sea exactamente 0,8, y la escala a 16 bits sigue la misma regla que la exportación (00 §6, paso 2). La prueba de 08 §10 fija el resultado byte a byte.
- **Archivos afectados:** `tools/gen-placeholder-kits.mjs`, `assets/kits/*/*.wav`.

## DI-16 · 2026-11-17 · T-005
- **Hueco o contradicción:** 06 §8.4 dice que la carga de kits «devuelve error», pero no con qué código de 04 §6.3. Tampoco dice cómo se convierte a `f32` una muestra de 16 bits.
- **Decisión tomada:** `load_kits` devuelve:
  - `NOT_FOUND` si falta un kit o un archivo de sonido;
  - `IO` ante cualquier otro error de lectura;
  - `UNSUPPORTED_FORMAT` si `kit.json` no es un kit con 8 sonidos en el orden de funciones de 05 §8.1, o si un WAV no es mono a 48 000 Hz en 16 bits o en coma flotante de 32 bits, o dura más de 4 s.

  Una muestra de 16 bits `v` se lee como `v / 32768`.
- **Por qué:** los códigos de error siguen la conversión de E/S de 07 §6.2. `v / 32768` deja todas las muestras de 16 bits dentro de [−1, 1). Es lo más simple (00 §6, paso 2), y el programa nativo solo escribe ese error en el registro (07 §6.1).
- **Archivos afectados:** `engine/src/kits.rs`, `engine/src/error.rs`.
