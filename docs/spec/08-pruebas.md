# Carabo · Especificación · 08 — Pruebas

> **PROMPT PARA EL AGENTE — léelo antes de seguir**
>
> Eres un agente de programación que trabaja en el repositorio de **Carabo**, un DAW de escritorio para Windows 11. Este documento dice **cómo se comprueba que el programa hace lo que la especificación exige**: qué tipos de prueba hay, dónde va cada una, qué datos de prueba existen, qué debe probar cada parte y cómo se ejecutan las pruebas de aceptación manuales.
>
> - Léelo en toda tarea: cada tarea del plan (09) incluye sus pruebas, y no está terminada si no las trae.
> - Lee siempre §1 a §3 y, además, el capítulo del nivel de prueba que tu tarea toca.
> - **Tu máquina puede no tener tarjeta de sonido.** Ninguna prueba automática puede necesitarla. Lo que exige oír, grabar o medir en un equipo real está en las pruebas de aceptación manuales (§12), que ejecuta el autor del proyecto, no tú. No des por comprobado lo que no has podido ejecutar.
> - Las herramientas y sus versiones están en 07 §2; las órdenes, en 07 §9.
> - Una prueba no se borra ni se debilita para que pase. Si una prueba contradice la especificación, manda la especificación y se aplica 00 §6.

---

## 1. Principios

1. **Cada criterio de aceptación tiene quien lo compruebe.** Todo criterio (`CA`) de un requisito [1.0] está cubierto por una prueba automática o por un paso de una prueba manual (RNF-21). La tabla de §13 dice cuál, y un script lo verifica (§13.3).
2. **La prueba vive lo más abajo posible.** Lo que puede comprobarse con una función pura se comprueba ahí, no a través de la interfaz. Los niveles altos (componentes, extremo a extremo) comprueban que las piezas están bien conectadas, no la lógica de nuevo.
3. **El motor se prueba sin dispositivo.** El mismo `Processor` que suena en directo se ejecuta en las pruebas llamándolo directamente (06 §1). Lo único que no se puede probar así, la entrada y la salida reales, está en §12.
4. **Las pruebas son deterministas.** Sin esperas de tiempo real en las pruebas unitarias ni de componentes, sin números aleatorios sin semilla fija, sin depender del orden de ejecución ni de archivos que deje otra prueba.
5. **Los números esperados salen de la especificación, no del programa.** Una prueba analítica calcula su resultado esperado con las fórmulas de 05 y 06. Las pruebas de regresión con huellas (§8.3) solo protegen frente a cambios involuntarios; no demuestran que algo sea correcto.
6. **El nombre de la prueba dice qué requisito cubre** (§13.2).

## 2. Niveles de prueba

| Sigla | Nivel | Herramienta | Dónde | Qué comprueba | Orden |
|---|---|---|---|---|---|
| **U** | Unitaria de la interfaz | Vitest | `src/core/**`, `src/store/**`, `src/services/**`: `*.test.ts` junto al archivo | Lógica del proyecto, historial, instantánea, servicios (con el puente simulado) | `npm test` |
| **C** | Componente | Vitest + Testing Library, con `jsdom` | `src/ui/**/*.test.tsx` | Un componente o una pestaña: qué muestra, qué órdenes lanza, estados desactivados, teclado | `npm test` |
| **E** | Extremo a extremo | Playwright, en Chromium, contra `npm run dev` con el puente simulado | `e2e/*.spec.ts` | Flujos completos de la interfaz, accesibilidad y fluidez | `npm run test:e2e` |
| **M** | Motor | `cargo test` | `engine/src/**` (unitarias) y `engine/tests/*.rs` (integración) | Algoritmos, transporte, exportación, reglas del hilo de audio | `cargo test -p carabo-engine` |
| **N** | Programa nativo | `cargo test` | `src-tauri/src/**` y `src-tauri/tests/*.rs` | Archivos de proyecto, escritura atómica, registro, errores del puente | `cargo test -p carabo` |
| **X** | Contrato | Vitest y `cargo test`, sobre los mismos archivos de `fixtures/` | §7 | Que TypeScript y Rust coinciden en constantes, fórmulas, instantáneas y tipos del puente | Las dos anteriores |
| **S** | Estática | ESLint, `tsc`, Clippy, scripts de `tools/` y pruebas que leen archivos del repositorio | §10 | Dependencias entre módulos, textos, configuración, licencias, trazabilidad | `npm run lint`, `npm test`, `npm run licenses` |
| **PA** | Aceptación manual | Una persona, con el programa compilado y un equipo real | §12 | Usabilidad, sonido real, dispositivos, rendimiento | — |

Reglas comunes:

- Las pruebas **M** y **N** que necesitan archivos usan carpetas temporales (`tempfile`) y no dejan nada.
- Dos familias de pruebas de Rust llevan `#[ignore]` y se ejecutan aparte: las de rendimiento, cuyo nombre empieza por `perf_` (`cargo test -p carabo-engine --release -- --ignored perf_`), y las que necesitan un dispositivo de audio real, cuyo nombre empieza por `device_` (`cargo test -p carabo-engine -- --ignored device_`, solo en un equipo real). La integración continua ejecuta las primeras y nunca las segundas.
- Ninguna prueba accede a la red.

## 3. Datos de prueba: `fixtures/`

Los datos que comparten TypeScript y Rust están en `fixtures/` y se guardan en Git. Son la referencia común de las pruebas de contrato (§7).

```text
fixtures/
├── constants.json              Nombre y valor de cada constante de 00 §4
├── formulas/time.json          Casos de las fórmulas de 05 §2.2
├── bridge/                     Un ejemplo en JSON de cada tipo que cruza el puente
│   ├── telemetry.json  recording-finished.json  devices-changed.json  bridge-error.json
│   ├── device-list.json  active-devices.json  imported-audio.json  export-result.json
├── projects/                   Proyectos de ejemplo: una carpeta de proyecto cada uno
│   ├── demo/project.carabo     El ejemplo completo de 05 §5.3, copiado literalmente (sin audio)
│   ├── p01-rhythm/  p02-rhythm-34/  p03-audio/  p04-mix/  p05-68/
│   └── invalid/                Archivos de proyecto no válidos, uno por regla (§4.4)
├── snapshots/                  La instantánea de cada proyecto p01 a p05
└── golden/                     La huella de la exportación de cada instantánea (§8.3)
```

### 3.1 Proyectos de ejemplo

Cada proyecto se construye con las funciones de `src/core/` siguiendo su receta. Todos empiezan con `factory.newProject()` y usan los nombres por defecto en español.

| Proyecto | Compás y tempo | Receta |
|---|---|---|
| `p01-rhythm` | 4/4, 120 | Una pista de ritmo (kit `acoustic`). Se carga el ritmo hecho `rock-basic` en su patrón. Su bloque se acorta a los compases 1 y 2 |
| `p02-rhythm-34` | 3/4, 90 | Una pista de ritmo con el kit `electronic` y el ritmo hecho `waltz`; volumen de la fila 0 a −6 dB; fila 3 silenciada; bloque en los compases 1 a 4. Un segundo patrón con el ritmo hecho `waltz-slow` y un bloque suyo en los compases 3 a 6, que tapa en parte al primero |
| `p03-audio` | 4/4, 120 | Una pista de audio con tres clips. A: fuente `audio/take-0001.wav`, en el tick 0, región `[2400, 45600)`. B: la misma fuente, en el compás 2, región `[0, 24000)`, con «Repetir» y alargado a 60 000 frames. C: fuente `audio/import-0001.wav` (estéreo), creado el último, colocado en el tick 480 con región `[0, 12000)`, de modo que tapa una parte de A |
| `p04-mix` | 4/4, 120 | Las pistas de `p01-rhythm` y de `p03-audio` juntas, más una segunda pista de audio silenciada con un clip igual que A. Pista de ritmo: volumen −6 dB, panorama −50, ecualizador activado (graves +6, medios −3, agudos +4), reverb activada (cantidad 40, tamaño 70). Primera pista de audio: volumen +3 dB, panorama +100. Maestro: −3 dB |
| `p05-68` | 6/8, 100 | Una pista de ritmo con el kit `percussion` y el ritmo hecho `ballad-68`, bloque en los compases 1 y 2. Una pista de audio con un clip de `audio/take-0001.wav` que empieza en el tick 1 440, región `[0, 48000)` |

