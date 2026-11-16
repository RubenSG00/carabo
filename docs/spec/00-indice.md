# Carabo · Especificación · 00 — Índice, convenciones y glosario

> **PROMPT PARA EL AGENTE — léelo antes de seguir**
>
> Eres un agente de programación que trabaja en el repositorio de **Carabo**, un DAW (estación de trabajo de audio digital) de escritorio para Windows 11. No has participado en el diseño y no puedes preguntar a nadie: todo lo que necesitas está escrito en los 11 documentos de `docs/spec/`.
>
> Este documento (00) es la puerta de entrada. Úsalo para:
>
> 1. saber qué documento leer para tu tarea (§2);
> 2. conocer las reglas comunes a todos los documentos (§3 a §6);
> 3. consultar el significado exacto de un término (§7) y el porqué de una decisión (§8).
>
> Reglas que se aplican siempre:
>
> - Lee este documento entero una vez. Después lee solo los documentos y secciones que tu tarea indique en `09-plan-de-implementacion.md`.
> - Haz lo que está escrito y solo eso. No añadas funciones, opciones ni preferencias que no figuren en la especificación.
> - Usa cada término con el significado del glosario (§7) y cada identificador de código tal como aparece en él.
> - Si encuentras un hueco o una contradicción, aplica §6. No lo resuelvas en silencio.

---

## 1. Qué es Carabo

Carabo es un DAW de escritorio para Windows 11 cuya virtud principal es resultar intuitivo. Sirve para montar una base rítmica con patrones de pasos, grabar audio encima con un micrófono, mezclar y exportar la canción a un archivo de audio.

- La interfaz se escribe en TypeScript con React y se ejecuta dentro de Tauri 2.
- El motor de audio se escribe en Rust y es una biblioteca independiente de la interfaz.
- El proyecto es de código abierto con licencia MIT.
- Hay tres niveles de alcance: versión **1.0** (lo imprescindible), versión **1.1** (lo deseable) y **posteriores** (solo esbozado). El detalle está en `01-producto-y-alcance.md` §6.

## 2. Mapa de documentos

Todos los documentos están en `docs/spec/`. Cada uno empieza con un prompt que dice para qué sirve y cuándo leerlo, y cada sección se entiende sin haber leído las demás.

| Nº | Archivo | Contenido | Léelo cuando… |
|---|---|---|---|
| 00 | `00-indice.md` | Mapa, convenciones, constantes, glosario y registro de decisiones | Siempre, una vez |
| 01 | `01-producto-y-alcance.md` | Visión, usuario, casos de uso, principios de diseño, alcance por versión y fuera de alcance | Cuando tu tarea lo cite; §4 al resolver un hueco (§6 de este documento) y §6 cuando dudes de si algo entra o no |
| 02 | `02-requisitos.md` | Requisitos funcionales (RF) y no funcionales (RNF) con criterios de aceptación | En toda tarea: lee los RF que la tarea cite |
| 03 | `03-interfaz.md` | Pantallas, componentes, estados, flujos paso a paso, atajos, textos y mensajes de error | En tareas de interfaz |
| 04 | `04-arquitectura.md` | Módulos, responsabilidades, API del puente entre interfaz y motor, hilos y diagramas | Antes de crear o modificar un módulo o un comando del puente |
| 05 | `05-datos-y-formato.md` | Modelo de datos, formato del proyecto, instantánea del motor, ajustes, kits y ritmos hechos, con esquemas y ejemplos | Al leer o escribir cualquier estructura de datos o archivo |
| 06 | `06-motor-de-audio.md` | Motor en Rust: hilos, reglas de tiempo real, recorrido de la señal y algoritmos | En tareas del motor |
| 07 | `07-tecnologia-y-repositorio.md` | Tecnologías con versiones, árbol de carpetas, convenciones de código y órdenes de compilación | Antes de crear archivos o añadir dependencias |
| 08 | `08-pruebas.md` | Estrategia de pruebas y pruebas de aceptación | En toda tarea: cada tarea incluye sus pruebas |
| 09 | `09-plan-de-implementacion.md` | Fases y tareas (T-nnn) ordenadas por dependencias | Para saber qué hacer y en qué orden |
| 10 | `10-riesgos-supuestos-cuestiones.md` | Riesgos, supuestos y cuestiones abiertas | Cuando tu tarea toque un punto marcado como delicado |

La versión 1.1 añadirá un documento más, `11-version-1-1.md`, que se escribe cuando la 1.0 esté terminada (09 §17).

## 3. Convenciones de la especificación

### 3.1 Identificadores

Los identificadores son estables: no se renumeran ni se reutilizan.