Archivos de audio de los proyectos (WAV de coma flotante de 32 bits a 48 000 Hz, 48 000 frames):

- `audio/take-0001.wav`: mono; `muestra[k] = 0,5 · sin(2π · 440 · k / 48000)`.
- `audio/import-0001.wav`: estéreo; ruido de amplitud 0,5 con el generador de 05 §8.4, semilla 101 en el canal izquierdo y 202 en el derecho.

Los escribe el propio generador de datos de prueba (§3.3), con código de TypeScript que escribe los bytes del WAV.

### 3.2 Instantáneas

`fixtures/snapshots/<proyecto>.json` es el resultado de `buildEngineSnapshot(doc, "{FIXTURES}/projects/<proyecto>", [])`. El texto `{FIXTURES}` es un marcador: las pruebas de Rust lo sustituyen por la ruta absoluta de la carpeta `fixtures/` antes de usar la instantánea.

### 3.3 Cómo se generan y cómo se protegen

- Las recetas están en `src/test/fixtureRecipes.ts`. La prueba `src/test/fixtures.test.ts`:
  - con la variable de entorno `UPDATE_FIXTURES=1`, escribe `fixtures/projects/p01…p05` (su `project.carabo` y su audio) y `fixtures/snapshots/`;
  - sin ella, comprueba que lo que producen las recetas es idéntico a lo guardado. Así, un cambio involuntario en `core/` que altere un proyecto o una instantánea hace fallar la prueba.
- Las huellas de `fixtures/golden/` las escribe la prueba de Rust de §8.3 con `CARABO_UPDATE_GOLDEN=1`.
- Regenerar datos de prueba es un cambio que se justifica en la solicitud de integración: qué ha cambiado y por qué es correcto.
- `fixtures/constants.json`, `fixtures/formulas/time.json`, `fixtures/bridge/*.json`, `fixtures/projects/demo/` y `fixtures/projects/invalid/` se escriben a mano a partir de la especificación.

### 3.4 Constructores para las pruebas

- TypeScript: `src/test/builders.ts` ofrece funciones para montar un proyecto en una línea (`projectWithRhythmTrack()`, `projectWithAudioClip({ startTick, lengthFrames })`…), siempre a través de `core/commands/`, nunca escribiendo el objeto a mano.
- Rust: `engine/tests/common/mod.rs` ofrece constructores de instantáneas (`snapshot().rhythm_track(…).audio_track(…)`), un generador de archivos WAV temporales y el arnés de §8.1.

## 4. Pruebas unitarias de la interfaz (U)

### 4.1 `core/`

| Módulo | Qué se prueba como mínimo |
|---|---|
| `time.ts` | Cada fórmula de 05 §2.2 con los casos de `fixtures/formulas/time.json`; ida y vuelta entre ticks, pasos y compases; presentación de la posición (RF-113) |
| `factory.ts`, `ids.ts` | Valores iniciales de 05 §3.5; identificadores únicos y crecientes; nombres por defecto en los dos idiomas |
| `lanes.ts` | `resolveLane` con: sin solapes, solape parcial, uno dentro de otro (tres segmentos), tapado total, tres niveles de `z` (RF-309) |
| `commands/*.ts` | Cada función: el caso normal, cada límite de 00 §4 (RNF-22) y cada motivo de rechazo. Las operaciones de clips, con la tabla de 05 §3.3 |
| `snapshot.ts` | `buildEngineSnapshot` con los proyectos de §3.1; clips de archivos que faltan excluidos (RF-028); fase de los segmentos tras un tapado |
| `validate.ts` | El proyecto `demo` es válido; cada archivo de `fixtures/projects/invalid/` se rechaza (§4.4); `formatVersion` mayor se distingue de «dañado» (RF-034) |
| `serialize.ts` | Guardar y leer da un proyecto igual, con igualdad profunda (RF-023); el orden de las claves es estable; dos serializaciones seguidas son idénticas |
| `kits.ts`, `presets.ts` | Los tres kits tienen 8 sonidos en el orden de 05 §8.1; los 15 ritmos hechos tienen la longitud de su compás; por compás hay 11, 2 y 2 (RF-209) |

### 4.2 Invariantes tras cada operación

`core/validate.ts` exporta `checkInvariants(doc)`, que devuelve la lista de invariantes de 05 §4 incumplidos. La prueba `src/core/commands/invariants.test.ts`:

1. Para cada semilla de 1 a 20, con un generador con semilla (`mulberry32`), aplica 200 operaciones elegidas al azar entre **todas** las funciones de `core/commands/`, con argumentos al azar, válidos y no válidos.
2. Tras cada operación comprueba: `checkInvariants` devuelve una lista vacía; el proyecto cumple el esquema; `buildEngineSnapshot` no lanza y sus segmentos están ordenados y no se solapan.
3. Al final deshace todas las operaciones y comprueba que el proyecto es igual al inicial, y las rehace y comprueba que es igual al final (RF-030 CA1).

Una función nueva de `core/commands/` se añade a esta prueba en la misma tarea que la crea.

### 4.3 `store/` y `services/`

- `projectStore`: el algoritmo de 04 §3.2 paso a paso: `commit`, gesto (un solo paso, cancelación), límite de `HISTORY_LIMIT`, `isDirty` con `savedIndex` (RF-025), `setView` sin historial (RF-031).
- `engineSync`: un cambio del documento produce una llamada a `engineSetProject` en el siguiente cuadro y no más de una cada 33 ms; los cambios de claqueta y bucle se reenvían.
- Servicios, con `mockBridge` (§5.2): cada flujo de 04 §7 y cada fila de la tabla de errores de 04 §6.5 (qué mensaje se muestra para qué código). Los temporizadores (`RECOVERY_INTERVAL_S`, duración de los avisos) se prueban con los temporizadores simulados de Vitest.

### 4.4 Proyectos no válidos

`fixtures/projects/invalid/` contiene, como mínimo, un archivo por cada caso: JSON cortado (una llave de menos, RF-021 CA2); `formatVersion` 2; falta una propiedad obligatoria; un valor fuera de rango (`tempoBpm: 300`); y un archivo por cada invariante INV-01 a INV-14, que cumple el esquema e incumple solo ese invariante. El nombre dice el caso: `inv-07.carabo`, `schema-missing-tracks.carabo`.

## 5. Pruebas de componentes (C) y puente simulado

### 5.1 Reglas

- Un componente se monta con los almacenes reales y con `mockBridge`. No se simulan módulos de `core/`, `store/` ni `services/`.
- Los elementos se buscan por su **rol y su nombre accesible** (`getByRole("button", { name: "Reproducir" })`), no por clases ni por identificadores de prueba. Si un control no se encuentra así, el defecto está en el control (RNF-16).
- Las interacciones se hacen con `@testing-library/user-event`.
- Cada pestaña y cada cuadro de diálogo tienen, como mínimo, una prueba de: contenido y orden de sus controles (03); estado desactivado de cada control y su explicación (RF-008); cada orden con el ratón y con el teclado; y sus textos en los dos idiomas.
- `jsdom` no dibuja: lo que depende de tamaños reales, del `<canvas>` o del desplazamiento se prueba en el nivel E.

### 5.2 `mockBridge`

`src/bridge/mockBridge.ts` implementa toda la interfaz `Bridge` (04 §6.1) en memoria y exporta, además, un objeto de control para las pruebas, `mockControl`. En `npm run dev` ese mismo objeto queda accesible como `window.__caraboMock`, que es como lo usan las pruebas E.

| `mockControl.…` | Para qué |
|---|---|
| `reset()` | Deja el puente como recién creado: sistema de archivos vacío, dispositivos por defecto, sin fallos programados |
| `files` | Sistema de archivos falso: `list(dir)`, `read(path)`, `write(path, text)`, `exists(path)`, `remove(path)`, `setReadOnly(path, bool)`. Los comandos `project_*`, `recovery_*`, `settings_*` y `session_*` trabajan sobre él con las mismas reglas que 04 §6.2 |
| `calls` | Lista de todas las llamadas recibidas: `{ method, args }`, en orden. `clearCalls()` la vacía |
| `failNext(method, code)` | La siguiente llamada a ese método se rechaza con ese `BridgeErrorCode` |
| `queuePick(method, result)` | Resultado que devolverá la siguiente llamada a `pickProjectFile`, `pickFolder`, `pickAudioFile` o `pickExportPath` (`null` = cancelado) |
| `setDevices({ outputs, inputs })` | Dispositivos que existen. Por defecto, una salida y una entrada, ambas predeterminadas |
| `loseOutput()`, `loseInput()` | Quita el dispositivo en uso y emite `engine://devices-changed` |
| `setCalibration(frames \| "fail")` | Resultado de `calibrationRun`. Por defecto, 1 920 |
| `setImportResult({ frames, channels, truncated })` | Resultado de la siguiente importación. Sin fijarlo: 96 000 frames, 2 canales, sin recortar. Un archivo cuya extensión no es `.wav` ni `.mp3` falla con `UNSUPPORTED_FORMAT` |
| `dropFiles(paths, x, y)` | Simula soltar archivos desde el Explorador (`onFileDrop`) |
| `emitEngineError(detail)`, `emitAudioUnavailable(files)` | Emiten esos eventos |
| `requestClose()` | Simula el cierre de la ventana (`onCloseRequested`). `closed` dice si se llegó a llamar a `closeWindow` |
| `engine` | Estado del motor falso: `state`, `mode`, `positionFrame`, `lastSnapshot`, `loop`, `metronome`, `inputOpen` |
| `useManualClock()` | Desde esta llamada, el motor falso no avanza solo: solo avanza con `tick` |
| `tick(ms)` | Avanza el reloj del motor falso esos milisegundos y emite la telemetría correspondiente |

Motor falso:

- `enginePlay` pasa a `playing` y la posición avanza 48 frames por milisegundo: sola, con un temporizador de 33 ms, o a mano con `tick` si antes se llamó a `mockControl.useManualClock()`. En modo canción respeta el bucle; en modo patrón da la vuelta al final del patrón de `lastSnapshot` y calcula `patternStep`.
- `engineRecordStart` crea en `files` una toma vacía con el nombre que toca (05 §6.2), pasa a `countIn` durante un compás al tempo de `lastSnapshot` y después a `recording`. `engineStop` emite `engine://recording-finished` con los frames transcurridos y el motivo `stopped` (o `cancelled`, con 0 frames, si seguía en la cuenta atrás).
- Los picos de la telemetría son 0 parado y 0,5 en reproducción. `peaksGet` devuelve una onda sintética determinista. `exportStart` emite el progreso 0, 50 y 100, escribe un archivo falso y devuelve `{ clippedSamples: 0, frames }`.
- Sin dispositivo de salida, `enginePlay`, `engineAudition` y `engineTestTone` fallan con `NO_OUTPUT_DEVICE`; sin entrada, `engineInputOpen` y `engineRecordStart` fallan con `NO_INPUT_DEVICE`.

El puente simulado tiene sus propias pruebas (`mockBridge.test.ts`): cada método responde con la forma de 04 §6.1.

## 6. Pruebas de extremo a extremo (E)

Playwright arranca `npm run dev` (configuración `webServer`, puerto 1420) y abre la interfaz en Chromium, donde `bridge/index.ts` elige el puente simulado (07 §6.5). El tamaño de la página es 1280 × 720, el mínimo de la ventana, salvo donde se indique. Cada prueba empieza con `mockControl.reset()` y recarga la página.

| Archivo | Qué recorre | Requisitos |
|---|---|---|
| `e2e/app-shell.spec.ts` | Pantalla de inicio, proyecto vacío, las tres barras no cambian al cambiar de pestaña, atajos de pestaña, orden de los controles de cada barra de herramientas, canal maestro visible con 16 pistas | RF-002 a RF-005, RF-100, RF-200, RF-300, RF-400 |
| `e2e/first-song.spec.ts` | El recorrido de la prueba de los cinco minutos: crear una pista de ritmo, reproducir, cambiar el tempo, crear una pista de audio, grabar, reproducir todo, cambiar golpes, mezclar y exportar | CU-01, RF-201, RF-205, RF-301, RF-311 a RF-313, RF-600 |
| `e2e/project-files.spec.ts` | Nuevo, guardar como, guardar, abrir, recientes, cambios sin guardar al cerrar, copia de recuperación y recuperación, archivos que faltan | RF-010, RF-020 a RF-028 |
| `e2e/song-editing.spec.ts` | Bloques y clips con arrastres de puntero y con el teclado: añadir, mover, alargar, recortar, repetir, partir, duplicar, eliminar; zoom y desplazamiento; selección única | RF-305 a RF-309, RF-316 a RF-321, RF-324 |
| `e2e/errors.spec.ts` | Sin salida, sin entrada, dispositivo perdido, importación no admitida, soltar un archivo en otra pestaña, error interno del motor, archivo que el motor no puede cargar | RF-007, RF-315, RF-503, RF-504 |
| `e2e/language.spec.ts` | Tras pasar a inglés, ningún texto visible coincide con un valor de `es.json` que sea distinto de su valor en `en.json` | RF-701 |
| `e2e/accessibility.spec.ts` | En cada pestaña y con cada cuadro de diálogo abierto: análisis de `@axe-core/playwright` sin infracciones de nivel A ni AA; el recorrido de `first-song` solo con el teclado; el foco siempre visible | RNF-16, RF-324 |
| `e2e/performance.spec.ts` | Con un proyecto de 16 pistas y 12 clips de 10 minutos (montado con `src/test/builders.ts` y escrito en el sistema de archivos falso), a 1920 × 1080 y en reproducción: arrastrar un clip, arrastrar un deslizador de volumen y desplazar la línea de tiempo. Se mide el tiempo entre cuadros con `requestAnimationFrame` | RNF-05 |

Criterio de `performance.spec.ts`: en cada uno de los tres gestos, de 2 segundos, la mediana del tiempo entre cuadros es menor o igual que 33 ms y no hay ningún cuadro de más de 100 ms. La respuesta a una pulsación (clic en Reproducir, en un paso y en una pestaña) se mide del evento al siguiente cuadro pintado con el cambio y debe ser menor que 100 ms.

Las pruebas E no esperan con pausas fijas: esperan a que aparezca lo que buscan.

## 7. Pruebas de contrato (X)

Comprueban que las dos mitades del programa, escritas en lenguajes distintos, dicen lo mismo. Cada contrato tiene un archivo de `fixtures/` y una prueba en cada lado.

| Contrato | Archivo | Prueba de TypeScript | Prueba de Rust |
|---|---|---|---|
| Constantes | `fixtures/constants.json`: un objeto con cada nombre de 00 §4 y su valor | `src/core/constants.test.ts`: cada constante exportada coincide, y no falta ni sobra ninguna (salvo `REPOSITORY_URL`) | `engine/src/constants.rs`: lo mismo para las constantes que usa el motor |
| Fórmulas de tiempo | `fixtures/formulas/time.json`: lista de casos `{ fn, args, result }` para `tickToFrame`, `frameToTick`, `stepToFrame`, `barToTick`, `barToStep` y `maxSongTick`, con los tres compases y los tempos 40, 60, 97, 120, 133, 200 y 240, y con entradas 0, 1, 959, 960, 100 000 y el máximo que cabe en la canción | `src/core/time.test.ts` | `engine/src/time.rs` |
| Fórmulas en todo el rango | — | — | `engine/tests/contract_time.rs`: para cada tempo de 40 a 240 y cada paso de 0 a 5 000, `step_to_frame` coincide con `round(s · 720000 / tempo)` calculado en enteros |
| Instantáneas | `fixtures/snapshots/*.json` | `src/test/fixtures.test.ts` (§3.3): las recetas producen exactamente esos archivos | `engine/tests/contract_snapshot.rs`: cada archivo se deserializa, pasa la validación de 06 §5.5 y se calcula sin error |
| Tipos del puente | `fixtures/bridge/*.json` | `src/bridge/bridge.contract.test.ts`: cada archivo se asigna a su tipo de 04 §6 (comprobación del compilador) y tiene exactamente las claves del tipo | `src-tauri/tests/contract_bridge.rs`: cada tipo de Rust, construido con los mismos valores, se serializa a un JSON igual al del archivo |
| Kits | `assets/kits/*/kit.json` | `src/core/kits.test.ts` | `engine/src/kits.rs`: los tres kits se cargan y sus grupos de corte son los de 05 §8 |