| Prefijo | Qué identifica | Dónde se define |
|---|---|---|
| `RF-nnn` | Requisito funcional | 02 |
| `RNF-nn` | Requisito no funcional | 02 |
| `PD-nn` | Principio de diseño | 01 §4 |
| `CU-nn` | Caso de uso | 01 §3 |
| `PA-nn` | Prueba de aceptación manual | 08 |
| `E-nnn` | Mensaje de error o aviso | 03 |
| `T-nnn` | Tarea del plan de implementación | 09 |
| `RG-nn` | Riesgo | 10 |
| `SUP-nn` | Supuesto | 10 |
| `Q-nn` | Cuestión abierta | 10 |
| `DEC-nn` | Decisión de diseño | 00 §8 |
| `AR-nn` | Principio de arquitectura | 04 §2 |
| `INV-nn` | Invariante del proyecto | 05 §4 |
| `RT-nn` | Regla del hilo de audio | 06 §4 |
| `DI-n` | Decisión de implementación tomada por un agente ante un hueco | `docs/DECISIONES-DE-IMPLEMENTACION.md` (§6) |

Dentro de cada requisito, los criterios de aceptación se numeran `CA1`, `CA2`… y se citan como `RF-204 CA2`.

### 3.2 Etiquetas de versión

Cada requisito, pantalla, campo de datos o tarea lleva una etiqueta:

- **[1.0]**: entra en la versión 1.0.
- **[1.1]**: entra en la versión 1.1. No lo implementes en una tarea de la 1.0, pero no lo impidas: los datos y las interfaces de la 1.0 ya le dejan sitio.
- **[Posterior]**: versiones futuras. Solo se describe lo necesario para no cerrarle el paso.
- **[Fuera]**: no se construirá. Si una tarea parece pedirlo, la tarea está mal leída.

Lo que no lleva etiqueta es [1.0].

### 3.3 Palabras normativas

- **DEBE**: obligatorio. Si no se cumple, la tarea no está terminada.
- **NO DEBE**: prohibido.
- **PUEDE**: permitido; elige lo más simple.

### 3.4 Idiomas

- Los documentos de `docs/spec/` están en español.
- El código, los nombres de archivos de código, los identificadores, los comentarios y los mensajes de *commit* van en inglés.
- Los textos que ve el usuario existen en español y en inglés y nunca se escriben dentro del código: van en los archivos de traducción (03 y 07).

### 3.5 Unidades y notación

- **Frame**: una muestra de audio por canal. A 48 000 Hz, 48 000 frames son un segundo. Todas las duraciones de audio se expresan en frames (enteros).
- **Tick**: unidad de tiempo musical. Una negra son 960 ticks. Todas las posiciones musicales de clips y del cursor se expresan en ticks (enteros).
- **Compás**: en los datos se numera desde 0; en pantalla, desde 1. «Compás 1» en pantalla es `bar = 0` en los datos.
- **Paso**: una semicorchea, 240 ticks. En los datos se numera desde 0.
- **dB**: decibelios de ganancia. **dBFS**: decibelios respecto al máximo digital (0 dBFS equivale a una amplitud de 1,0).
- Conversión entre ganancia lineal y dB: `gain = 10^(dB / 20)`.
- En el texto de los documentos los decimales llevan coma (`0,5`); en código y en JSON llevan punto (`0.5`).

### 3.6 Referencias cruzadas

- `02 §4` significa «sección 4 del documento 02».
- `RF-204` remite al requisito con ese identificador en el documento 02.
- Las rutas de archivo son relativas a la raíz del repositorio.

## 4. Constantes globales

Estos valores son únicos en todo el proyecto. En código se definen una sola vez por lenguaje, en el archivo de constantes que indica el documento 07, y los demás módulos los importan de ahí. NO DEBE haber copias de estos números en otros archivos.