## 8. Pruebas del motor (M)

### 8.1 Cómo se prueba el `Processor`

`engine/src/lib.rs` expone, bajo `#[doc(hidden)] pub mod testing`, lo necesario para manejar el motor desde `engine/tests/` sin dispositivo: el `Processor`, el tipo `RtCommand`, la preparación de una instantánea (06 §5.3) y la función de captura del hilo de entrada (06 §14.1). Ningún otro código usa ese módulo.

`engine/tests/common/mod.rs` define el arnés:

```rust
let mut h = Harness::new(&snapshot);          // prepara la instantánea y crea un Processor en Stopped
h.send(RtCommand::Play(PlayMode::Song { start_frame: 0 }));
let out = h.run(48_000, 480);                 // calcula 48 000 frames en bloques de 480; devuelve estéreo entrelazado
h.telemetry();                                // estado, posición, picos, cortes
```

Las utilidades de comprobación del arnés: `first_nonzero_frame(&out, from)`, `rms(&out, canal, rango)`, `peak(&out, canal, rango)`, `tone_level(&out, hz, canal, rango)` (amplitud de un tono por correlación con seno y coseno) y `max_step(&out, rango)` (mayor diferencia entre dos muestras seguidas).

Para probar algoritmos con valores exactos se usa un **kit de prueba** creado en una carpeta temporal: 8 sonidos de un solo frame de valor 1,0 (o de la forma que la prueba necesite). Con él, un golpe es un impulso y su frame se lee directamente.

### 8.2 Qué se prueba

Cada fila es, como mínimo, una prueba. Donde se cita un criterio de aceptación, la prueba lleva su nombre (§13.2).

**Tiempo y secuenciador**

| Prueba | Comprobación |
|---|---|
| Frame exacto de cada golpe (RF-211 CA1, RNF-10) | Con el kit de impulsos y golpes en los pasos 0, 4, 8 y 12, a 120: impulsos en los frames 0, 24 000, 48 000 y 72 000. A 97: en `round(s · 720000 / 97)`. En ambos casos, ni un frame antes ni después |
| Bloque más largo que el patrón (RF-308 CA1) | Un segmento de 3 compases de un patrón de 2 hace sonar los pasos del patrón y después los de su primer compás |
| Fase tras un tapado | Un segmento con `phaseSteps` distinto de 0 empieza en el paso de patrón que le corresponde |
| Volumen y silencio de fila | Ganancia `LEVEL_GAINS[nivel] · 10^(gainDb/20)`; una fila silenciada no produce nada |
| Grupo de corte (RF-213 CA1) | Con sonidos largos: al dispararse el charles cerrado, el abierto cae a 0 en 240 frames; un golpe de la misma fila no corta al anterior |
| Límite de voces (RF-213) | Con sonidos de 2 segundos y golpes en las 8 filas durante 6 pasos seguidos (48 golpes), nunca hay más de `MAX_VOICES` voces sin apagarse, y la que se apaga cada vez es la más antigua |
| Escucha (RF-206 CA1) | `Audition` en `Stopped` produce sonido desde el primer frame del bloque siguiente, también con la pista silenciada; la instantánea no cambia |

**Pistas de audio**

| Prueba | Comprobación |
|---|---|
| Correspondencia de frames | Con un archivo cuya muestra `k` vale `k / 100 000`: la salida en el frame `f` es la muestra `regionStartFrame + ((phaseFrames + f − startFrame) mod regionLengthFrames)` por la ganancia de 06 §9.2 |
| Microfundidos (RF-322 CA1) | La primera y la última muestra de cada segmento valen 0; a 240 frames del borde la ganancia es 1; un segmento de 100 frames sube 50 y baja 50 |
| Repetición (RF-318 CA1) | Una región de 2 compases alargada a 5 suena dos veces entera y media más, con fundido en cada unión |
| Tapado (RF-309 CA1) | La instantánea de `p03-audio` hace sonar A, C y A en los tramos que da `resolveLane` |
| Partir (RF-319 CA1) | La salida con un clip entero y con ese clip partido en dos segmentos es idéntica, salvo los 240 frames a cada lado del corte |
| Archivo estéreo y mono | Un archivo mono va igual a los dos canales; uno estéreo conserva izquierda y derecha |
| Archivo que no se puede cargar (RF-028 CA2) | El cálculo termina sin error y esos segmentos son silencio |

**Canal y efectos**

| Prueba | Comprobación |
|---|---|
| Volumen (RF-401) | A −6,0 dB, ganancia 0,501 ± 0,001; en `FADER_MIN_DB`, silencio exacto; un cambio durante la reproducción llega al valor nuevo en 480 frames, sin ninguna diferencia entre muestras mayor que la de una rampa lineal |
| Panorama (RF-402) | +100: canal izquierdo en silencio exacto; −50: canal derecho a la mitad; 0: los dos iguales |
| Silenciar y solo (RF-403) | Las cuatro combinaciones con tres pistas; silenciar durante la reproducción no produce saltos (`max_step`) |
| Medidores (RF-404 CA1) | Un tono de amplitud 0,5 da un pico de 0,5 ± 0,01 en la telemetría; cada lectura devuelve el máximo desde la anterior |
| Maestro (RF-405 CA1) | Bajar el maestro 6 dB baja 6 dB los dos canales; una suma mayor que 1 se recorta a ±1,0 y cuenta muestras recortadas (RF-603 CA1) |
| Ecualizador (RF-406) | Graves a +12 dB: un tono de 50 Hz sale 12 ± 0,5 dB más fuerte y uno de 5 000 Hz, igual ± 0,5 dB. Lo mismo para medios (1 000 Hz) y agudos (15 000 Hz). Con los tres valores a 0, salida igual a la entrada ± 0,001. Desactivado, salida idéntica a la entrada |
| Reverb (RF-407) | Cantidad 0: salida idéntica a la entrada. Un impulso con tamaño 100 tarda más en caer 60 dB que con tamaño 0. Al desactivarla, la cola llega a 0 en 960 frames |
| Orden (RF-408 CA1) | Con el volumen en `FADER_MIN_DB` y la reverb activada, la salida de la pista es silencio exacto |

**Transporte**

| Prueba | Comprobación |
|---|---|
| Parar (RF-101 CA2) | Tras `Stop`, la salida baja a 0 en 240 frames sin ningún salto mayor que el de la propia señal, y después es silencio exacto |
| Bucle (RF-111) | Con el tramo en los compases 1 a 4: tras `loop_end` suena el compás 1 y la posición informada vuelve a `loop_start`; el último frame de audio antes del salto vale 0. Empezando en el compás 6, no hay salto |
| Mover el cursor | Tras `Seek`, la posición nueva vale como mucho 240 frames después y el audio entra con fundido |
| Modo patrón (RF-102 CA1) | Suena solo la pista elegida, aunque esté silenciada o haya otra con «Solo»; al final del patrón vuelve al paso 0; al cambiar de patrón conserva el paso módulo la longitud nueva |
| Cambio de tempo (RF-106 CA2) | En modo patrón, al adoptar una instantánea con otro tempo, el siguiente golpe llega cuando corresponde a la posición musical, sin hueco ni repetición |
| Claqueta (RF-110) | En 6/8, seis clics por compás, el primero a 1 500 Hz y los demás a 1 000 Hz; desactivada, ninguno; en un cálculo sin dispositivo, ninguno aunque se active |
| Cuenta atrás (RF-311 CA1) | En 4/4, 4 clics y después estado `Recording` exactamente a los `tickToFrame(3840)` frames; en 6/8, 6 clics. Durante la cuenta atrás no suena la canción |
| Límite (RF-112 CA2) | Reproduciendo o grabando desde `MAX_SONG_FRAMES − 1000`, el estado pasa a parado a los 1 000 frames, y en grabación queda marcado el fin por límite |
| Instantánea con el audio cambiado | Al adoptarla en reproducción, el audio baja a 0, cambia y vuelve a subir, sin saltos; las pistas de ritmo no se alteran |
| Validación (RNF-22) | Cada condición de 06 §5.5 tiene una instantánea que la incumple y se rechaza con `INVALID_SNAPSHOT` |

**Captura, relojes y calibración** (sin dispositivo: se llama a las funciones con datos fabricados)