| Nombre en código | Valor | Significado |
|---|---|---|
| `APP_NAME` | `"Carabo"` | Nombre visible del programa |
| `PROJECT_FILE_NAME` | `"project.carabo"` | Nombre del archivo de proyecto dentro de la carpeta de proyecto |
| `FORMAT_VERSION` | `1` | Versión del formato del archivo de proyecto |
| `SAMPLE_RATE` | `48000` | Frecuencia de muestreo interna, en Hz |
| `PPQ` | `960` | Ticks por negra |
| `TICKS_PER_STEP` | `240` | Ticks por paso (semicorchea) |
| `TEMPO_MIN` / `TEMPO_MAX` / `TEMPO_DEFAULT` | `40` / `240` / `120` | Tempo en negras por minuto, siempre entero |
| `TIME_SIGNATURES` | `4/4`, `3/4`, `6/8` | Compases admitidos; el valor inicial es `4/4` |
| `STEPS_PER_BAR` | `16` (4/4), `12` (3/4), `12` (6/8) | Pasos por compás |
| `MAX_TRACKS` | `16` | Máximo de pistas por proyecto, sumando las de ritmo y las de audio |
| `MAX_SONG_FRAMES` | `28800000` | Duración máxima de la canción: 10 minutos |
| `MAX_PATTERNS_PER_TRACK` | `32` | Máximo de patrones por pista de ritmo |
| `PATTERN_BARS` | `1`, `2`, `4` | Longitudes de patrón admitidas, en compases |
| `KIT_ROWS` | `8` | Filas (sonidos) por kit |
| `FIRST_BLOCK_BARS` | `16` | Longitud del bloque que se coloca solo al crear una pista de ritmo |
| `MIN_CLIP_FRAMES` | `2400` | Longitud mínima de un clip de audio: 50 ms |
| `MICROFADE_FRAMES` | `240` | Duración de un microfundido: 5 ms |
| `COUNT_IN_BARS` | `1` | Compases de cuenta atrás antes de grabar |
| `DEFAULT_LATENCY_FRAMES` | `2400` | Retardo de ida y vuelta que se supone mientras no haya calibración: 50 ms |
| `RECOVERY_INTERVAL_S` | `120` | Segundos entre copias de recuperación |
| `HISTORY_LIMIT` | `200` | Máximo de pasos de historial que se conservan |
| `RECENTS_MAX` | `10` | Máximo de proyectos recientes |
| `NAME_MAX_CHARS` | `40` | Longitud máxima de un nombre de pista, patrón o proyecto |
| `FADER_MIN_DB` / `FADER_MAX_DB` | `-60` / `6` | Recorrido de un control de volumen; por debajo de −60 dB la ganancia es 0 |
| `ROW_GAIN_MIN_DB` / `ROW_GAIN_MAX_DB` | `-48` / `6` | Recorrido del volumen de una fila |
| `EQ_GAIN_MIN_DB` / `EQ_GAIN_MAX_DB` | `-12` / `12` | Recorrido de cada banda del ecualizador |
| `EQ_LOW_HZ` / `EQ_MID_HZ` / `EQ_HIGH_HZ` | `150` / `1000` / `6000` | Frecuencias fijas de las tres bandas |
| `LEVEL_GAINS` | `[0, 0.45, 0.75, 1.0]` | Ganancia de un golpe según su nivel: 0 apagado, 1 suave, 2 normal, 3 fuerte |
| `MAX_VOICES` | `32` | Voces simultáneas por pista de ritmo |
| `TELEMETRY_HZ` | `30` | Frecuencia con la que el motor informa a la interfaz |
| `EXPORT_TAIL_FRAMES` | `96000` | Cola que se añade al exportar: 2 segundos |
| `ZOOM_LEVELS` | `6, 12, 24, 48, 96, 192` | Píxeles por negra en la pestaña Canción; el valor inicial es `24` |
| `WINDOW_MIN_WIDTH` / `WINDOW_MIN_HEIGHT` | `1280` / `720` | Tamaño mínimo de la ventana, en píxeles |

Dos constantes quedan fuera de esta tabla porque no afectan al comportamiento que describe el documento 02: `REPOSITORY_URL`, la dirección del repositorio que muestra «Acerca de» (07 §7), y `MAX_BLOCK`, interna del motor (06 §19).

## 5. Precedencia entre documentos

Los documentos no deberían contradecirse. Si ocurre, manda este orden y la contradicción se anota según §6:

1. Para **qué hace el programa** y sus criterios de aceptación manda el documento 02.
2. Para **estructuras de datos y archivos** manda el documento 05.
3. Para **nombres y firmas de la API del puente** manda el documento 04.
4. Para **textos, disposición y atajos** manda el documento 03.
5. Para **algoritmos del motor** manda el documento 06.
6. Para **versiones, rutas y convenciones de código** manda el documento 07.
7. Para **qué pruebas hacen falta y cómo se nombran** manda el documento 08.
8. Para **el orden del trabajo y el alcance de cada tarea** manda el documento 09.
9. Los principios de diseño (01 §4) deciden cualquier caso que los anteriores no resuelvan.

## 6. Qué hacer ante un hueco o una contradicción

Un hueco es una situación que tu tarea necesita resolver y que ningún documento describe.

1. Busca el término en el glosario (§7) y los RF del área en el documento 02. Muchos huecos aparentes están resueltos allí.
2. Si sigue sin resolverse, elige el comportamiento **más simple** que cumpla los principios de diseño (01 §4). En particular: no añadas opciones (PD-06), no añadas una segunda manera de hacer algo (PD-05) y haz que el cambio se pueda deshacer (PD-08).
3. Impleméntalo y regístralo en `docs/DECISIONES-DE-IMPLEMENTACION.md`, añadiendo al final una entrada con este formato exacto:

```markdown
## DI-<número consecutivo> · <fecha AAAA-MM-DD> · <tarea T-nnn>
- **Hueco o contradicción:** <qué faltaba o qué chocaba, citando documento y sección>
- **Decisión tomada:** <qué has implementado>
- **Por qué:** <principio PD-nn o regla en la que te apoyas>
- **Archivos afectados:** <rutas>
```

4. Nunca resuelvas un hueco ampliando el alcance. Si la única salida es construir algo marcado [1.1], [Posterior] o [Fuera], no lo construyas: deja la función sin ese caso, muestra un mensaje de error conforme a RF-007 si el usuario puede llegar a él, y regístralo.

## 7. Glosario

Un mismo término significa siempre lo mismo en todos los documentos. La columna «En código» da el identificador en inglés que se usa en TypeScript y en Rust (con la capitalización propia de cada lenguaje).

| Término | En código | Definición |
|---|---|---|
| Ajuste a la rejilla | `snapEnabled` | Modo en el que mover o recortar un clip de audio redondea la posición al paso más cercano. Los bloques de patrón se ajustan siempre a compases, esté o no activado. |
| Almacén de audio | `AudioPool` | Conjunto de archivos de audio que el motor tiene cargados enteros en memoria para hacerlos sonar sin leer del disco (06 §9.1). |
| Archivo de audio (fuente) | `AudioSource` | Archivo WAV de la subcarpeta `audio/` del proyecto: 48 000 Hz, coma flotante de 32 bits, 1 o 2 canales. Una vez creado no se modifica nunca. |
| Archivo de proyecto | `project.carabo` | Archivo JSON que contiene todo el estado del proyecto salvo el audio. |
| Barra de estado | `StatusBar` | Franja inferior de la ventana con mensajes breves y el indicador de estado del audio. |
| Barra de reproducción | `TransportBar` | Franja con Reproducir, Grabar, posición, tempo, compás, claqueta y bucle. Es común a todas las pestañas y siempre está visible. |
| Barra superior | `TopBar` | Franja con el menú Archivo, Deshacer, Rehacer, las pestañas y Ajustes. |
| Bloque de audio | *block* | Grupo de frames que el dispositivo de sonido pide de una vez al motor. En el documento 06, «bloque» a secas significa esto; el bloque de patrón se nombra allí siempre completo. |
| Bloque de patrón (bloque) | `PatternBlock` | Aparición de un patrón en la canción: ocupa un número entero de compases en el carril de su pista de ritmo y hace sonar el patrón repetido durante ese tramo. Varios bloques pueden referirse al mismo patrón. |
| Bucle | `loop` | Tramo de la canción, en compases enteros, que se repite al reproducir en modo canción cuando está activado. |
| Calibración | `calibration` | Medición automática del retardo de ida y vuelta del equipo: el programa emite clics, los graba por el micrófono y calcula cuánto tardan. |
| Canal de mezcla (canal) | `ChannelStrip` | Columna de la pestaña Mezcla que reúne los controles de una pista: efectos, panorama, silenciar, solo, volumen y medidor. |
| Canal maestro | `master` | Canal por el que pasa la suma de todas las pistas antes de llegar a la salida. |
| Canción | `song` | Disposición en el tiempo de los bloques y clips de todas las pistas. Es lo que suena en modo canción y lo que se exporta. |
| Captura | `capture` | Traslado de un tramo exacto de la señal de entrada a un archivo (al grabar una toma) o a memoria (al calibrar) (06 §14.1). |
| Carpeta de proyecto | — | Carpeta que contiene el archivo de proyecto y sus subcarpetas `audio/`, `peaks/` y `recovery/`. Su nombre es el nombre del proyecto. |
| Carril | `lane` | Franja horizontal de una pista en la pestaña Canción, donde se colocan sus bloques o clips. |
| Claqueta | `metronome` | Clic que marca los pulsos mientras se reproduce o se graba. No se exporta nunca. |
| Clip de audio (clip) | `AudioClip` | Fragmento de un archivo de audio colocado en el carril de una pista de audio. Un clip no contiene audio: apunta a un archivo y dice qué región de él suena y dónde. |
| Compás (indicación) | `timeSignature` | Indicación métrica del proyecto: `4/4`, `3/4` o `6/8`. |
| Compás (unidad) | `bar` | Unidad de tiempo musical que agrupa pulsos. En 4/4 dura 4 negras (3 840 ticks); en 3/4 y en 6/8 dura 3 negras (2 880 ticks). |
| Comprobación completa | — | Lista de órdenes de análisis, pruebas y licencias que debe pasar entera antes de dar una tarea por terminada (07 §9). |
| Copia de recuperación | `recovery` | Copia del estado del proyecto que el programa escribe por su cuenta en `recovery/` para no perder trabajo si se cierra de golpe. No sustituye al guardado manual ni toca el archivo de proyecto. |
| Criterio de aceptación | `CA` | Comprobación concreta con la que se decide si un requisito se cumple. Se cita como `RF-204 CA2`. |
| Cuenta atrás | `countIn` | Compás de clics que suena antes de empezar a grabar. |
| Cursor de reproducción (cursor) | `playhead` | Posición de la canción desde la que se reproduce o se graba. Se expresa en ticks. |
| Ecualizador | `eq` | Efecto que sube o baja tres bandas de frecuencia: graves, medios y agudos. |
| Edición no destructiva | — | Ninguna operación de edición modifica ni borra un archivo de audio. Recortar, partir o tapar solo cambia los datos del clip. |
| Estado de vista | `view` | Parte del proyecto que se guarda pero no entra en el historial ni marca cambios sin guardar: claqueta, bucle, ajuste a la rejilla y zoom. |
| Exportar | `export` | Generar un archivo WAV o MP3 con la mezcla de la canción. |
| Fila | `row` | Línea horizontal de la rejilla de pasos. Hay 8 filas y cada una hace sonar un sonido del kit. |
| Frame | `frame` | Una muestra de audio por canal. |
| Gesto continuo | `gesture` | Interacción que cambia un valor muchas veces seguidas (arrastrar un control de volumen, un clip o un borde). Mientras dura no genera pasos de historial; al soltar genera uno solo. |
| Golpe | `hit` | Sonido del kit que se dispara en un paso de una fila. |
| Grupo de corte | `chokeGroup` | Conjunto de sonidos de un kit que se cortan entre sí: al sonar uno, calla cualquier otro del mismo grupo (por ejemplo, el charles cerrado corta al abierto). |
| Hilo de audio | *audio thread* | Hilo del sistema que pide al motor cada bloque de sonido. Tiene restricciones de tiempo real (06). |
| Huella | *fingerprint* | Resumen numérico de una exportación que las pruebas guardan y comparan para detectar cambios involuntarios en el sonido (08 §8.3). |
| Importar | `import` | Convertir un archivo WAV o MP3 externo en un archivo de audio del proyecto y crear un clip con él. |
| Instantánea del motor | `EngineSnapshot` | Descripción completa e inmutable de lo que debe sonar, que la interfaz envía al motor cada vez que cambia el proyecto. |
| Instantánea preparada | `EngineProject` | Forma interna de una instantánea del motor, lista para el hilo de audio: con el audio ya en memoria y los valores ya calculados (06 §5.3). |
| Kit | `Kit` | Conjunto de 8 sonidos de percusión, uno por fila, siempre en el mismo orden de funciones (05). |
| Línea de tiempo | `timeline` | Zona de la pestaña Canción que representa el tiempo de izquierda a derecha, con la regla arriba y los carriles debajo. |
| Medidor | `meter` | Indicador del nivel de pico de una señal, en dBFS. |
| Microfundido | `MICROFADE_FRAMES` | Fundido lineal de 5 ms que el motor aplica solo en cada borde audible de un clip para que los cortes no hagan clic. |
| Modo canción | `playMode: "song"` | Reproducción de la canción desde el cursor. Se usa al pulsar Reproducir en las pestañas Canción o Mezcla. |
| Modo patrón | `playMode: "pattern"` | Reproducción en bucle del patrón seleccionado. Se usa al pulsar Reproducir en la pestaña Ritmo. |
| Motor de audio (motor) | `engine` | Biblioteca en Rust que genera, graba y exporta el sonido. No decide nada musical: reproduce la instantánea que recibe. |
| Negra, corchea, semicorchea | — | Figuras musicales. Una negra son 960 ticks; una corchea, 480; una semicorchea, 240. |
| Nivel (de un golpe) | `level` | Intensidad de un golpe: 0 apagado, 1 suave, 2 normal, 3 fuerte. En la 1.0 solo se usan 0 y 2. |
| Nivel de salto | `jump_level` | Ganancia con la que el motor apaga y recupera el sonido de las pistas de audio alrededor de un salto de posición o de un cambio del audio que suena (06 §6.5). |
| Panorama | `pan` | Posición de una pista entre el altavoz izquierdo (−100) y el derecho (+100). |
| Pantalla de inicio | `StartScreen` | Cuadro que aparece al arrancar con Nuevo proyecto, Abrir proyecto y la lista de recientes. |
| Paso | `step` | Casilla de la rejilla. Equivale a una semicorchea. |
| Paso de historial | `HistoryEntry` | Cambio del proyecto que se puede deshacer y rehacer como una unidad. |
| Patrón | `Pattern` | Rejilla de 8 filas por un número de pasos que define un ritmo de 1, 2 o 4 compases. Pertenece a una pista de ritmo. |
| Pestaña | `tab` | Pantalla completa dedicada a una herramienta. La 1.0 tiene tres: Ritmo, Canción y Mezcla. |
| Pista | `Track` | Elemento de la canción que produce sonido por un canal de mezcla propio. Hay dos tipos. |
| Pista armada | `armedTrackId` | Pista de audio preparada para recibir la próxima grabación. Como mucho hay una. |
| Pista de audio | `AudioTrack` | Pista cuyo carril contiene clips de audio. |
| Pista de ritmo | `RhythmTrack` | Pista con un kit, una lista de patrones y un carril de bloques de patrón. |
| Programa nativo | `src-tauri` | Parte de la aplicación escrita en Rust que crea la ventana e implementa los comandos del puente: archivos, ajustes, registro y acceso al motor (04 §4). |
| Proyecto | `Project` | Todo lo que se guarda: tempo, compás, pistas, patrones, bloques, clips, mezcla y estado de vista. |
| Puente | `bridge` | Conjunto de comandos y eventos de Tauri por los que la interfaz habla con el motor y con el sistema de archivos (04). |
| Puente simulado | `mockBridge` | Implementación del puente en memoria, sin Tauri ni motor real, con la que se prueba toda la interfaz (08 §5.2). |
| Pulso | `beat` | Cada una de las partes iguales del compás que marca la claqueta: una negra en 4/4 y 3/4, una corchea en 6/8. |
| Ranura | `slot` | Cada una de las 16 posiciones fijas en las que el motor guarda el estado de una pista entre una instantánea y la siguiente (06 §5.4). |
| Región (de un clip) | `regionStartFrame`, `regionLengthFrames` | Tramo del archivo de audio que un clip hace sonar. |
| Regla | `ruler` | Franja superior de la línea de tiempo con los números de compás. |
| Regla de tapado | `z`, `resolveLane` | Cuando dos elementos de un mismo carril se solapan, en el solape solo suena y solo se ve el que está encima, que es el último creado, movido o redimensionado. El de debajo queda intacto. |
| Reloj del motor | — | Número total de frames que el motor ha calculado desde que arrancó. Sirve para relacionar la entrada con la salida al grabar (06 §6.2). |
| Repetir (un clip) | `repeat` | Estado de un clip en el que alargarlo por la derecha repite su región en lugar de descubrir más audio. |
| Retardo de ida y vuelta (latencia) | `latencyFrames` | Tiempo entre que el motor emite un sonido y lo recibe grabado por la entrada. Se usa para colocar cada grabación en su sitio. |
| Rejilla de pasos | `StepGrid` | Cuadrícula de filas y pasos con la que se edita un patrón. |
| Reverb | `reverb` | Efecto que añade sensación de sala. |
| Ritmo hecho | `RhythmPreset` | Contenido de patrón ya preparado para un estilo, que el usuario puede cargar en un patrón. |
| Saturación | `overload` | Situación en la que una señal supera 0 dBFS. En código se llama *overload* para no confundirla con *clip* (clip de audio). |
| Segmento | `Segment` | Tramo de un carril ya resuelto con la regla de tapado: no se solapa con ningún otro del mismo carril. La instantánea del motor solo contiene segmentos. |
| Silenciar | `mute` | Hacer que una pista no suene. |
| Solo | `solo` | Hacer que solo suenen las pistas que lo tienen activado. |
| Sonido (de kit) | `KitSound` | Muestra de percusión de un kit: un archivo WAV mono corto que se dispara entero. |
| Subbloque | — | Tramo de un bloque de audio en el que la posición avanza sin saltos y nada cambia de estado (06 §6.4). |
| Tarea | `T-nnn` | Unidad de trabajo del plan de implementación, pensada para una sesión de un agente (09). |
| Telemetría | `telemetry` | Datos que el motor envía periódicamente a la interfaz: posición, niveles y cortes. |
| Tempo | `tempoBpm` | Velocidad de la canción en negras por minuto, sea cual sea el compás. |
| Tick | `tick` | Unidad de tiempo musical: 1/960 de negra. |
| Toma | `take` | Archivo de audio creado por una grabación. |
| Trazabilidad | — | Correspondencia entre cada criterio de aceptación y la prueba que lo comprueba (08 §13). |
| Xrun (corte de audio) | `xrun` | Bloque de sonido que el motor no ha podido entregar a tiempo y que se oye como un corte. |