| Prueba | Comprobación |
|---|---|
| Captura | Dadas llamadas de entrada con bloques de tamaños variados, `capture_start`, un `desfase` y un `capture_end`, los frames enviados al escritor son exactamente los de 06 §14.1: ni uno más ni uno menos, y en orden. Con `capture_abort`, se deja de enviar |
| Relación de relojes | Con marcas de tiempo fabricadas, periódicas y con retrasos al azar de hasta 5 ms, `DEntrada − DSalida` se estima con un error menor que 24 frames |
| Escritor | Los frames llegan al archivo WAV completos y en orden; una toma cancelada no deja archivo; un fallo de escritura termina con error de disco (RF-314) |
| Calibración (RF-506 CA2) | Una captura fabricada con la señal retrasada 1 234 frames y ruido de amplitud 0,01 da 1 234 ± 1. Una captura en silencio, una con solo 4 clics y una con los clics a retardos dispares dan `CALIBRATION_FAILED` |
| Conversión de frecuencia (RF-502) | El adaptador de salida, con un dispositivo de 44 100 Hz, convierte un tono de 440 Hz en uno de 440 ± 1 Hz medido en frames del dispositivo; el de entrada, lo mismo a la inversa |

**Archivos**

| Prueba | Comprobación |
|---|---|
| Importar (RF-315) | Un WAV de 16 bits mono a 44 100 Hz y un MP3 estéreo a 44 100 Hz (creado en la propia prueba con el codificador) dan un archivo a 48 000 Hz cuya duración difiere de la original en menos de 1 ms por minuto, con el tono en su frecuencia. Un archivo de texto con extensión `.wav` da `UNSUPPORTED_FORMAT` y no deja archivos. Un archivo más largo que `MAX_SONG_FRAMES` se recorta y lo indica |
| Picos | El archivo `.peaks` tiene la cabecera y el número de cubos de 05 §6.3, y cada cubo contiene el mínimo y el máximo reales |
| Exportar (RF-601) | Un bloque de 4 compases a 120 en 4/4 da 480 000 frames (CA1). Dos exportaciones dan archivos idénticos byte a byte (CA2). El WAV, leído de nuevo, coincide con la salida del `Processor` manejado con el arnés en modo canción desde 0, con un error menor que 1/8 388 608 (CA3). Sin segmentos: `NOTHING_TO_EXPORT`. Con la cancelación activada: `CANCELLED` |
| MP3 | El archivo exportado se puede decodificar con `symphonia`, dura lo mismo que el WAV ± 0,1 s y es estéreo a 48 000 Hz |

### 8.3 Regresión con huellas

`engine/tests/golden.rs` calcula cada instantánea de `fixtures/snapshots/` con `render_offline` a memoria y compara su **huella** con `fixtures/golden/<nombre>.json`:

```json
{
  "frames": 288000,
  "clippedSamples": 0,
  "peak": [0.4127, 0.3981],
  "segments": [[0.0312, 0.0298], "… 100 pares: valor eficaz de cada canal en cada centésima parte del archivo …"],
  "probes": [[0, 0.0, 0.0], [576, 0.1182, 0.1175], "… 500 tríos: frame, izquierda, derecha, en los frames floor(i · frames / 500) …"]
}
```

La prueba pasa si `frames` y `clippedSamples` son iguales y cada número difiere del guardado en menos de `1e-4`. Con `CARABO_UPDATE_GOLDEN=1`, escribe las huellas en lugar de comparar (§3.3).

### 8.4 Determinismo e independencia del bloque

- **Determinismo** (RNF-14): cada instantánea de `fixtures/snapshots/` se calcula dos veces en el mismo proceso y las dos salidas son idénticas bit a bit.
- **Tamaño de bloque** (06 §16.4, RT-10): cada instantánea se calcula con bloques de 64, 480, 1 024 y 4 096 frames, y con una secuencia de tamaños al azar con semilla fija entre 1 y 4 096; todas las salidas son idénticas bit a bit.

### 8.5 Reglas del hilo de audio

`engine/tests/rt_no_alloc.rs` instala un asignador de memoria global que envuelve al del sistema y cuenta las reservas, las liberaciones y los cambios de tamaño que ocurren mientras una marca local del hilo dice «estoy en el hilo de audio». El asignador no hace nada más: no falla ni provoca un pánico. (Este archivo es el único del crate que usa `unsafe`, para implementar el asignador.)

La prueba activa la marca alrededor de cada llamada a `Processor::process` y a la función de captura, y ejecuta un guion que pasa por todo lo que puede ocurrir en el hilo de audio: reproducción de `p04-mix`, veinte cambios de instantánea (de mezcla y de audio), `Seek`, bucle, `Stop`, escuchas, modo patrón con cambio de patrón, claqueta, cuenta atrás, grabación con captura, tono de prueba, señal de calibración y un bloque con una muestra no finita. Al final, el contador DEBE ser 0 (RNF-11).

Además, la revisión de cada cambio en `engine/src/rt/`, `engine/src/dsp/` y las funciones de `engine/src/io/` que llama `cpal` comprueba a mano las reglas RT-01 a RT-10.

### 8.6 Errores provocados

| Prueba | Comprobación (RNF-12) |
|---|---|
| Muestra no finita | Con un archivo de audio que contiene un valor no numérico, el bloque afectado sale en silencio, se activa la bandera de error interno y el `Processor` sigue aceptando órdenes |
| Pánico en la función de audio | La envoltura de la función que llama `cpal`, ejecutada con un cálculo que provoca un pánico, entrega silencio y activa la bandera, sin propagar el pánico |
| Cola de órdenes llena | 1 000 órdenes `SetMetronome` alternas, enviadas desde un hilo mientras otro ejecuta el `Processor` cada 10 ms, llegan todas y en orden |
| Desechos | Tras cien cambios de instantánea, con el arnés vaciando la cola de desechos como lo hace el hilo de control, las cien instantáneas anteriores han llegado a esa cola: ninguna se pierde ni se libera dentro del `Processor` |

### 8.7 Rendimiento

`engine/tests/perf.rs` (pruebas `perf_*`, con `#[ignore]`, en modo optimizado):

| Prueba | Criterio |
|---|---|
| `perf_render_max_load` | El proyecto de carga máxima reducido a 1 minuto (§9) se calcula a memoria, sin contar la carga del audio, en 15 segundos o menos: al menos 4 veces más rápido que el tiempo real (RNF-04). A esa velocidad, los 10 minutos completos tardan como mucho 150 segundos (RNF-08) |
| `perf_block_worst_case` | Con ese mismo proyecto, ningún bloque de 480 frames tarda más de 5 ms en calcularse (la mitad de su duración), descartando los 10 primeros |

## 9. Proyecto de carga máxima

Es el proyecto con el que se miden los requisitos de rendimiento (02 §12).

| Aspecto | Valor |
|---|---|
| Compás y tempo | 4/4, 120. Un compás dura 2 segundos, así que 10 minutos son 300 compases y exactamente `MAX_SONG_FRAMES` |
| Pistas de ritmo | 4, con los kits `acoustic`, `electronic`, `percussion` y `acoustic`. Cada una tiene un patrón de 4 compases con nivel 2 en los 64 pasos de sus 8 filas y un bloque desde el compás 1 con toda la duración |
| Pistas de audio | 12. Cada una tiene un clip desde el tick 0 con toda la duración, sin «Repetir», sobre una toma mono propia de esa misma duración |
| Contenido de las tomas | `muestra[k] = 0,05 · sin(2π · (110 · n) · k / 48000)`, con `n` de 1 a 12 según la pista |
| Mezcla | En las 16 pistas: volumen −12 dB, panorama repartido de −75 a +75 en pasos de 10, ecualizador activado (graves +3, medios −2, agudos +2) y reverb activada (cantidad 20, tamaño 50). Maestro a 0 dB |

Dos formas de obtenerlo:

- **Como carpeta de proyecto**, para las pruebas manuales: `node tools/gen-max-load-project.mjs <carpeta> [--minutes N]` (07 §13.2). Con 10 minutos ocupa 1,4 GB. La prueba `tools/gen-max-load-project.test.mjs` lo genera con `--minutes 0.1` y comprueba que el resultado se abre con `core/validate`.
- **Como instantánea**, para las pruebas de rendimiento del motor: `engine/tests/common/mod.rs` la construye con la duración que se le pida, escribiendo las tomas en una carpeta temporal.