## 8. Registro de decisiones

Estas decisiones están tomadas. No las reabras: si una tarea parece exigir lo contrario, relee la tarea. La columna «Origen» dice quién la tomó: **U**, el autor del producto; **E**, quien redactó esta especificación para completar lo que el autor delegó.

| Id | Origen | Decisión | Por qué |
|---|---|---|---|
| DEC-01 | U | El programa se llama Carabo y es un DAW de escritorio cuya virtud principal es resultar intuitivo. | Es la razón de ser del producto: todo lo demás se subordina a ello. |
| DEC-02 | U | Es una herramienta para su autor, que toca la guitarra y canta, con experiencia básica en FL Studio. | Permite recortar el alcance a lo que un músico aficionado necesita. |
| DEC-03 | U | Uso principal: montar una base rítmica y grabar audio encima. Sirve para música de cualquier estilo. | Es el núcleo más pequeño con el que el programa ya resulta útil. |
| DEC-04 | U | La base rítmica se hace con patrones de pasos, no con loops de audio ajustados al tempo. | Un patrón sigue cualquier tempo sin tratamiento del audio y es el primer paso hacia los instrumentos virtuales. |
| DEC-05 | U | Los patrones son reutilizables: se crean en la pestaña Ritmo y se colocan como bloques en la pestaña Canción. No hay interruptor patrón/canción: la pestaña activa decide qué suena. Al crear una pista de ritmo, su primer patrón se coloca solo en la canción. | Es el modelo de FL Studio, que el autor conoce, sin sus dos confusiones clásicas. |
| DEC-06 | U | «Intuitivo» significa superar la prueba de los cinco minutos (PA-01). | Convierte una cualidad vaga en un criterio comprobable. |
| DEC-07 | U | Una pestaña a pantalla completa por herramienta: Ritmo, Canción y Mezcla en la 1.0; Afinador en la 1.1. Cada herramienta nueva entra como pestaña nueva. | Cada pantalla tiene pocos controles y añadir funciones no complica las existentes. |
| DEC-08 | U | Al arrancar se muestra la pantalla de inicio y después un proyecto vacío, sin contenido precargado, asistentes ni guías. | El autor lo considera innecesario. |
| DEC-09 | U | Se usa el vocabulario estándar de producción musical. Cada control lleva su nombre escrito y una explicación breve al pasar el ratón. | El autor prefiere los términos correctos; el texto visible evita adivinar iconos. |
| DEC-10 | U | Una sola manera de hacer cada cosa y casi ninguna preferencia. | Cada opción multiplica lo que hay que aprender y probar. |
| DEC-11 | U | Los mensajes de error dicen qué pasa y qué hacer. | Un error sin salida es lo que más frena a un usuario nuevo. |
| DEC-12 | U | El guardado es manual. El programa escribe aparte una copia de recuperación. Todo cambio del proyecto se puede deshacer y rehacer en varios pasos. La edición es no destructiva. | El autor quiere decidir cuándo se guarda; la copia y el historial evitan perder trabajo. |
| DEC-13 | U | Un tempo y un compás por proyecto: 4/4, 3/4 o 6/8. | Los cambios a mitad de canción complican el motor y quedan para más adelante. |
| DEC-14 | U | Al grabar sobre algo ya grabado, la toma nueva tapa a la anterior, que queda intacta debajo. | Es simple y no destruye nada. |
| DEC-15 | U | El alcance se reparte en 1.0, 1.1 y posteriores según 01 §6. | Cada versión debe servir por sí sola. |
| DEC-16 | U | No se alojan plugins de terceros (VST3, CLAP, AU). | Es de lo más caro de un DAW y el autor no lo necesita. |
| DEC-17 | U | El programa no deja oír la entrada mientras se graba. El retardo se compensa al colocar la grabación, con una calibración previa. | Evita la exigencia de latencia muy baja. |
| DEC-18 | U | 48 000 Hz fijos, 16 pistas como máximo y canciones de hasta 10 minutos. | Una sola configuración que construir y probar. |
| DEC-19 | U | El equipo de referencia es un micrófono USB y unos auriculares, con el sonido estándar de Windows y sin ASIO. | Es el equipo del autor. |
| DEC-20 | U | Solo Windows 11 de escritorio. Se distribuye como el ejecutable que resulta de compilar, sin instalador ni actualizaciones automáticas. | Es una herramienta personal. |
| DEC-21 | U | El proyecto es una carpeta con un archivo JSON legible y una subcarpeta de audio. | Se puede inspeccionar y depurar, y guardar es rápido. |
| DEC-22 | U | La interfaz está en español y en inglés, a elegir en Ajustes. | Decisión del autor; es una de las pocas preferencias admitidas. |
| DEC-23 | U | Interfaz web dentro de Tauri 2 y motor de audio nativo en Rust. | Deja abierta la puerta a baja latencia y plugins sin rehacer el motor. |
| DEC-24 | U | Lo programan agentes de Claude Code; el autor revisa, depura y aprueba. | — |
| DEC-25 | U | Código abierto con licencia MIT. | — |
| DEC-26 | E | Todo el estado del proyecto, incluido el historial de deshacer, vive en la interfaz (TypeScript). El motor solo recibe instantáneas ya resueltas y las reproduce. | Mantiene el código Rust pequeño y predecible, que es donde más cuesta programar y revisar. |
| DEC-27 | E | El motor es una biblioteca independiente de la interfaz, capaz de generar el audio a un archivo sin dispositivo de sonido. | Permite probarlo de forma automática y es la base de la exportación. |
| DEC-28 | E | Cada acción está en un único lugar: las pistas de ritmo se crean en Ritmo, las de audio en Canción, y volumen, panorama, silenciar y solo solo están en Mezcla. | Aplicación estricta de DEC-10. |
| DEC-29 | E | Solo se puede seleccionar un elemento a la vez. | La selección múltiple multiplica los casos de cada operación de edición. |
| DEC-30 | E | Atajos de teclado fijos y un único tema visual oscuro. | DEC-10. |
| DEC-31 | E | Se importa WAV y MP3. Se exporta WAV (48 000 Hz, 24 bits) y MP3 (320 kbps), sin más opciones. | DEC-10. |
| DEC-32 | E | La copia de recuperación se escribe cada 120 segundos si hay cambios y justo al terminar cada grabación. | Una toma no se puede repetir igual. |
| DEC-33 | E | Los sonidos de los kits son de dominio público (CC0). Mientras no estén elegidos, el programa usa sonidos sustitutos sintetizados por un script del repositorio. | Evita bloquear el desarrollo y cualquier problema de licencias. |
| DEC-34 | E | Control de versiones con Git en GitHub; las pruebas se ejecutan solas en cada subida. | — |
| DEC-35 | E | Documentos en español; código en inglés. | Los agentes trabajan mejor con identificadores en inglés. |
| DEC-36 | E | El tempo se expresa siempre en negras por minuto, también en 6/8. | Una sola fórmula de conversión entre ticks y tiempo. |
| DEC-37 | E | Al parar, el cursor vuelve a la posición desde la que se empezó a reproducir. La reproducción no se para sola al terminar la canción. | Facilita repetir una toma y elimina casos especiales. |
| DEC-38 | E | Una pista de audio nueva queda armada sola. El micrófono solo se abre mientras hay una pista armada, mientras la ventana de ajustes está abierta (medidor y calibración) o mientras la pestaña Afinador está activa. | Menos pasos para grabar (PD-04) sin tener el micrófono siempre abierto. |
| DEC-39 | E | Recortar un clip descubre u oculta audio del archivo. Para repetir su contenido hay que activar «Repetir» en ese clip. | Evita que un mismo gesto haga dos cosas distintas. |
| DEC-40 | E | Los efectos de cada pista son fijos y van en un orden fijo; no hay cadena de efectos que ordenar. | DEC-10. |
| DEC-41 | E | En 6/8 la claqueta marca las seis corcheas. | Regla única: la claqueta marca cada pulso (glosario). |
| DEC-42 | E | El motor carga entero en memoria el audio que suena; no lee del disco mientras reproduce. | Es mucho más simple de programar bien y hace que lo exportado sea idéntico a lo que se oye. El coste es memoria (RNF-07). |
| DEC-43 | E | Un archivo de audio que el motor no consigue cargar se trata igual que uno que falta. | Un solo comportamiento que explicar y que probar (RF-028). |
| DEC-44 | E | El MP3 se codifica con LAME, de licencia LGPL. Es la única excepción a las licencias permisivas y queda aislada tras una interfaz propia. | Es el codificador de referencia, y el código de Carabo es público, así que cualquiera puede recompilarlo (07 §11.3). |
| DEC-45 | E | Los arrastres dentro de la ventana se hacen con eventos de puntero, no con la función de arrastrar y soltar del navegador. | Esa función deja de servir en Windows al activar la recepción de archivos del Explorador (07 §6.4). |
| DEC-46 | E | De cada dependencia se usa la última versión estable de una línea fija, anotada en los archivos de bloqueo. No se cambia de línea. | Compilaciones repetibles, sin perseguir novedades (07 §2.1). |
| DEC-47 | E | Cada tarea se hace en una rama propia y termina en una solicitud de integración que revisa y fusiona el autor. | DEC-24. |
| DEC-48 | E | Toda prueba que cubre un criterio de aceptación lleva su identificador en el nombre, y un script comprueba que no queda ninguno sin cubrir. | Hace comprobable RNF-21 sin tablas mantenidas a mano (08 §13). |