La duración admite cualquier número de minutos que dé un número entero de compases.

## 10. Comprobaciones estáticas (S)

| Comprobación | Cómo | Requisito |
|---|---|---|
| Dependencias entre módulos | Reglas `no-restricted-imports` de ESLint (07 §5.4) | 04 §9 |
| Tipos | `npm run typecheck` | — |
| Formato y avisos | `prettier --check`, `cargo fmt --check`, Clippy sin avisos | 07 §8 |
| Textos en los dos idiomas | `src/i18n/i18n.test.ts`: `es.json` y `en.json` tienen exactamente las mismas claves; ningún valor está vacío; toda clave `….label` de un control tiene su `….help` cuando 03 la define; los mensajes `msg.E…` de los dos archivos usan los mismos marcadores `{…}` | RNF-17 |
| Textos fuera de los archivos de traducción | La misma prueba recorre `src/ui/**/*.tsx` y falla si encuentra texto con letras escrito directamente entre etiquetas o en los atributos `aria-label`, `title` o `placeholder` | RNF-17 |
| Mensajes sin datos internos | La misma prueba comprueba que ningún valor contiene `E-` seguido de tres cifras, una ruta de código ni la palabra `Error:` | RF-007 CA1 |
| Configuración de la aplicación | `tools/config.test.mjs` lee `src-tauri/tauri.conf.json` y `capabilities/default.json` y comprueba: una sola ventana, tamaño mínimo igual a `WINDOW_MIN_WIDTH` × `WINDOW_MIN_HEIGHT`, `csp` sin ningún origen `http:` ni `https:` salvo `http://ipc.localhost`, y exactamente los permisos de 07 §5.7 | RF-001, RF-009 CA2, RNF-18 |
| Atajos | `src/services/shortcuts.test.ts` compara la tabla de atajos del servicio con una copia literal de la tabla de RF-703 | RF-703 CA1 |
| Sonidos sustitutos | `tools/gen-placeholder-kits.test.mjs` genera los 24 archivos en una carpeta temporal y comprueba que son idénticos a los de `assets/kits/` | 05 §8.4 |
| Licencias | `npm run licenses` | RNF-19 |
| Trazabilidad | `npm run traceability` (§13.3) | RNF-21 |

## 11. Pruebas del programa nativo (N)

Cada comando de `src-tauri/src/commands/` es una envoltura de pocas líneas sobre una función normal de Rust que recibe rutas y datos. Las pruebas llaman a esa función con carpetas temporales; no hace falta arrancar Tauri.

| Qué | Comprobación |
|---|---|
| `project_create_temp` (RF-020 CA2) | La carpeta existe dentro de `unsaved/` y contiene `audio/`, `peaks/` y `recovery/` vacías |
| `project_write` y `project_read` (RF-023) | Lo leído es idéntico a lo escrito. Con el archivo protegido contra escritura: error `IO` y el archivo no cambia (CA2). Archivo inexistente: `NOT_FOUND` |
| `project_save_as` (RF-024) | Crea `parentDir/name/` con `project.carabo` y solo los archivos de `files` y sus `.peaks` (CA1, CA2). Nombres no válidos (`a/b`, vacío, más de `NAME_MAX_CHARS`, terminado en punto, nombres reservados de Windows): `NAME_INVALID` (CA3). Carpeta ya existente: `ALREADY_EXISTS`. Si falla a medias, no queda nada creado |
| `project_cleanup` (RF-029 CA1) | Borra de `audio/` y `peaks/` lo que no está en `keepFiles` y nada más |
| `project_discard_temp` | Borra una carpeta de `unsaved/`; una ruta fuera de `unsaved/` se rechaza y no se borra nada |
| `project_check_files` | Devuelve exactamente los archivos que faltan |
| Copia de recuperación y sesión | `recovery_write`, `recovery_read` (con su fecha) y `recovery_delete`; `session_get` sin archivo devuelve `null` |
| Escritura atómica (RNF-13) | `atomic.rs` escribe en un archivo temporal de la misma carpeta y después lo renombra. Si se interrumpe entre los dos pasos (la prueba ejecuta solo el primero), el archivo de destino conserva su contenido anterior completo; la siguiente escritura elimina el temporal sobrante |
| Registro (RNF-20) | Cada línea lleva fecha, hora, nivel y mensaje; al pasar de 1 MB el archivo rota y se conservan 3 |
| Exportación | `export_start` produce el archivo; con `export_cancel`, termina con `CANCELLED` y no queda archivo (RF-602 CA1); una segunda exportación simultánea da `BUSY` |
| `audio_discard` | Borra la toma y su `.peaks`; una ruta fuera de `audio/` se rechaza |
| Errores | Cada `EngineError` se convierte en el `BridgeErrorCode` de 04 §6.3; un error de E/S «no encontrado» da `NOT_FOUND` y los demás, `IO` |
| Eventos | Cada `EngineEvent` se convierte en el nombre de evento y el contenido de 04 §6.4 |
| Versión (RF-704 CA1) | La versión que devuelve `app_info` es la de `package.json` |

## 12. Pruebas de aceptación manuales (PA)

Las ejecuta el autor del proyecto, o alguien a quien se lo encargue, con el programa compilado (`npm run package`) en un equipo real. Un agente no las ejecuta ni las da por pasadas: prepara lo necesario y deja el registro en blanco.

**Registro.** Cada ejecución se anota en `docs/pruebas/PA-nn-AAAA-MM-DD.md` con: fecha; versión y commit; equipo (procesador, memoria, dispositivos de audio); una línea por paso con «pasa» o «no pasa» y lo observado; y el resultado global. Una prueba que no pasa genera una tarea de corrección antes de repetirla.

**Equipo de referencia** (RNF-02): procesador de 4 núcleos, 8 GB de memoria, disco SSD, pantalla de 1920 × 1080, Windows 11, micrófono USB y auriculares.

### PA-01 · Prueba de los cinco minutos

- **Cubre:** RNF-15, PD-01, CU-01.
- **Participantes:** dos personas, por separado, que cumplan 01 §5: tocan un instrumento, han usado alguna vez un DAW sencillo, no han visto Carabo ni han leído nada sobre él. El autor no puede ser participante.
- **Preparación:** equipo de referencia con el micrófono y los auriculares funcionando en Windows; el retardo ya calibrado; Carabo recién arrancado en la pantalla de inicio; un cronómetro; quien dirige la prueba se sienta detrás y no habla.
- **Pasos:**
  1. Leer el encargo de 01 §5, tal cual, y poner en marcha el cronómetro.
  2. Anotar el minuto y segundo en que el participante consigue cada uno de los siete hitos de 01 §5.
  3. No responder a ninguna pregunta sobre el programa. Si el participante se bloquea, se espera.
  4. Parar a los 15 minutos.
  5. Al terminar, preguntar qué le ha costado y anotarlo.
- **Se supera si:** los dos participantes alcanzan los hitos 1 a 4 antes del minuto 5 y los hitos 5 a 7 antes del minuto 15, sin ninguna indicación.

### PA-02 · Precisión de la grabación

- **Cubre:** RNF-09, RF-313 CA1, RF-507 CA1.
- **Preparación:** equipo de referencia. En Ajustes, «Calibrar retardo» con un auricular pegado al micrófono, hasta obtener un resultado.
- **Pasos:**
  1. Proyecto nuevo a 120 en 4/4. Crear una pista de ritmo con el kit «Percusión» y dejar en el patrón solo la fila «Claves» con golpes en los pasos 1, 5, 9 y 13. Alargar su bloque a 32 compases.
  2. Crear una pista de audio (queda armada). Con el auricular pegado al micrófono, grabar desde el compás 1 hasta el final del compás 30 (60 segundos) y parar.
  3. Exportar dos veces a WAV: una con «Solo» en la pista de ritmo (`ritmo.wav`) y otra con «Solo» en la pista de audio (`toma.wav`).
  4. Ejecutar `node tools/measure-offset.mjs ritmo.wav toma.wav` (07 §13.7) y anotar su salida.
  5. Repetir los pasos 2 a 4 con el retardo de grabación cambiado a mano a 0 ms y a 100 ms, y anotar la diferencia entre las dos medidas.
- **Se supera si:** en el paso 4, el desfase de todos los golpes medidos es menor que 5 ms en valor absoluto, al principio, en el centro y al final de la toma; y en el paso 5 las dos medidas difieren en 100 ± 1 ms.

### PA-03 · Rendimiento con carga máxima

- **Cubre:** RNF-03, RNF-05, RNF-06, RNF-07, RNF-08.
- **Preparación:** equipo de referencia, sin otros programas abiertos. Generar el proyecto de carga máxima de 10 minutos (§9). Un cronómetro y el Administrador de tareas de Windows.
- **Pasos y criterios:**

| Paso | Medida | Se supera si |
|---|---|---|
| 1. Arrancar Carabo tres veces | Tiempo hasta ver la pantalla de inicio; se toma el peor | Menos de 3 segundos |
| 2. Con un proyecto vacío abierto y tras visitar las tres pestañas, mirar la memoria del grupo «Carabo» en el Administrador de tareas | Memoria total del grupo | Menos de 700 MB |
| 3. Abrir el proyecto de carga máxima | Tiempo hasta poder mover un deslizador de la pestaña Mezcla y reproducir | Menos de 3 segundos |
| 4. Reproducir en cuanto se pueda | Tiempo, desde la apertura, hasta que suenan las pistas de audio | Menos de 10 segundos |
| 5. Mirar de nuevo la memoria del grupo | Memoria total | Menos de 2,1 GB |
| 6. Reproducir la canción entera desde el compás 1 | Indicador de estado del audio al terminar los 10 minutos | «Audio: correcto» (0 cortes) |
| 7. Durante esa reproducción: arrastrar un clip, arrastrar un deslizador de volumen y desplazar la línea de tiempo | Observación | El movimiento sigue al ratón sin retraso ni saltos apreciables, y el sonido no se corta |
| 8. Exportar a WAV | Tiempo hasta «Exportación terminada» | Menos de 150 segundos |

### PA-04 · Accesibilidad

- **Cubre:** RNF-16, RF-324.
- **Preparación:** el ratón, desconectado o apartado.
- **Pasos:** solo con el teclado, hacer el recorrido completo: cerrar la pantalla de inicio; crear una pista de ritmo; marcar y quitar golpes; cambiar de patrón y de kit; reproducir y parar; cambiar el tempo y el compás; ir a Canción; crear una pista de audio; añadir, mover, alargar, duplicar y eliminar un bloque; mover, recortar, partir y eliminar un clip; ir a Mezcla y cambiar un volumen, un panorama, silenciar, solo y un efecto; abrir Ajustes y cambiar el idioma; guardar como; exportar; deshacer y rehacer; cerrar.
- **Se supera si:** todo el recorrido se completa sin ratón; en todo momento se ve qué control tiene el foco; ningún control atrapa el foco; todo estado (activado, silenciado, armado, seleccionado, saturación) se distingue sin depender del color; y las pruebas automáticas de `e2e/accessibility.spec.ts` pasan.

### PA-05 · Dispositivos reales

- **Cubre:** RF-206 CA1, RF-211 CA2, RF-301 CA1, RF-314 CA1, RF-500 a RF-506, RF-509.
- **Preparación:** equipo de referencia y, además, unos altavoces o segundos auriculares.

| Paso | Se supera si | Criterio |
|---|---|---|
| 1. Borrar `%APPDATA%\com.carabo.app\settings.json`, arrancar, crear una pista de ritmo y reproducir | Suena por el dispositivo predeterminado de Windows sin abrir Ajustes | RF-500 CA1 |
| 2. Pulsar el nombre de una fila; marcar golpes mientras suena el patrón | El sonido llega sin retraso apreciable; el paso resaltado coincide con el golpe que se oye | RF-206 CA1, RF-211 CA2 |
| 3. En Ajustes, elegir la otra salida y reproducir | Suena por la salida nueva | RF-501 CA1 |
| 4. En la configuración de sonido de Windows, poner la salida a 44 100 Hz; reiniciar Carabo; «Probar salida» junto a un afinador o a un tono de 440 Hz de referencia | El tono suena a 440 Hz, sin que Carabo pida cambiar nada | RF-502 CA1 |
| 5. Poner la entrada a 44 100 Hz; grabar 5 segundos de un tono de 440 Hz emitido por otro aparato; exportar con «Solo» en esa pista y medir el tono con `node tools/measure-offset.mjs --tone toma.wav` | 440 ± 1 Hz | RF-502 CA2 |
| 6. Crear una pista de audio con el micrófono conectado; en silencio y hablando | La pista queda armada con su medidor; en silencio marca menos de −40 dBFS; al hablar se mueve | RF-301 CA1, RF-505 CA1 |
| 7. Con la pista armada, hablar al micrófono con el programa parado y reproduciendo | La voz no sale por los auriculares | RF-509 CA1 |
| 8. Calibrar dos veces seguidas con un cable entre la salida y la entrada (o con el auricular pegado al micrófono) | Los dos valores difieren en menos de 2 ms | RF-506 CA1 |
| 9. Silenciar el micrófono en Windows y calibrar | Aparece el diálogo de error y el valor anterior se conserva | RF-506 CA2 |
| 10. Empezar a grabar y desconectar el micrófono a los 10 segundos | Queda un clip de unos 10 segundos y un diálogo lo explica | RF-314 CA1 |
| 11. Reproducir y desconectar los auriculares USB | El programa sigue respondiendo, nombra el dispositivo perdido y se puede guardar | RF-504 CA1 |
| 12. Deshabilitar en Windows todos los dispositivos de salida y arrancar | Se puede editar y guardar; Reproducir muestra el diálogo con «Abrir ajustes»; la barra de estado dice «Audio: sin salida» | RF-503 CA1 |

### PA-06 · Recorrido de la aplicación real

- **Cubre:** RNF-01, RNF-18, RF-001, RF-009 CA1, RF-027 CA1, RF-315, RF-602, RF-702 CA1, RF-704 CA1.
- **Preparación:** la carpeta que produce `npm run package`, copiada a un Windows 11 en el que no se haya desarrollado el programa.

| Paso | Se supera si |
|---|---|
| 1. Ejecutar `Carabo.exe` sin permisos de administrador | Arranca y muestra la pantalla de inicio; no pide instalar nada |
| 2. Intentar reducir la ventana | No baja de 1280 × 720; no aparece ninguna otra ventana |
| 3. Crear una pista de ritmo y reproducir; «Guardar como» con el nombre «Demo» | Suena; la carpeta `Demo` contiene `project.carabo`; el título es `Demo — Carabo` |
| 4. Cambiar algo; cerrar la ventana | El título lleva asterisco; aparece la pregunta de cambios sin guardar; «No guardar» cierra |
| 5. Arrancar y abrir «Demo» desde la lista de recientes | Se abre sin el cambio no guardado |
| 6. Cambiar algo, esperar 130 segundos y terminar el proceso desde el Administrador de tareas; arrancar de nuevo | Se ofrece recuperar; «Recuperar» restaura el cambio y el título lleva asterisco |
| 7. Arrastrar un archivo WAV y uno MP3 desde el Explorador a la pestaña Canción | Aparecen como clips, con su onda, y suenan |
| 8. Exportar a WAV y a MP3; pulsar «Abrir carpeta» | Los dos archivos se reproducen en otro programa; el Explorador se abre en su carpeta |
| 9. Activar el modo avión de Windows y repetir los pasos 3, 7 y 8 | Todo funciona igual |
| 10. Cambiar el idioma a inglés, cerrar y arrancar | Arranca en inglés |
| 11. Abrir Ajustes y mirar «Acerca de» | La versión coincide con la de `package.json` |

## 13. Trazabilidad

### 13.1 Dónde se comprueba cada criterio

La tabla da, para cada requisito [1.0], el nivel (§2) de la prueba que cubre cada criterio de aceptación. `1` significa `CA1`. Cuando hay dos niveles, hacen falta las dos pruebas. Los criterios de los requisitos [1.1] se asignarán al planificar la 1.1 (09).

| Requisito | Criterio → nivel | Requisito | Criterio → nivel |
|---|---|---|---|
| RF-001 | 1 S · 2 U · 3 S, PA-06 | RF-211 | 1 M · 2 PA-05 |
| RF-002 | 1 E · 2 E | RF-212 | 1 U |
| RF-003 | 1 E · 2 E | RF-213 | 1 M |
| RF-004 | 1 E · 2 E | RF-300 | 1 E |
| RF-005 | 1 E · 2 E | RF-301 | 1 C, PA-05 · 2 U |
| RF-006 | 1 C · 2 C | RF-302 | 1 U · 2 U |
| RF-007 | 1 S · 2 C | RF-303 | 1 C |
| RF-008 | 1 C · 2 C | RF-304 | 1 C |
| RF-009 | 1 PA-06 · 2 S | RF-305 | 1 C |
| RF-010 | 1 C · 2 E | RF-306 | 1 U |
| RF-020 | 1 U · 2 N | RF-307 | 1 U |
| RF-021 | 1 U · 2 C | RF-308 | 1 M · 2 U |
| RF-022 | 1 U · 2 U | RF-309 | 1 U, M · 2 U |
| RF-023 | 1 U, N · 2 N, C | RF-310 | 1 U · 2 U |
| RF-024 | 1 N, E · 2 N · 3 N, C | RF-311 | 1 M · 2 C |
| RF-025 | 1 U · 2 U | RF-312 | 1 C |
| RF-026 | 1 U · 2 U · 3 U | RF-313 | 1 PA-02 · 2 U |
| RF-027 | 1 U, PA-06 · 2 U · 3 U | RF-314 | 1 M, PA-05 |
| RF-028 | 1 C · 2 U, M | RF-315 | 1 M · 2 M, C · 3 E |
| RF-029 | 1 N, U · 2 U | RF-316 | 1 U |
| RF-030 | 1 U · 2 U · 3 U | RF-317 | 1 U · 2 U |
| RF-031 | 1 U · 2 U | RF-318 | 1 M · 2 U |
| RF-032 | 1 U · 2 C | RF-319 | 1 M |
| RF-034 | 1 U | RF-320 | 1 U |
| RF-100 | 1 E | RF-321 | 1 U |
| RF-101 | 1 C · 2 M | RF-322 | 1 M |
| RF-102 | 1 M · 2 U | RF-323 | 1 C |
| RF-103 | 1 C | RF-324 | 1 E, PA-04 |
| RF-104 | 1 U · 2 C | RF-400 | 1 E |
| RF-105 | 1 U | RF-401 | 1 M · 2 M |
| RF-106 | 1 C · 2 M | RF-402 | 1 M · 2 M |
| RF-107 | 1 U · 2 C | RF-403 | 1 M · 2 M |
| RF-108 | 1 C | RF-404 | 1 M · 2 C |
| RF-109 | 1 U · 2 U | RF-405 | 1 M |
| RF-110 | 1 M · 2 M | RF-406 | 1 M · 2 M |
| RF-111 | 1 M · 2 M | RF-407 | 1 M · 2 M |
| RF-112 | 1 U · 2 M · 3 U | RF-408 | 1 M |
| RF-113 | 1 U | RF-500 | 1 PA-05 |
| RF-114 | 1 U | RF-501 | 1 PA-05 |
| RF-200 | 1 C | RF-502 | 1 M, PA-05 · 2 M, PA-05 |
| RF-201 | 1 U · 2 U | RF-503 | 1 E |
| RF-202 | 1 C | RF-504 | 1 C, PA-05 |
| RF-203 | 1 U | RF-505 | 1 PA-05 |
| RF-204 | 1 C | RF-506 | 1 PA-05 · 2 M, C |
| RF-205 | 1 U · 2 C | RF-507 | 1 U, PA-02 · 2 U |
| RF-206 | 1 M, PA-05 | RF-508 | 1 U |
| RF-207 | 1 U | RF-509 | 1 PA-05 |
| RF-208 | 1 U · 2 C | RF-600 | 1 C |
| RF-209 | 1 U · 2 U | RF-601 | 1 M · 2 M · 3 M |
| RF-210 | 1 U | RF-602 | 1 N |
| RF-700 | 1 C | RF-603 | 1 M, C |
| RF-701 | 1 E | RF-703 | 1 S · 2 C |
| RF-702 | 1 U, PA-06 | RF-704 | 1 N, PA-06 |

### 13.2 Cómo se nombra una prueba

Toda prueba automática que cubre un criterio de aceptación lleva su identificador en el nombre:

- TypeScript y Playwright: el título empieza por el identificador. `it("RF-205 CA1: two clicks leave the step unchanged and add two history entries", …)`.
- Rust: el nombre de la función empieza por el identificador en minúsculas y con guiones bajos. `fn rf_211_ca1_hits_start_on_their_exact_frame()`.

Una prueba puede cubrir varios criterios (se ponen todos: `"RF-401 CA1, RF-401 CA2: …"`; en Rust, una función por criterio). Las pruebas que no corresponden a ningún criterio se nombran con libertad.

### 13.3 Comprobación automática

`npm run traceability` ejecuta `tools/check-traceability.mjs` (07 §13.6), que:

1. Lee `docs/spec/02-requisitos.md` y obtiene la lista de criterios de los requisitos [1.0].
2. Busca identificadores de criterio en los nombres de las pruebas de `src/`, `e2e/`, `tools/`, `engine/` y `src-tauri/`, con las dos formas de §13.2.
3. Lee `fixtures/manual-coverage.json`, que asocia cada criterio cubierto a mano con su prueba: `{ "RF-009 CA1": "PA-06", … }`. Su contenido son las entradas `PA-…` de la tabla de §13.1.
4. Escribe un informe: criterios con prueba automática, criterios solo manuales y criterios sin cubrir.
5. Con la opción `--strict`, termina con código 1 si queda algún criterio sin cubrir. Sin ella, termina siempre con código 0.

Durante el desarrollo se usa sin `--strict`, como informe. La última tarea de la versión 1.0 (09) añade `npm run traceability -- --strict` a la comprobación completa.

### 13.4 Requisitos no funcionales

| Requisito | Cómo se comprueba |
|---|---|
| RNF-01 | PA-06 |
| RNF-02 | Define el equipo de las pruebas manuales (§12) |
| RNF-03 | PA-03, paso 6 |
| RNF-04 | `perf_render_max_load` (§8.7) |
| RNF-05 | `e2e/performance.spec.ts` (§6) y PA-03, paso 7 |
| RNF-06 | PA-03, pasos 1, 3 y 4 |
| RNF-07 | PA-03, pasos 2 y 5 |
| RNF-08 | `perf_render_max_load` (§8.7) y PA-03, paso 8 |
| RNF-09 | PA-02 |
| RNF-10 | Pruebas de frame exacto (§8.2) |
| RNF-11 | `rt_no_alloc` y revisión (§8.5) |
| RNF-12 | Errores provocados (§8.6); prueba C del mensaje E-150 |
| RNF-13 | Escritura atómica (§11) |
| RNF-14 | Determinismo (§8.4) |
| RNF-15 | PA-01 |
| RNF-16 | `e2e/accessibility.spec.ts` (§6), reglas de §5.1 y PA-04 |
| RNF-17 | Pruebas de textos (§10) |
| RNF-18 | Configuración de la aplicación (§10) y PA-06, paso 9 |
| RNF-19 | `npm run licenses` |
| RNF-20 | Registro (§11) |
| RNF-21 | `npm run traceability` (§13.3) |
| RNF-22 | Pruebas de límites de `core/commands/` (§4.1) y validación de la instantánea (§8.2) |

## 14. Qué pruebas trae cada tarea

Una tarea del plan (09) no está terminada hasta que:

1. Existen las pruebas de los criterios de aceptación que la tarea cita, en el nivel que indica §13.1 y con el nombre de §13.2.
2. Existen las pruebas mínimas del módulo que la tarea crea o cambia (§4, §5, §8, §11).
3. Si la tarea añade una función a `core/commands/`, está en la prueba de invariantes (§4.2).
4. Si la tarea cambia algo que cruza el puente, están actualizados el archivo de `fixtures/bridge/`, `mockBridge` y las dos pruebas de contrato (§7).
5. Si la tarea cambia el sonido que produce el motor a propósito, las huellas se han regenerado y la solicitud de integración explica por qué el resultado nuevo es correcto (§3.3).
6. Pasa la comprobación completa (07 §9).
7. Lo que la tarea no ha podido comprobar por falta de dispositivo de audio está escrito en la solicitud de integración, con el paso de la prueba manual que lo cubre.
