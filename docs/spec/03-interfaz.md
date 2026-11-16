# Carabo · Especificación · 03 — Interfaz

> **PROMPT PARA EL AGENTE — léelo antes de seguir**
>
> Eres un agente de programación que trabaja en el repositorio de **Carabo**, un DAW de escritorio para Windows 11. Este documento define **cómo se ve y cómo se maneja el programa**: aspecto, disposición de cada pantalla, comportamiento de cada control con ratón y con teclado, cuadros de diálogo, mensajes y todos los textos en español y en inglés.
>
> - Léelo en toda tarea de interfaz. Lee siempre §1 a §4 (reglas, aspecto, estructura y controles comunes) y, además, el capítulo de la pantalla que tu tarea toca.
> - Las medidas, los colores, los textos y las claves de traducción son **exactos**: cópialos. No inventes textos; si falta uno, aplica 00 §6.
> - Este documento manda sobre los demás en cuanto a textos, disposición y atajos (00 §5). **Qué** hace cada control está en 02 (se cita cada requisito); aquí se describe **cómo** se presenta y se maneja.
> - Ningún texto visible se escribe en el código: todos van en los archivos de traducción con las claves de este documento (§14).
> - Los términos (bloque, clip, cursor, tapar, gesto continuo…) están en el glosario (00 §7). Los nombres en `MAYÚSCULAS` son constantes de 00 §4.

---

## 1. Reglas generales

Estas reglas salen de los principios de diseño (01 §4) y valen para todas las pantallas.

1. **Una pestaña a la vez, a pantalla completa** (PD-02). No hay ventanas flotantes, paneles acoplables ni vistas divididas.
2. **Todo control lleva su nombre escrito** (PD-07). No hay controles identificados solo por un icono. Se consideran controles los botones, deslizadores, campos, listas desplegables y menús. Los pasos de la rejilla, los bloques y los clips son elementos de contenido: no llevan texto de ayuda, pero sí nombre accesible (§15).
3. **Todo control tiene una explicación** de una frase (RF-008), que aparece a los 500 ms de dejar el ratón encima o de recibir el foco. Si el control está desactivado, la explicación dice por qué.
4. **Un botón de dos estados activado se distingue por algo más que el color**: fondo de acento y una marca `✓` delante del texto.
5. **Cada acción tiene un único control visible** (PD-05). La tabla de §12 es la lista completa.
6. **Las acciones que se pueden deshacer no piden confirmación** (PD-08).
7. **Un control desactivado se ve atenuado, no desaparece.** Así su explicación puede decir por qué.
8. **Durante la grabación** solo están activos «Parar» y las pestañas (RF-312). Todo lo demás muestra la explicación `common.disabledWhileRecording`.

## 2. Aspecto

Hay un único tema visual, oscuro (DEC-30). Los valores se definen una sola vez como variables CSS (ruta en 07) y los componentes usan las variables, nunca los valores.

### 2.1 Colores

| Variable | Valor | Uso |
|---|---|---|
| `--bg-app` | `#1b1d21` | Fondo de la ventana y de la zona de pestaña |
| `--bg-panel` | `#24272c` | Barras, cabeceras, canales de mezcla y cuadros de diálogo |
| `--bg-raised` | `#2e3238` | Botones y elementos elevados |
| `--bg-input` | `#15171a` | Campos, pasos vacíos y fondo de carriles |
| `--border` | `#4a505a` | Bordes y separadores |
| `--grid-line` | `#33383f` | Líneas de pulso en carriles |
| `--grid-line-strong` | `#4a505a` | Líneas de compás |
| `--text` | `#e8eaed` | Texto principal |
| `--text-dim` | `#a6abb3` | Texto secundario y controles desactivados |
| `--accent` | `#4da3ff` | Botón de dos estados activado, cursor de reproducción, tramo de bucle |
| `--accent-text` | `#0b1420` | Texto sobre `--accent` |
| `--record` | `#ff5a5a` | Grabar activo, bloque de grabación, indicador de saturación |
| `--ok` | `#4cc38a` | Tramo normal de un medidor |
| `--warn` | `#f2b94b` | Tramo alto de un medidor, avisos |
| `--focus` | `#ffd166` | Contorno del foco del teclado |
| `--overlay` | `rgba(0, 0, 0, 0.55)` | Velo detrás de un cuadro de diálogo |

Paleta de pistas, por `colorIndex` (05 §3.1). El texto sobre estos colores es `#101214`.

| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|---|
| `#5b8def` | `#e0719e` | `#58c4a7` | `#e0a84f` | `#a682e6` | `#6ec3e8` | `#d98666` | `#9bc45a` |

El color de una pista tiñe sus bloques, sus clips, los golpes de sus patrones y una franja de 4 píxeles en su cabecera y en su canal de mezcla. Nunca es la única forma de distinguir nada (RNF-16).

### 2.2 Tipografía y medidas

| Elemento | Valor |
|---|---|
| Fuente | `"Segoe UI", system-ui, sans-serif` |
| Fuente de números de posición | `"Cascadia Mono", Consolas, monospace` |
| Tamaño base | 13 px |
| Texto pequeño (valores, tipo de pista) | 12 px |
| Títulos de cuadro de diálogo | 16 px, seminegrita |
| Unidad de espaciado | 4 px; los márgenes son múltiplos de ella |
| Altura de un control | 28 px |
| Radio de las esquinas | 4 px |
| Contorno del foco | 2 px, `--focus`, separado 1 px del control |

## 3. Estructura de la ventana

```text
┌──────────────────────────────────────────────────────────────────────────────────┐
│ Barra superior (40 px)                                                           │
│ [Archivo ▾] [Deshacer] [Rehacer]      [ Ritmo ] [ Canción ] [ Mezcla ]  [Ajustes]│
├──────────────────────────────────────────────────────────────────────────────────┤
│ Barra de reproducción (48 px)                                                    │
│ [Reproducir] [Grabar]  Parado   1.1  0:00.0   Tempo [120][−][+]  Compás [4/4 ▾]  │
│                                                          [Claqueta] [Bucle]      │
├──────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│ Zona de pestaña (todo el espacio restante): contenido de la pestaña activa       │
│                                                                                  │
├──────────────────────────────────────────────────────────────────────────────────┤
│ Barra de estado (24 px)   mensaje…                               Audio: correcto │
└──────────────────────────────────────────────────────────────────────────────────┘
```

- La ventana mide como mínimo 1280 × 720 (RF-001). Las tres barras tienen altura fija y ocupan todo el ancho.
- El orden del foco con `Tab` es: barra superior, barra de reproducción, zona de pestaña y barra de estado, cada una de izquierda a derecha y de arriba abajo.
- Cada pestaña tiene, arriba, su propia barra de herramientas de 44 px.

## 4. Controles comunes

Todos los controles de la aplicación son uno de estos. Se implementan una vez (`ui/common/`, 04) y se reutilizan.

| Control | Aspecto | Ratón | Teclado |
|---|---|---|---|
| **Botón** | Texto sobre `--bg-raised`, borde `--border` | Clic: ejecuta | `Intro` o `Espacio` |
| **Botón de dos estados** | Como el botón; activado: fondo `--accent`, texto `--accent-text` y `✓` delante | Clic: alterna | `Intro` o `Espacio` |
| **Deslizador** | Carril con tirador; a su lado, el valor en texto | Arrastrar el tirador o pulsar en el carril: gesto continuo (RF-032) | `←` `↓` bajan un paso; `→` `↑` suben un paso; `RePág` y `AvPág`, 10 pasos; `Inicio` y `Fin`, mínimo y máximo. Cada pulsación es un paso de historial |
| **Campo numérico** | Caja de texto con el valor | Clic: edita | `Intro` confirma; `Esc` restaura; al perder el foco, confirma |
| **Lista desplegable** | Valor actual y `▾` | Clic: abre la lista; clic en una opción: elige | `Intro` o `Espacio` abren; `↑` `↓` recorren; `Intro` elige; `Esc` cierra sin cambiar |
| **Menú** | Botón con `▾` que abre una lista de órdenes | Clic en una orden: la ejecuta | Igual que la lista desplegable |
| **Selector de opciones** | Varios botones contiguos de los que uno está activado (con `✓`) | Clic: elige | `←` `→` cambian la opción |
| **Explicación** | Recuadro `--bg-raised` con texto de 12 px, bajo el control | Aparece a los 500 ms; desaparece al salir o al pulsar | Aparece al recibir el foco |
| **Medidor** | Barra vertical de 8 px de ancho por canal; de −60 a −12 dBFS en `--ok`, de −12 a 0 en `--warn`, por encima de 0 en `--record`; una raya marca el pico retenido 1 segundo | — | — |
| **Indicador de saturación** | Texto `SAT` (`CLIP` en inglés) sobre `--record` cuando está encendido; atenuado cuando no | Clic: lo apaga | `Intro` o `Espacio` |
| **Cuadro de diálogo** | Panel centrado sobre un velo `--overlay`; título, contenido y botones abajo a la derecha, con el principal a la derecha del todo | — | El foco no sale del cuadro; `Esc` equivale al botón «Cancelar» o «Cerrar»; `Intro` equivale al botón principal |

Reglas comunes de arrastre (valen para deslizadores, bloques, clips, bordes y la franja de bucle):

1. Un arrastre empieza cuando el ratón se ha movido 3 píxeles o más con el botón pulsado. Menos que eso es un clic.
2. Mientras dura, el control sigue al ratón aunque salga de él o de la ventana.
3. `Esc` cancela y restaura (RF-032).
4. No hay desplazamiento automático de la vista durante un arrastre.

## 5. Barra superior

| Clave | Etiqueta ES | Etiqueta EN | Explicación ES | Explicación EN |
|---|---|---|---|---|
| `topbar.file` | Archivo | File | Proyecto: nuevo, abrir, guardar y exportar. | Project: new, open, save and export. |
| `topbar.undo` | Deshacer | Undo | Deshacer: {step} (Ctrl+Z). | Undo: {step} (Ctrl+Z). |
| `topbar.redo` | Rehacer | Redo | Rehacer: {step} (Ctrl+Y). | Redo: {step} (Ctrl+Y). |
| `tab.rhythm` | Ritmo | Rhythm | Crear y editar patrones de ritmo (Ctrl+1). | Create and edit rhythm patterns (Ctrl+1). |
| `tab.song` | Canción | Song | Colocar patrones y grabar audio a lo largo del tiempo (Ctrl+2). | Arrange patterns and record audio over time (Ctrl+2). |
| `tab.mix` | Mezcla | Mix | Ajustar el volumen, el panorama y los efectos de cada pista (Ctrl+3). | Set the volume, pan and effects of each track (Ctrl+3). |
| `topbar.settings` | Ajustes | Settings | Idioma, dispositivos de audio y retardo de grabación. | Language, audio devices and recording latency. |

- `{step}` es el nombre del paso de historial (§14.3). Sin nada que deshacer o rehacer, el botón está desactivado y su explicación es `topbar.undo.none` («No hay nada que deshacer.» / «Nothing to undo.») o `topbar.redo.none` («No hay nada que rehacer.» / «Nothing to redo.»).
- Las pestañas forman un grupo: la activa se muestra con fondo `--bg-app` y una línea inferior `--accent` de 2 px, y además con el texto en seminegrita.

Menú «Archivo» (RF-020 a RF-024, RF-600):

| Clave | ES | EN | Atajo | Desactivada cuando |
|---|---|---|---|---|
| `menu.file.new` | Nuevo proyecto | New project | Ctrl+N | Se graba |
| `menu.file.open` | Abrir proyecto… | Open project… | Ctrl+O | Se graba |
| `menu.file.recent` | Abrir reciente | Open recent | — | Se graba. Abre un submenú con los recientes (nombre y ruta); vacío muestra `menu.file.recent.empty`: «No hay proyectos recientes» / «No recent projects» |
| `menu.file.save` | Guardar | Save | Ctrl+S | Se graba |
| `menu.file.saveAs` | Guardar como… | Save as… | Ctrl+Mayús+S | Se graba |
| `menu.file.export` | Exportar audio… | Export audio… | Ctrl+E | Se graba |
| `menu.file.quit` | Salir | Quit | Alt+F4 | Nunca |

El atajo se muestra a la derecha de cada orden.

## 6. Barra de reproducción

Es idéntica en todas las pestañas (PD-03, RF-100). De izquierda a derecha:

| Clave | Etiqueta ES | Etiqueta EN | Control | Explicación ES | Explicación EN |
|---|---|---|---|---|---|
| `transport.play` | Reproducir | Play | Botón (96 px) | Reproducir lo que corresponde a la pestaña activa: el patrón en Ritmo, la canción en Canción y Mezcla (Espacio). | Play what matches the active tab: the pattern in Rhythm, the song in Song and Mix (Space). |
| `transport.stop` | Parar | Stop | El mismo botón, mientras suena | Parar y devolver el cursor al punto de partida (Espacio). | Stop and return the playhead to where it started (Space). |
| `transport.record` | Grabar | Record | Botón (80 px); mientras graba, fondo `--record` | Grabar en la pista armada desde el cursor, tras un compás de cuenta atrás (R). | Record on the armed track from the playhead, after a one-bar count-in (R). |
| `transport.status` | — | — | Texto (240 px) | — | — |
| `transport.position` | — | — | Dos textos en fuente de números: `compás.pulso` y `min:seg.décimas` | Posición: compás y pulso, y minutos y segundos. | Position: bar and beat, and minutes and seconds. |
| `transport.tempo` | Tempo | Tempo | Campo numérico (56 px) con botones `−` y `+` | Velocidad de la canción en negras por minuto (40 a 240). | Song speed in quarter notes per minute (40 to 240). |
| `transport.timeSignature` | Compás | Time signature | Lista desplegable: `4/4`, `3/4`, `6/8` | Indicación de compás del proyecto. | The project's time signature. |
| `transport.metronome` | Claqueta | Metronome | Botón de dos estados | Clic en cada pulso al reproducir y al grabar. No se exporta. | A click on every beat while playing and recording. It is never exported. |
| `transport.loop` | Bucle | Loop | Botón de dos estados | Repetir el tramo marcado en la regla de la pestaña Canción. | Repeat the range marked on the ruler of the Song tab. |

Textos de estado (`transport.status`, RF-103):

| Clave | ES | EN |
|---|---|---|
| `transport.status.stopped` | Parado | Stopped |
| `transport.status.pattern` | Suena: patrón «{name}» | Playing: pattern “{name}” |
| `transport.status.song` | Suena: canción | Playing: song |
| `transport.status.countIn` | Cuenta atrás | Count-in |
| `transport.status.recording` | Grabando | Recording |

Explicaciones cuando un control está desactivado:

| Clave | Cuándo | ES | EN |
|---|---|---|---|
| `transport.play.noPattern` | Pestaña Ritmo sin pistas de ritmo | Crea primero una pista de ritmo. | Create a rhythm track first. |
| `transport.record.inRhythm` | Pestaña Ritmo | Para grabar, ve a la pestaña Canción. | To record, go to the Song tab. |
| `transport.record.whilePlaying` | Se está reproduciendo | Para la reproducción antes de grabar. | Stop playback before recording. |
| `transport.timeSignature.whilePlaying` | Se reproduce | Para la reproducción para cambiar el compás. | Stop playback to change the time signature. |
| `common.disabledWhileRecording` | Se graba (cualquier control) | No disponible durante la grabación. | Not available while recording. |

Comportamiento:

- Los botones `−` y `+` tienen los nombres accesibles `transport.tempo.down` («Bajar el tempo» / «Decrease tempo») y `transport.tempo.up` («Subir el tempo» / «Increase tempo»).
- Al confirmar un tempo fuera de rango se ajusta al límite y se muestra el aviso E-015.
- El aviso E-160 (RF-107) y el aviso E-161 (RF-109) se muestran antes de aplicar el cambio.
- Durante la cuenta atrás, la posición muestra el punto de inicio sin moverse.

## 7. Barra de estado

- **Izquierda**: el último aviso o mensaje informativo (§13), durante 5 segundos (RF-006). Un mensaje nuevo sustituye al anterior. Los avisos van precedidos de «⚠ » y los informativos no llevan prefijo.
- **Derecha**: el indicador de estado del audio (RF-508):

| Clave | ES | EN |
|---|---|---|
| `audio.status.ok` | Audio: correcto | Audio: OK |
| `audio.status.noOutput` | Audio: sin salida | Audio: no output |
| `audio.status.xruns` | Audio: {n} cortes | Audio: {n} dropouts |
| `audio.status.help` | Un corte es un instante en el que el ordenador no ha podido calcular el sonido a tiempo. Si hay muchos, cierra otros programas. | A dropout is a moment when the computer could not compute the sound in time. If there are many, close other programs. |

## 8. Pestaña Ritmo

### 8.1 Disposición

```text
┌ Barra de herramientas (44 px) ───────────────────────────────────────────────────┐
│ [Nueva pista de ritmo]  Pista [Ritmo 1 ▾]  Kit [Batería acústica ▾]              │
│ [Cargar ritmo hecho…] [Borrar golpes]  Compases [✓1][2][4]                        │
├ Patrones (220 px) ──┬ Rejilla ───────────────────────────────────────────────────┤
│ Patrones            │                         1       2       3       4          │
│ [Nuevo patrón]      │ [Bombo  ] [Silenciar] ──○── 0 dB  ■□□□ □□□□ ■□□□ □□□□       │
│ [Duplicar]          │ [Caja   ] [Silenciar] ──○── 0 dB  □□□□ ■□□□ □□□□ ■□□□       │
│ [Renombrar]         │ [Aro    ] [Silenciar] ──○── 0 dB  □□□□ □□□□ □□□□ □□□□       │
│ [Eliminar]          │  … 8 filas …                                               │
│ ─────────────       │                                                            │
│ ▸ Patrón 1          │                                                            │
│   Patrón 2          │                                                            │
└─────────────────────┴────────────────────────────────────────────────────────────┘
```

Medidas de la rejilla:

| Elemento | Medida |
|---|---|
| Cabecera de fila | 360 px de ancho: botón con el nombre del sonido (112 px), «Silenciar» (88 px), deslizador de volumen (96 px) y su valor (48 px) |
| Altura de fila | 36 px |
| Paso | 28 × 28 px, con 2 px de separación |
| Separación entre pulsos | 6 px más |
| Separación entre compases | 12 px más, con una línea vertical `--grid-line-strong` |
| Línea de números | 20 px sobre la rejilla: el número de cada pulso sobre su primer paso; en el primer pulso de cada compás, el número va en seminegrita |

Un pulso son 4 pasos en 4/4 y en 3/4, y 2 pasos en 6/8 (05 §2.1). Si los pasos no caben, la zona de pasos se desplaza en horizontal; las cabeceras de fila permanecen fijas (RF-204).

### 8.2 Controles

| Clave | Etiqueta ES | Etiqueta EN | Control | Explicación ES | Explicación EN | Req. |
|---|---|---|---|---|---|---|
| `rhythm.newTrack` | Nueva pista de ritmo | New rhythm track | Botón | Crear una pista de ritmo con un kit y un patrón vacío, ya colocado en la canción. | Create a rhythm track with a kit and an empty pattern, already placed in the song. | RF-201 |
| `rhythm.track` | Pista | Track | Lista desplegable | Pista de ritmo que se edita. | The rhythm track being edited. | RF-202 |
| `rhythm.kit` | Kit | Kit | Lista desplegable | Conjunto de sonidos de la pista. | The track's set of sounds. | RF-203 |
| `rhythm.loadPreset` | Cargar ritmo hecho… | Load ready-made rhythm… | Botón | Sustituir el contenido del patrón por un ritmo ya hecho. | Replace the pattern's content with a ready-made rhythm. | RF-209 |
| `rhythm.clear` | Borrar golpes | Clear hits | Botón | Quitar todos los golpes del patrón. | Remove all hits from the pattern. | RF-212 |
| `rhythm.bars` | Compases | Bars | Selector de opciones `1`, `2`, `4` | Longitud del patrón en compases. | Pattern length in bars. | RF-207 |
| `rhythm.patterns` | Patrones | Patterns | Título de la lista | — | — | RF-208 |
| `rhythm.newPattern` | Nuevo patrón | New pattern | Botón | Crear un patrón vacío de un compás. | Create an empty one-bar pattern. | RF-208 |
| `rhythm.duplicatePattern` | Duplicar | Duplicate | Botón | Crear una copia del patrón seleccionado. | Create a copy of the selected pattern. | RF-208 |
| `rhythm.renamePattern` | Renombrar | Rename | Botón | Cambiar el nombre del patrón seleccionado (F2). | Rename the selected pattern (F2). | RF-208 |
| `rhythm.deletePattern` | Eliminar | Delete | Botón | Eliminar el patrón seleccionado y sus bloques de la canción. | Delete the selected pattern and its blocks in the song. | RF-208 |
| `rhythm.rowListen` | (nombre del sonido) | (sound name) | Botón | Escuchar este sonido. | Listen to this sound. | RF-206 |
| `rhythm.rowMute` | Silenciar | Mute | Botón de dos estados | Silenciar esta fila en todos los patrones de la pista. | Mute this row in all the track's patterns. | RF-210 |
| `rhythm.rowGain` | Volumen | Volume | Deslizador, −48 a +6 dB, paso 1 | Volumen de esta fila. | Volume of this row. | RF-210 |

La etiqueta «Volumen» de cada fila no se dibuja (no cabe): es su nombre accesible. El valor se muestra como `{n} dB`.

Explicaciones cuando un control está desactivado:

| Clave | Cuándo | ES | EN |
|---|---|---|---|
| `rhythm.disabled.noTrack` | No hay pistas de ritmo (todos menos «Nueva pista de ritmo») | Crea primero una pista de ritmo. | Create a rhythm track first. |
| `common.maxTracks` | Hay `MAX_TRACKS` pistas | Se ha alcanzado el máximo de 16 pistas. | The maximum of 16 tracks has been reached. |
| `rhythm.newPattern.max` | La pista tiene `MAX_PATTERNS_PER_TRACK` patrones | Esta pista ya tiene 32 patrones. | This track already has 32 patterns. |
| `rhythm.deletePattern.last` | La pista solo tiene un patrón | Una pista de ritmo necesita al menos un patrón. | A rhythm track needs at least one pattern. |
| `rhythm.clear.empty` | El patrón no tiene golpes | El patrón ya está vacío. | The pattern is already empty. |

### 8.3 Lista de patrones

- Cada patrón es una línea con su nombre. El seleccionado lleva fondo `--bg-raised`, una marca `▸` delante y el texto en seminegrita.
- Un clic en una línea selecciona ese patrón. Con el foco en la lista, `↑` y `↓` cambian la selección.
- «Renombrar» convierte la línea seleccionada en un campo de texto con el nombre actual seleccionado. `Intro` o perder el foco confirman; `Esc` cancela; un nombre vacío o de solo espacios restaura el anterior. Se recortan los espacios de los extremos y se limita a `NAME_MAX_CHARS` caracteres.

### 8.4 Rejilla de pasos

Aspecto de un paso:

| Estado | Aspecto |
|---|---|
| Vacío | Fondo `--bg-input`, borde `--border` |
| Con golpe (nivel 2) | Fondo del color de la pista y un cuadrado interior de 12 px `#101214` centrado |
| En la columna que suena (modo patrón) | Además, borde superior de 3 px `--accent` |
| Con el foco | Además, contorno de foco |
| Fila silenciada | Toda la fila de pasos al 40 % de opacidad; los golpes siguen visibles |

Manejo:

1. Un clic en un paso lo alterna (RF-205). El cambio ocurre al pulsar el botón del ratón, no al soltarlo.
2. La rejilla es una sola parada de `Tab`. Dentro, `←` `→` `↑` `↓` mueven el foco de paso en paso; `Inicio` y `Fin` van al primer y al último paso de la fila; `Espacio` o `Intro` alternan el paso enfocado.
3. Mientras el foco está en la rejilla, `Espacio` no inicia la reproducción (RF-205 CA2).
4. No se pueden marcar varios pasos arrastrando.

### 8.5 Cuadro «Cargar ritmo hecho»

| Clave | ES | EN |
|---|---|---|
| `preset.title` | Cargar ritmo hecho | Load ready-made rhythm |
| `preset.text` | Elige un ritmo para el compás {timeSignature}. Sustituirá el contenido del patrón «{name}». | Choose a rhythm for {timeSignature}. It will replace the content of pattern “{name}”. |
| `preset.load` | Cargar | Load |
| `common.cancel` | Cancelar | Cancel |

Contiene una lista con los nombres de los ritmos hechos del compás actual (05 §9.2), en su orden, con el primero seleccionado. `↑` y `↓` cambian la selección; «Cargar» (o `Intro`) aplica RF-209 y cierra; «Cancelar» (o `Esc`) cierra sin cambios.

## 9. Pestaña Canción

### 9.1 Disposición

```text
┌ Barra de herramientas (44 px) ───────────────────────────────────────────────────┐
│ [Nueva pista de audio] [Importar audio…] [✓ Ajuste a la rejilla]                  │
│ [Repetir] [Partir] [Duplicar] [Eliminar]                      [Alejar] [Acercar]  │
├ Cabeceras (240 px) ─┬ Regla (44 px): números de compás (22 px) ──────────────────┤
│                     │  1       2       3       4       5       6                 │
│                     │ Franja de bucle (22 px):  [■■■■■■■■■■■■■■■■■■■■■■■]          │
├─────────────────────┼────────────────────────────────────────────────────────────┤
│▌Ritmo 1      Ritmo  │ ┌ Patrón 1 ┆       ┆       ┆       ┆       ┆      ┐        │
│ [Pista ▾]           │ └──────────┴───────┴───────┴───────┴───────┴──────┘        │
│ Patrón [P1 ▾] [Añadir bloque]                                                    │
├─────────────────────┼────────────────────────────────────────────────────────────┤
│▌Audio 1      Audio  │                 ┌ Toma 1 ∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿ ┐              │
│ [Pista ▾]           │                 └────────────────────────────┘              │
│ [✓ Armar] ▮▮▮▯▯ SAT │                                                            │
└─────────────────────┴────────────────────────────────────────────────────────────┘
```

| Elemento | Medida |
|---|---|
| Columna de cabeceras | 240 px de ancho; no se desplaza en horizontal |
| Regla | 44 px de alto: 22 px de números de compás y 22 px de franja de bucle; no se desplaza en vertical |
| Carril y cabecera de una pista | 72 px de alto |
| Bloque o clip | 64 px de alto, centrado en su carril |
| Zona sensible de un borde | 6 px a cada lado; 3 px si el elemento mide menos de 18 px de ancho |
| Cursor de reproducción | Línea vertical de 1 px `--accent` sobre regla y carriles, con un triángulo de 8 px en la regla |

Conversión entre posición y píxeles, con `zoom` = píxeles por negra:

```text
x(tick) = tick · zoom / 960 − desplazamientoHorizontal        (en píxeles, desde el borde izquierdo de los carriles)
tick(x) = round((x + desplazamientoHorizontal) · 960 / zoom)
```

El ancho total de la línea de tiempo es `x(maxSongTick)` (05 §2.2). Las líneas de la rejilla se dibujan según RF-304.

### 9.2 Controles de la barra de herramientas

| Clave | Etiqueta ES | Etiqueta EN | Control | Explicación ES | Explicación EN | Req. |
|---|---|---|---|---|---|---|
| `song.newAudioTrack` | Nueva pista de audio | New audio track | Botón | Crear una pista de audio lista para grabar. | Create an audio track ready to record. | RF-301 |
| `song.import` | Importar audio… | Import audio… | Botón | Añadir un archivo WAV o MP3 a la canción. También puedes arrastrarlo aquí desde el Explorador. | Add a WAV or MP3 file to the song. You can also drag it here from File Explorer. | RF-315 |
| `song.snap` | Ajuste a la rejilla | Snap to grid | Botón de dos estados | Al mover o recortar clips, ajustar al paso más cercano. | When moving or trimming clips, snap to the nearest step. | RF-316 |
| `song.repeat` | Repetir | Repeat | Botón de dos estados | Hacer que el clip seleccionado repita su contenido al alargarlo. | Make the selected clip repeat its content when lengthened. | RF-318 |
| `song.split` | Partir | Split | Botón | Dividir el clip seleccionado en el cursor (S). | Split the selected clip at the playhead (S). | RF-319 |
| `song.duplicate` | Duplicar | Duplicate | Botón | Crear una copia del elemento seleccionado justo después (Ctrl+D). | Create a copy of the selected item right after it (Ctrl+D). | RF-320 |
| `song.delete` | Eliminar | Delete | Botón | Eliminar el elemento seleccionado (Supr). | Delete the selected item (Del). | RF-321 |
| `song.zoomOut` | Alejar | Zoom out | Botón | Ver más tiempo en pantalla (Ctrl+−). | See more time on screen (Ctrl+−). | RF-305 |
| `song.zoomIn` | Acercar | Zoom in | Botón | Ver la canción con más detalle (Ctrl++). | See the song in more detail (Ctrl++). | RF-305 |

«Repetir» se muestra activado cuando el clip seleccionado tiene `repeat = true`.

Explicaciones cuando un control está desactivado:

| Clave | Cuándo | ES | EN |
|---|---|---|---|
| `song.needClip` | «Repetir» o «Partir» sin un clip seleccionado | Selecciona un clip de audio. | Select an audio clip. |
| `song.needItem` | «Duplicar» o «Eliminar» sin selección | Selecciona un bloque o un clip. | Select a block or a clip. |
| `song.split.block` | «Partir» con un bloque seleccionado | Los bloques de patrón no se pueden partir. | Pattern blocks cannot be split. |
| `song.split.repeat` | «Partir» con un clip con «Repetir» | No se puede partir un clip con «Repetir» activado. | A clip with Repeat on cannot be split. |
| `song.split.cursor` | «Partir» con el cursor fuera del clip o a menos de `MIN_CLIP_FRAMES` de un borde | Coloca el cursor dentro del clip, a más de 50 ms de sus bordes. | Place the playhead inside the clip, more than 50 ms from its edges. |
| `song.zoom.min` / `song.zoom.max` | Zoom en un extremo | Ya está en el zoom mínimo. / Ya está en el zoom máximo. | Already at minimum zoom. / Already at maximum zoom. |

### 9.3 Cabecera de pista

Cada cabecera tiene una franja vertical de 4 px con el color de la pista y dos líneas de contenido (RF-303):

| Clave | Etiqueta ES | Etiqueta EN | Control | Explicación ES | Explicación EN | Req. |
|---|---|---|---|---|---|---|
| — | (nombre de la pista) | — | Texto; se recorta con «…» si no cabe | — | — | — |
| `track.type.rhythm` / `track.type.audio` | Ritmo / Audio | Rhythm / Audio | Texto de 12 px, `--text-dim` | — | — | RF-303 |
| `track.menu` | Pista | Track | Menú | Renombrar, mover o eliminar esta pista. | Rename, move or delete this track. | RF-302 |
| `track.arm` | Armar | Arm | Botón de dos estados (solo en pistas de audio) | Preparar esta pista para recibir la próxima grabación. | Get this track ready to receive the next recording. | RF-310 |
| `track.inputLevel` | — | — | Medidor horizontal mono de 72 × 8 px e indicador de saturación (solo en la pista armada) | Nivel del micrófono. Si aparece SAT, baja el nivel del micrófono en Windows. | Microphone level. If CLIP shows, lower the microphone level in Windows. | RF-505 |
| `track.pattern` | Patrón | Pattern | Lista desplegable (solo en pistas de ritmo) | Patrón que se usará al añadir un bloque. | The pattern used when adding a block. | RF-307 |
| `track.addBlock` | Añadir bloque | Add block | Botón (solo en pistas de ritmo) | Colocar un bloque del patrón elegido en el compás del cursor. | Place a block of the chosen pattern at the playhead's bar. | RF-307 |

Órdenes del menú «Pista»:

| Clave | ES | EN | Desactivada cuando |
|---|---|---|---|
| `track.menu.rename` | Renombrar | Rename | — |
| `track.menu.up` | Subir | Move up | Es la primera pista |
| `track.menu.down` | Bajar | Move down | Es la última pista |
| `track.menu.delete` | Eliminar pista | Delete track | — |

- «Renombrar» convierte el nombre en un campo de texto, con el mismo comportamiento que en §8.3.
- Un clic en cualquier punto de la cabecera que no sea un control selecciona la pista. La pista seleccionada se distingue por un borde de 1 px `--accent` y fondo `--bg-raised`.
- La lista «Patrón» recuerda su valor mientras el proyecto está abierto; no forma parte del proyecto.

### 9.4 Regla y franja de bucle

- **Números de compás** (RF-304): cada compás lleva su número sobre su línea. Si a ese zoom dos números quedarían a menos de 40 px, solo se numera uno de cada 2, 4, 8 o 16 compases (el menor salto que deja 40 px o más).
- **Clic en la zona de números**: coloca el cursor (RF-104) o, si se reproduce en modo canción, mueve la reproducción (RF-114).
- **Franja de bucle** (RF-111): el tramo se dibuja como un rectángulo `--accent` al 60 % de opacidad si el bucle está activado, y solo con su contorno si no lo está.
  - Arrastrar sobre una zona vacía de la franja crea un tramo nuevo desde el compás donde se pulsó hasta el compás donde está el ratón, ambos incluidos.
  - Arrastrar un borde del tramo (zona sensible de 6 px) lo mueve de compás en compás.
  - El tramo mide como mínimo un compás y no pasa del último compás completo antes del límite de la canción.
  - Explicación de la franja (`song.loopStrip`): «Arrastra para marcar el tramo de bucle.» / «Drag to mark the loop range.»

### 9.5 Bloques y clips

Aspecto:

| Elemento | Aspecto |
|---|---|
| Bloque de patrón | Rectángulo del color de la pista; arriba a la izquierda, el nombre del patrón en 12 px; una línea vertical discontinua `#101214` en cada repetición del patrón |
| Clip de audio | Rectángulo del color de la pista al 70 % de opacidad; arriba a la izquierda, el nombre de su archivo de audio; dentro, la forma de onda en `#101214`; con «Repetir», una línea vertical discontinua en cada repetición y el símbolo `↻` tras el nombre |
| Clip sin datos de onda todavía | Rectángulo liso |
| Clip cuyo archivo falta | Rayado diagonal `--text-dim` sobre `--bg-raised` y el texto `clip.missing`: «Archivo no encontrado» / «File not found» |
| Elemento seleccionado | Contorno de 2 px `--text` |
| Bloque de grabación | Rectángulo `--record` al 70 % con el texto `clip.recording`: «Grabando» / «Recording», desde el punto de inicio hasta la posición actual |
| Destino al arrastrar un archivo | El carril sobre el que está el ratón muestra un contorno discontinuo `--accent`; fuera de los carriles de audio, el contorno rodea toda la línea de tiempo |

Solo se dibuja la parte visible de cada elemento (los segmentos que da `resolveLane`, 05 §10.2). La onda se dibuja a partir del archivo de picos (05 §6.3): por cada columna de píxeles, una línea vertical entre el mínimo y el máximo de los frames que le corresponden.

Manejo con el ratón:

| Gesto | Sobre | Efecto | Req. |
|---|---|---|---|
| Clic | Cuerpo o borde de un elemento | Lo selecciona | RF-306 |
| Clic | Zona vacía de un carril | Quita la selección de elemento y selecciona esa pista | RF-306 |
| Arrastrar en horizontal | Cuerpo de un bloque | Lo mueve de compás en compás | RF-308 |
| Arrastrar en horizontal | Borde derecho de un bloque | Cambia su longitud de compás en compás | RF-308 |
| Arrastrar en horizontal | Cuerpo de un clip | Lo mueve; al paso más cercano si «Ajuste a la rejilla» está activado | RF-316 |
| Arrastrar en horizontal | Borde de un clip sin «Repetir» | Recorta o descubre audio | RF-317 |
| Arrastrar en horizontal | Borde derecho de un clip con «Repetir» | Cambia su longitud repitiendo la región | RF-318 |
| Soltar un archivo del Explorador | La pestaña Canción | Importa | RF-315 |

- El puntero cambia a flecha doble horizontal sobre un borde que se puede arrastrar, y a «prohibido» sobre el borde izquierdo de un bloque o de un clip con «Repetir».
- Un arrastre también selecciona el elemento.
- Los elementos no se pueden arrastrar en vertical ni cambiar de pista.
- Cuando una operación se limita por el final de la canción se muestra el aviso E-004; cuando no cabe nada, E-005.

Cálculo de un movimiento, a partir del desplazamiento horizontal `dx` del ratón desde que empezó el arrastre:

```text
Clip:    tickNuevo = tickInicial + round(dx · 960 / zoom)
         si el ajuste está activado: tickNuevo = round(tickNuevo / 240) · 240
         se limita a  0 ≤ tickNuevo  y  endFrame ≤ MAX_SONG_FRAMES
Bloque:  compásNuevo = compásInicial + round(dx · 960 / zoom / ticksPerBar)
         se limita a  0 ≤ compásNuevo  y  endFrame ≤ MAX_SONG_FRAMES
```

Los bordes se calculan igual, aplicando el desplazamiento al borde y las fórmulas y límites de 05 §3.3. El manejo con el teclado está en RF-324.

### 9.6 Desplazamiento y seguimiento

RF-305 define el zoom, el desplazamiento y el seguimiento del cursor. Además:

- La columna de cabeceras y los carriles se desplazan juntos en vertical; la regla y los carriles, juntos en horizontal.
- `Ctrl` + rueda cambia el zoom igual que los botones.

## 10. Pestaña Mezcla

### 10.1 Disposición

```text
┌ Canales (112 px cada uno) ──────────────────────────────────────┬ Maestro (128 px)┐
│▀▀▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀▀▀                                       │                 │
│ Ritmo 1      Audio 1                                            │ Maestro         │
│ Ritmo        Audio                                              │                 │
│ [Ecualizador][Ecualizador]                                      │                 │
│ Graves ─○─   Graves ─○─                                         │                 │
│ Medios ─○─   Medios ─○─                                         │                 │
│ Agudos ─○─   Agudos ─○─                                         │                 │
│ [Reverb]     [✓ Reverb]                                         │                 │
│ Cantidad ─○─ Cantidad ─○─                                       │                 │
│ Tamaño ─○─   Tamaño ─○─                                         │                 │
│ Panorama ─○─ C   …                                              │                 │
│ [Silenciar]  [Silenciar]                                        │                 │
│ [Solo]       [Solo]                                             │                 │
│  ┃ ▮▮        ┃ ▮▮                                               │  ┃ ▮▮           │
│  ○ ▮▮  SAT   ○ ▮▮  SAT                                          │  ○ ▮▮  SAT      │
│  ┃ ▮▮        ┃ ▮▮                                               │  ┃ ▮▮           │
│ −3,0 dB      0,0 dB                                             │ 0,0 dB          │
└─────────────────────────────────────────────────────────────────┴─────────────────┘
```

- Cada canal mide 112 px de ancho; el maestro, 128 px, y queda fijo a la derecha (RF-400). Entre canales hay una línea `--border`.
- Arriba de cada canal hay una franja de 4 px con el color de la pista.
- El deslizador de volumen es vertical y ocupa la altura sobrante, con un mínimo de 160 px. A su derecha va el medidor estéreo (dos barras) y, junto a él, el indicador de saturación.
- Si los canales no caben, se desplazan en horizontal.

### 10.2 Controles de un canal

| Clave | Etiqueta ES | Etiqueta EN | Control | Explicación ES | Explicación EN | Req. |
|---|---|---|---|---|---|---|
| `mix.eq` | Ecualizador | Equalizer | Botón de dos estados | Activar o desactivar el ecualizador de esta pista. | Turn this track's equalizer on or off. | RF-406 |
| `mix.eq.low` | Graves | Low | Deslizador, −12 a +12 dB, paso 0,5 | Subir o bajar las frecuencias por debajo de 150 Hz. | Boost or cut frequencies below 150 Hz. | RF-406 |
| `mix.eq.mid` | Medios | Mid | Deslizador, −12 a +12 dB, paso 0,5 | Subir o bajar las frecuencias en torno a 1 kHz. | Boost or cut frequencies around 1 kHz. | RF-406 |
| `mix.eq.high` | Agudos | High | Deslizador, −12 a +12 dB, paso 0,5 | Subir o bajar las frecuencias por encima de 6 kHz. | Boost or cut frequencies above 6 kHz. | RF-406 |
| `mix.reverb` | Reverb | Reverb | Botón de dos estados | Activar o desactivar la reverb de esta pista. | Turn this track's reverb on or off. | RF-407 |
| `mix.reverb.amount` | Cantidad | Amount | Deslizador, 0 a 100 %, paso 1 | Cuánta reverberación se añade. | How much reverb is added. | RF-407 |
| `mix.reverb.size` | Tamaño | Size | Deslizador, 0 a 100 %, paso 1 | Lo que tarda la reverberación en apagarse. | How long the reverb takes to fade. | RF-407 |
| `mix.pan` | Panorama | Pan | Deslizador, −100 a +100, paso 1 | Posición entre el altavoz izquierdo y el derecho. | Position between the left and right speakers. | RF-402 |
| `mix.mute` | Silenciar | Mute | Botón de dos estados | Hacer que esta pista no suene. | Silence this track. | RF-403 |
| `mix.solo` | Solo | Solo | Botón de dos estados | Hacer que solo suenen las pistas con Solo activado. | Play only the tracks with Solo on. | RF-403 |
| `mix.gain` | Volumen | Volume | Deslizador vertical, −60 a +6 dB, paso 0,5 | Volumen de la pista. | Track volume. | RF-401 |
| `mix.level` | — | — | Medidor estéreo e indicador de saturación | Nivel de pico en dBFS. Pulsa SAT para apagar el aviso. | Peak level in dBFS. Click CLIP to clear the warning. | RF-404 |

Canal maestro: título `mix.master` («Maestro» / «Master»), deslizador `mix.master.gain` («Volumen» / «Volume»; explicación «Volumen de toda la mezcla.» / «Volume of the whole mix.») y medidor con indicador de saturación (RF-405).

Presentación de valores:

| Valor | ES | EN |
|---|---|---|
| Volumen y bandas | `−3,0 dB`; en el mínimo del volumen, `−∞` | `−3.0 dB`; `−∞` |
| Cantidad y tamaño | `20 %` | `20%` |
| Panorama | `C` en 0; `I 30` a la izquierda; `D 30` a la derecha | `C`; `L 30`; `R 30` |

Con el ecualizador o la reverb desactivados, sus deslizadores siguen activos y conservan su valor; solo cambia el botón.

## 11. Cuadros de diálogo

Todos siguen las reglas de §4. Los cuadros de mensaje (errores, avisos con consecuencias, guardar cambios, recuperación) están en el catálogo de §13; aquí están los que tienen contenido propio.

### 11.1 Pantalla de inicio (RF-004)

| Clave | ES | EN |
|---|---|---|
| `start.title` | Carabo | Carabo |
| `start.new` | Nuevo proyecto | New project |
| `start.open` | Abrir proyecto… | Open project… |
| `start.recents` | Recientes | Recent |
| `start.recents.empty` | Aún no hay proyectos recientes. | No recent projects yet. |

Los dos botones van arriba, uno junto al otro, y «Nuevo proyecto» tiene el foco inicial. Debajo, la lista de recientes: cada línea muestra el nombre del proyecto (13 px) y la ruta de su carpeta (12 px, `--text-dim`); un clic o `Intro` la abre. `Esc` equivale a «Nuevo proyecto».

### 11.2 Guardar como (RF-024)

| Clave | ES | EN |
|---|---|---|
| `saveAs.title` | Guardar como | Save as |
| `saveAs.name` | Nombre del proyecto | Project name |
| `saveAs.location` | Ubicación | Location |
| `saveAs.change` | Cambiar… | Change… |
| `saveAs.save` | Guardar | Save |

- El campo de nombre empieza con el nombre actual del proyecto (vacío si es «Sin título») y tiene el foco.
- La ubicación se muestra como texto, con el botón «Cambiar…» a su derecha.
- «Guardar» está desactivado mientras el nombre está vacío. Los errores E-104 y E-105 se muestran en `--record` bajo el campo de nombre, sin cerrar el cuadro.

### 11.3 Exportar audio (RF-600 a RF-603)

| Clave | ES | EN |
|---|---|---|
| `export.title` | Exportar audio | Export audio |
| `export.format` | Formato | Format |
| `export.format.wav` | WAV (48 kHz, 24 bits) | WAV (48 kHz, 24-bit) |
| `export.format.mp3` | MP3 (320 kbps) | MP3 (320 kbps) |
| `export.go` | Exportar… | Export… |
| `export.progress` | Exportando… {percent} % | Exporting… {percent}% |
| `export.done` | Exportación terminada | Export finished |
| `export.openFolder` | Abrir carpeta | Open folder |

Tiene tres estados sucesivos:

1. **Elegir formato**: selector de opciones con WAV elegido; botones «Exportar…» y «Cancelar». «Exportar…» abre el cuadro nativo de guardar; si el usuario lo cancela, se vuelve a este estado.
2. **Progreso**: texto `export.progress`, barra de progreso y botón «Cancelar».
3. **Final**: texto `export.done`, la ruta del archivo y, si procede, el texto E-132; botones «Abrir carpeta» y «Cerrar».

### 11.4 Ajustes (RF-700)

| Clave | ES | EN | Explicación ES | Explicación EN |
|---|---|---|---|---|
| `settings.title` | Ajustes | Settings | — | — |
| `settings.language` | Idioma | Language | Idioma de la interfaz. | Interface language. |
| `settings.audio` | Audio | Audio | — | — |
| `settings.output` | Salida | Output | Dispositivo por el que suena Carabo. | The device Carabo plays through. |
| `settings.input` | Entrada | Input | Dispositivo con el que se graba. | The device used to record. |
| `settings.device.default` | Predeterminado de Windows | Windows default | — | — |
| `settings.testOutput` | Probar salida | Test output | Emite un tono de un segundo para comprobar que se oye. | Plays a one-second tone to check you can hear it. |
| `settings.inputLevel` | Nivel de entrada | Input level | Nivel del micrófono en este momento. | The microphone level right now. |
| `settings.calibrate` | Calibrar retardo | Calibrate latency | Mide cuánto tarda el sonido en salir y volver a entrar, para colocar bien las grabaciones. | Measures how long sound takes to go out and come back in, to place recordings correctly. |
| `settings.calibrate.hint` | Acerca los auriculares al micrófono y sube el volumen. Sonarán 8 clics. | Hold the headphones close to the microphone and turn the volume up. You will hear 8 clicks. | — | — |
| `settings.calibrate.running` | Calibrando… | Calibrating… | — | — |
| `settings.calibrate.result` | Retardo medido: {n} ms | Measured latency: {n} ms | — | — |
| `settings.latency` | Retardo de grabación (ms) | Recording latency (ms) | Tiempo que se descuenta al colocar cada grabación (0 a 500 ms). | Time subtracted when placing each recording (0 to 500 ms). |
| `settings.about` | Acerca de | About | — | — |
| `settings.about.license` | Licencia MIT | MIT License | — | — |
| `common.close` | Cerrar | Close | — | — |

- Tres secciones, una debajo de otra: Idioma (selector de opciones «Español» / «English»), Audio y Acerca de.
- «Salida» y «Entrada» son listas desplegables cuya primera opción es `settings.device.default`. Si no hay ningún dispositivo de un tipo, la lista muestra `settings.device.none`: «No hay ningún dispositivo» / «No device found».
- El texto `settings.calibrate.hint` se muestra siempre bajo el botón «Calibrar retardo». Al pulsarlo, el botón pasa a «Calibrando…» y se desactiva hasta que termina; después aparece `settings.calibrate.result` o el cuadro E-140.
- «Acerca de» muestra `Carabo {version}`, `settings.about.license` y la dirección del repositorio como texto (07).
- Al abrir el cuadro se abre el dispositivo de entrada, si no lo estaba, para alimentar el medidor «Nivel de entrada»; al cerrarlo se cierra, salvo que haya una pista armada.
- El único botón del cuadro es «Cerrar».

## 12. Tabla de acciones

Lista completa de las acciones del programa. Cada una tiene un único control visible (PD-05); el atajo y el gesto son equivalentes opcionales.

| Acción | Control | Dónde | Atajo | Gesto | Req. |
|---|---|---|---|---|---|
| Nuevo proyecto | Archivo > Nuevo proyecto | Barra superior | Ctrl+N | — | RF-020 |
| Abrir proyecto | Archivo > Abrir proyecto… | Barra superior | Ctrl+O | — | RF-021 |
| Abrir un reciente | Archivo > Abrir reciente | Barra superior | — | — | RF-022 |
| Guardar | Archivo > Guardar | Barra superior | Ctrl+S | — | RF-023 |
| Guardar como | Archivo > Guardar como… | Barra superior | Ctrl+Mayús+S | — | RF-024 |
| Exportar audio | Archivo > Exportar audio… | Barra superior | Ctrl+E | — | RF-600 |
| Salir | Archivo > Salir | Barra superior | Alt+F4 | Cerrar la ventana | RF-010 |
| Deshacer / Rehacer | Deshacer / Rehacer | Barra superior | Ctrl+Z / Ctrl+Y | — | RF-030 |
| Cambiar de pestaña | Ritmo / Canción / Mezcla | Barra superior | Ctrl+1 / 2 / 3 | — | RF-003 |
| Abrir ajustes | Ajustes | Barra superior | — | — | RF-700 |
| Reproducir o parar | Reproducir / Parar | Barra de reproducción | Espacio | — | RF-101 |
| Grabar | Grabar | Barra de reproducción | R | — | RF-311 |
| Cambiar el tempo | Tempo | Barra de reproducción | — | — | RF-106 |
| Cambiar el compás | Compás | Barra de reproducción | — | — | RF-108 |
| Activar la claqueta | Claqueta | Barra de reproducción | — | — | RF-110 |
| Activar el bucle | Bucle | Barra de reproducción | — | — | RF-111 |
| Marcar el tramo de bucle | Franja de bucle | Canción, regla | — | Arrastrar | RF-111 |
| Colocar el cursor | Zona de números de la regla | Canción, regla | Inicio (al compás 1) | Clic | RF-104 |
| Crear pista de ritmo | Nueva pista de ritmo | Ritmo | — | — | RF-201 |
| Elegir pista de ritmo / kit | Pista / Kit | Ritmo | — | — | RF-202, RF-203 |
| Cargar un ritmo hecho | Cargar ritmo hecho… | Ritmo | — | — | RF-209 |
| Borrar los golpes | Borrar golpes | Ritmo | — | — | RF-212 |
| Cambiar la longitud del patrón | Compases | Ritmo | — | — | RF-207 |
| Crear / duplicar / renombrar / eliminar patrón | Nuevo patrón / Duplicar / Renombrar / Eliminar | Ritmo, lista de patrones | F2 (renombrar) | — | RF-208 |
| Marcar o quitar un golpe | Paso de la rejilla | Ritmo | Espacio (con el foco en el paso) | Clic | RF-205 |
| Escuchar un sonido | Nombre de la fila | Ritmo | — | — | RF-206 |
| Volumen / silenciar una fila | Volumen / Silenciar de la fila | Ritmo | — | — | RF-210 |
| Crear pista de audio | Nueva pista de audio | Canción | — | — | RF-301 |
| Renombrar / subir / bajar / eliminar pista | Menú Pista | Canción, cabecera | — | — | RF-302 |
| Armar una pista | Armar | Canción, cabecera | — | — | RF-310 |
| Añadir un bloque | Patrón + Añadir bloque | Canción, cabecera | — | — | RF-307 |
| Importar audio | Importar audio… | Canción | — | Soltar un archivo | RF-315 |
| Ajuste a la rejilla | Ajuste a la rejilla | Canción | — | — | RF-316 |
| Seleccionar un elemento | El propio elemento | Canción | Tab, flechas | Clic | RF-306 |
| Mover un elemento | El propio elemento | Canción | ← → | Arrastrar | RF-308, RF-316 |
| Alargar, acortar o recortar | Borde del elemento | Canción | Mayús o Alt + ← → | Arrastrar | RF-308, RF-317 |
| Repetir un clip | Repetir | Canción | — | — | RF-318 |
| Partir un clip | Partir | Canción | S | — | RF-319 |
| Duplicar un elemento | Duplicar | Canción | Ctrl+D | — | RF-320 |
| Eliminar un elemento | Eliminar | Canción | Supr | — | RF-321 |
| Acercar / alejar | Acercar / Alejar | Canción | Ctrl++ / Ctrl+− | Ctrl + rueda | RF-305 |
| Volumen, panorama, silenciar, solo | Controles del canal | Mezcla | — | — | RF-401 a RF-403 |
| Ecualizador y reverb | Controles del canal | Mezcla | — | — | RF-406, RF-407 |
| Volumen maestro | Volumen del maestro | Mezcla | — | — | RF-405 |
| Apagar un aviso de saturación | Indicador SAT | Mezcla, cabecera de pista armada | — | — | RF-404 |
| Idioma, dispositivos, calibrar, retardo, probar salida | Controles de Ajustes | Cuadro de ajustes | — | — | RF-501, RF-506, RF-507, RF-700, RF-701 |

## 13. Catálogo de mensajes

Todos los avisos y cuadros de mensaje del programa (RF-007). El identificador `E-nnn` no se muestra nunca al usuario; la clave de traducción de cada texto es `msg.E<nnn>` (por ejemplo, `msg.E103`) y la de cada botón, `msg.E<nnn>.<botón>`. `{…}` son datos que se sustituyen.

### 13.1 Mensajes de la barra de estado

«Tipo»: **I**, informativo; **A**, aviso.

| Id | Tipo | Cuándo | ES | EN |
|---|---|---|---|---|
| E-001 | I | Guardado correcto (RF-023) | Proyecto guardado. | Project saved. |
| E-002 | A | Crear una pista con 16 ya existentes | No se pueden crear más pistas: el máximo es 16. Elimina una pista para añadir otra. | No more tracks can be created: the maximum is 16. Delete a track to add another. |
| E-003 | A | Crear un patrón con 32 ya existentes | Esta pista ya tiene 32 patrones, que es el máximo. Elimina uno para crear otro. | This track already has 32 patterns, the maximum. Delete one to create another. |
| E-004 | A | Un elemento se ha limitado por el final de la canción (RF-112) | La canción no puede pasar de 10 minutos. El elemento se ha ajustado al límite. | The song cannot be longer than 10 minutes. The item was fitted to the limit. |
| E-005 | A | Un bloque, clip o copia no cabe (RF-307, RF-320) | No cabe antes del límite de 10 minutos de la canción. Acorta o elimina algo para hacer sitio. | It does not fit before the song's 10-minute limit. Shorten or delete something to make room. |
| E-006 | I | Patrón eliminado junto con sus bloques (RF-208) | Se ha eliminado el patrón y sus {n} bloques de la canción. Puedes deshacerlo con Ctrl+Z. | The pattern and its {n} blocks in the song were deleted. You can undo with Ctrl+Z. |
| E-007 | I | Importación en curso (RF-315) | Importando «{file}»… {percent} % | Importing “{file}”… {percent}% |
| E-008 | A | Archivo importado de más de 10 minutos | «{file}» dura más de 10 minutos: solo se han importado los 10 primeros. | “{file}” is longer than 10 minutes: only the first 10 were imported. |
| E-009 | A | Se intenta importar con otra importación en curso | Espera a que termine la importación en curso. | Wait for the current import to finish. |
| E-010 | A | Un dispositivo guardado no existe al arrancar (RF-500) | No se encuentra el dispositivo «{device}». Se usa el predeterminado de Windows; puedes elegir otro en Ajustes. | The device “{device}” was not found. The Windows default is used; you can choose another in Settings. |
| E-011 | A | La entrada desaparece con una pista armada (RF-504) | El micrófono «{device}» se ha desconectado y la pista se ha desarmado. Vuelve a conectarlo y pulsa Armar. | The microphone “{device}” was disconnected and the track was disarmed. Reconnect it and click Arm. |
| E-012 | A | Toma demasiado corta (RF-313) | La grabación era demasiado corta y se ha descartado. Vuelve a grabar. | The recording was too short and was discarded. Record again. |
| E-013 | A | Cambio de tempo o de compás que dejaría algo fuera del límite (RF-112) | No se puede aplicar el cambio: parte de la canción quedaría más allá de los 10 minutos. Acorta la canción antes. | The change cannot be applied: part of the song would end beyond 10 minutes. Shorten the song first. |
| E-014 | I | Importación terminada | «{file}» importado. | “{file}” imported. |
| E-015 | A | Tempo escrito fuera de rango (RF-106) | El tempo debe estar entre 40 y 240. Se ha ajustado a {value}. | Tempo must be between 40 and 240. It was set to {value}. |

### 13.2 Cuadros de mensaje

El botón que aparece primero es el principal. Todos se cierran también con `Esc`, que equivale a «Cerrar» o a «Cancelar».

| Id | Cuándo | ES | EN | Botones ES / EN |
|---|---|---|---|---|
| E-100 | El archivo de proyecto no es válido (RF-021) | No se puede abrir el proyecto: su archivo está dañado o no es un proyecto de Carabo. Abre otro proyecto. | The project cannot be opened: its file is damaged or is not a Carabo project. Open another project. | Cerrar / Close |
| E-101 | Versión de formato más reciente (RF-034) | Este proyecto se creó con una versión más reciente de Carabo y no se puede abrir con esta. Actualiza Carabo para abrirlo. | This project was created with a newer version of Carabo and cannot be opened with this one. Update Carabo to open it. | Cerrar / Close |
| E-102 | Un reciente ya no existe (RF-022) | El proyecto «{name}» ya no está en {path}. Puede que se haya movido o borrado. | The project “{name}” is no longer at {path}. It may have been moved or deleted. | Quitar de la lista / Remove from list · Cerrar / Close |
| E-103 | No se puede guardar (RF-023) | No se ha podido guardar el proyecto en {path}. Comprueba que el disco tiene espacio y que puedes escribir en esa carpeta, o guárdalo en otro sitio. | The project could not be saved to {path}. Check that the disk has space and that you can write to that folder, or save it somewhere else. | Guardar como… / Save as… · Cerrar / Close |
| E-104 | Nombre no válido en «Guardar como» (junto al campo) | El nombre no puede estar vacío, contener \ / : * ? " < > \| ni empezar o terminar con espacio o punto. | The name cannot be empty, contain \ / : * ? " < > \| or start or end with a space or a dot. | — |
| E-105 | Ya existe ese nombre (junto al campo) | Ya existe «{name}» en esa ubicación. Elige otro nombre u otra ubicación. | “{name}” already exists in that location. Choose another name or location. | — |
| E-106 | Faltan archivos de audio (RF-028) | Faltan {n} archivos de audio de este proyecto. Los clips que los usan aparecen como «Archivo no encontrado» y no suenan. Si los has movido, devuélvelos a la carpeta «audio» del proyecto y vuelve a abrirlo. | {n} audio files of this project are missing. The clips that use them show “File not found” and are silent. If you moved them, put them back in the project's “audio” folder and open it again. | Cerrar / Close |
| E-107 | No se puede crear un proyecto nuevo (RF-020) | No se ha podido crear el proyecto en la carpeta de datos de Carabo. Comprueba el espacio en disco y vuelve a intentarlo. | The project could not be created in Carabo's data folder. Check the disk space and try again. | Cerrar / Close |
| E-110 | No hay salida (RF-503) | No hay ningún dispositivo de salida de audio. Conecta unos auriculares o unos altavoces, o elige otro dispositivo en Ajustes. | There is no audio output device. Connect headphones or speakers, or choose another device in Settings. | Abrir ajustes / Open settings · Cerrar / Close |
| E-111 | No hay entrada (RF-503) | No hay ningún micrófono ni entrada de audio. Conecta uno y pulsa Reintentar, o elige otro dispositivo en Ajustes. | There is no microphone or audio input. Connect one and click Retry, or choose another device in Settings. | Reintentar / Retry · Abrir ajustes / Open settings · Cerrar / Close |
| E-112 | Grabar sin pistas de audio (RF-311) | Para grabar hace falta una pista de audio. Crea una y vuelve a pulsar Grabar. | Recording needs an audio track. Create one and click Record again. | Crear pista de audio / Create audio track · Cerrar / Close |
| E-113 | Grabar sin pista armada (RF-311) | Ninguna pista está armada para grabar. Arma la pista en la que quieres grabar y vuelve a pulsar Grabar. | No track is armed for recording. Arm the track you want to record on and click Record again. | Armar «{track}» / Arm “{track}” · Cerrar / Close |
| E-114 | La salida desaparece y hay otra (RF-504) | El dispositivo de salida «{old}» se ha desconectado. Ahora se usa «{new}»; puedes elegir otro en Ajustes. | The output device “{old}” was disconnected. “{new}” is now used; you can choose another in Settings. | Abrir ajustes / Open settings · Cerrar / Close |
| E-115 | La salida desaparece y no hay otra (RF-504) | El dispositivo de salida «{old}» se ha desconectado y no hay ningún otro. Conecta unos auriculares o unos altavoces para seguir oyendo. | The output device “{old}” was disconnected and there is no other. Connect headphones or speakers to keep listening. | Abrir ajustes / Open settings · Cerrar / Close |
| E-116 | La entrada desaparece durante la grabación (RF-314) | El micrófono se ha desconectado durante la grabación. Se ha conservado lo grabado hasta ese momento. Vuelve a conectarlo para seguir grabando. | The microphone was disconnected while recording. What was recorded up to that point was kept. Reconnect it to keep recording. | Cerrar / Close |
| E-117 | Fallo de disco al grabar (RF-314) | No se ha podido seguir escribiendo la grabación en el disco. Se ha conservado lo grabado hasta ese momento. Libera espacio en el disco antes de volver a grabar. | The recording could no longer be written to disk. What was recorded up to that point was kept. Free up disk space before recording again. | Cerrar / Close |
| E-120 | Archivo no admitido al importar (RF-315) | No se puede importar «{file}»: no es un archivo WAV o MP3 válido. Conviértelo a WAV o MP3 e inténtalo de nuevo. | “{file}” cannot be imported: it is not a valid WAV or MP3 file. Convert it to WAV or MP3 and try again. | Cerrar / Close |
| E-121 | Fallo de lectura o escritura al importar | No se ha podido importar «{file}» porque ha fallado la lectura del archivo o la escritura en la carpeta del proyecto. Comprueba el espacio en disco e inténtalo de nuevo. | “{file}” could not be imported because reading the file or writing to the project folder failed. Check the disk space and try again. | Cerrar / Close |
| E-130 | Exportar una canción vacía (RF-600) | No hay nada que exportar: la canción no tiene ningún bloque ni clip. Añade algo en la pestaña Canción. | There is nothing to export: the song has no blocks or clips. Add something in the Song tab. | Cerrar / Close |
| E-131 | No se puede escribir la exportación (RF-602) | No se ha podido escribir el archivo en {path}. Elige otra carpeta o comprueba el espacio en disco. | The file could not be written to {path}. Choose another folder or check the disk space. | Cerrar / Close |
| E-132 | Texto añadido al final de la exportación (RF-603) | La mezcla satura en algunos puntos. Baja el volumen maestro en la pestaña Mezcla y vuelve a exportar. | The mix clips at some points. Lower the master volume in the Mix tab and export again. | — |
| E-140 | Calibración fallida (RF-506) | No he oído los clics con claridad. Acerca los auriculares al micrófono, sube el volumen y vuelve a calibrar. Si no es posible, escribe el retardo a mano. | I could not hear the clicks clearly. Hold the headphones closer to the microphone, turn the volume up and calibrate again. If that is not possible, type the latency by hand. | Cerrar / Close |
| E-141 | Primera grabación con una pareja de dispositivos sin retardo propio (RF-507) | Aún no has calibrado el retardo de este equipo, así que la grabación puede quedar un poco desplazada respecto al ritmo. Calibrarlo lleva unos segundos. | You have not calibrated this equipment's latency yet, so the recording may be slightly off the rhythm. Calibrating takes a few seconds. | Calibrar ahora / Calibrate now · Más tarde / Later |
| E-150 | Error interno (RNF-12) | El motor de audio ha tenido un problema y se ha detenido. Tu proyecto no se ha perdido: guárdalo y reinicia Carabo. | The audio engine had a problem and stopped. Your project is not lost: save it and restart Carabo. | Guardar / Save · Cerrar / Close |
| E-160 | Antes del primer cambio de tempo con audio (RF-107) | Si cambias el tempo, el audio grabado o importado no cambia de velocidad y dejará de coincidir con los ritmos. | If you change the tempo, recorded or imported audio does not change speed and will no longer match the rhythms. | Cambiar el tempo / Change tempo · Cancelar / Cancel |
| E-161 | Antes de cambiar el compás con contenido (RF-109) | Si cambias el compás, los patrones se recortan o se amplían para ajustarse al nuevo número de pasos por compás, y el audio grabado o importado dejará de coincidir con los ritmos. Puedes deshacerlo con Ctrl+Z. | If you change the time signature, patterns are cut or extended to fit the new number of steps per bar, and recorded or imported audio will no longer match the rhythms. You can undo with Ctrl+Z. | Cambiar el compás / Change time signature · Cancelar / Cancel |
| E-170 | Cambios sin guardar (RF-010) | ¿Guardar los cambios de «{name}»? | Save the changes to “{name}”? | Guardar / Save · No guardar / Don't save · Cancelar / Cancel |
| E-171 | Recuperación al arrancar (RF-027) | Carabo se cerró de forma inesperada. Hay una copia de recuperación de «{name}» del {date}. | Carabo closed unexpectedly. There is a recovery copy of “{name}” from {date}. | Recuperar / Recover · Descartar / Discard |

`{date}` se escribe con la fecha y la hora locales en el formato del idioma activo (por ejemplo, «15/11/2026 20:14»). El cuadro E-106 muestra, bajo el texto, la lista de los nombres de archivo que faltan, uno por línea.

## 14. Textos y claves de traducción

### 14.1 Reglas

1. Todos los textos visibles están en `src/i18n/es.json` y `src/i18n/en.json`, con las claves de este documento. Los dos archivos tienen exactamente las mismas claves (RNF-17).
2. La clave de la etiqueta de un control es su clave más `.label`; la de su explicación, más `.help`. Ejemplo: `song.split.label` = «Partir», `song.split.help` = «Dividir el clip seleccionado en el cursor (S).». Las claves que en las tablas de este documento ya nombran un texto concreto (por ejemplo `transport.record.inRhythm` o `clip.missing`) se usan tal cual.
3. Los datos entre llaves (`{name}`, `{n}`) se sustituyen al mostrar el texto.
4. Los números se formatean según el idioma: coma decimal en español, punto en inglés. El signo menos es `−` (U+2212).
5. Los nombres de kits y sonidos (05 §8.2) y de ritmos hechos (05 §9.2) no están en estos archivos: vienen en sus propios archivos de datos, en los dos idiomas.

### 14.2 Nombres que pone el programa

Se generan en el idioma activo en el momento de crearlos y no cambian después (RF-701).

| Clave | ES | EN |
|---|---|---|
| `names.untitled` | Sin título | Untitled |
| `names.rhythmTrack` | Ritmo {n} | Rhythm {n} |
| `names.audioTrack` | Audio {n} | Audio {n} |
| `names.pattern` | Patrón {n} | Pattern {n} |
| `names.copy` | {name} copia | {name} copy |
| `names.take` | Toma {n} | Take {n} |

### 14.3 Nombres de los pasos de historial

Se usan en las explicaciones de Deshacer y Rehacer (`{step}`).

| Clave | ES | EN |
|---|---|---|
| `history.createRhythmTrack` | crear pista de ritmo | create rhythm track |
| `history.createAudioTrack` | crear pista de audio | create audio track |
| `history.renameTrack` | renombrar pista | rename track |
| `history.moveTrack` | mover pista | move track |
| `history.deleteTrack` | eliminar pista | delete track |
| `history.setKit` | cambiar kit | change kit |
| `history.setRowGain` | volumen de fila | row volume |
| `history.setRowMute` | silenciar fila | mute row |
| `history.createPattern` | crear patrón | create pattern |
| `history.duplicatePattern` | duplicar patrón | duplicate pattern |
| `history.renamePattern` | renombrar patrón | rename pattern |
| `history.deletePattern` | eliminar patrón | delete pattern |
| `history.toggleStep` | cambiar golpe | change hit |
| `history.setPatternBars` | longitud del patrón | pattern length |
| `history.loadPreset` | cargar ritmo hecho | load ready-made rhythm |
| `history.clearPattern` | borrar golpes | clear hits |
| `history.addBlock` | añadir bloque | add block |
| `history.moveBlock` | mover bloque | move block |
| `history.resizeBlock` | alargar bloque | resize block |
| `history.duplicateItem` | duplicar | duplicate |
| `history.deleteItem` | eliminar | delete |
| `history.recordTake` | grabar toma | record take |
| `history.importAudio` | importar audio | import audio |
| `history.moveClip` | mover clip | move clip |
| `history.trimClip` | recortar clip | trim clip |
| `history.setClipRepeat` | repetir clip | repeat clip |
| `history.splitClip` | partir clip | split clip |
| `history.setStripGain` | volumen de pista | track volume |
| `history.setStripPan` | panorama | pan |
| `history.setStripMute` | silenciar pista | mute track |
| `history.setStripSolo` | solo | solo |
| `history.setEq` | ecualizador | equalizer |
| `history.setReverb` | reverb | reverb |
| `history.setMasterGain` | volumen maestro | master volume |
| `history.setTempo` | cambiar tempo | change tempo |
| `history.setTimeSignature` | cambiar compás | change time signature |

## 15. Accesibilidad

Requisito: RNF-16.

1. **Teclado.** Todo control se alcanza con `Tab` y se acciona con las teclas de §4. Los atajos están en RF-703.
2. **Foco visible.** El elemento con el foco lleva siempre el contorno de §2.2.
3. **Paradas únicas.** La rejilla de pasos, la lista de patrones, cada carril de la línea de tiempo y cada lista de un cuadro son una sola parada de `Tab`; dentro se navega con flechas.
4. **Nombres y estados accesibles.** Cada control expone su etiqueta y su estado:

| Elemento | Función accesible | Nombre y estado |
|---|---|---|
| Pestañas | `tablist`, `tab`, `tabpanel` | Etiqueta; `aria-selected` |
| Botón de dos estados | `button` | Etiqueta; `aria-pressed` |
| Deslizador | `slider` | Etiqueta; `aria-valuemin`, `aria-valuemax`, `aria-valuenow` y `aria-valuetext` con el valor tal como se muestra |
| Medidor | `meter` | `mix.level` o `track.inputLevel`; valor en dBFS |
| Paso de la rejilla | `gridcell` con `button` | `a11y.step`: «{sound}, paso {n}» / «{sound}, step {n}»; `aria-pressed` |
| Bloque | `button` | `a11y.block`: «Bloque {pattern}, compases {from} a {to}» / «Block {pattern}, bars {from} to {to}» |
| Clip | `button` | `a11y.clip`: «Clip {name}, empieza en {position}, dura {seconds} s» / «Clip {name}, starts at {position}, lasts {seconds} s» |
| Cuadro de diálogo | `dialog` con `aria-modal` | Su título |
| Barra de estado | `status` | Los mensajes se anuncian al aparecer |

5. **Contraste.** Los colores de §2.1 cumplen 4,5:1 para texto y 3:1 para el resto. No se introducen colores nuevos.
6. **Color.** Ningún estado se comunica solo con color: los activados llevan `✓`, la saturación lleva texto, la selección lleva contorno y los golpes llevan un cuadrado interior.

## 16. Añadidos de la versión 1.1

No se construyen en la 1.0. Se enumeran para que la 1.0 les deje sitio sin mover nada (PD-12).

| Novedad | Dónde aparece | Req. |
|---|---|---|
| Pestaña «Afinador» / «Tuner» (`tab.tuner`, Ctrl+4) | A la derecha de «Mezcla» | RF-800 |
| Botón de dos estados «Pinchazo» / «Punch-in» | Barra de reproducción, a la derecha de «Bucle» | RF-115 |
| Selector «Intensidad» / «Velocity» (Suave, Normal, Fuerte / Soft, Normal, Hard) y deslizador «Swing» | Barra de herramientas de Ritmo, a la derecha de «Compases» | RF-250, RF-251 |
| Golpes de tres niveles: cuadrado interior de 6, 12 y 20 px | Rejilla | RF-250 |
| «Cambiar sonido…» y «Restaurar sonido del kit» | Menú nuevo en la cabecera de cada fila, a la derecha del volumen | RF-252 |
| Tiradores de fundido en las esquinas superiores de un clip | Clips | RF-350 |
| Deslizador «Volumen del clip» / «Clip volume» y botón «Añadir marcador» / «Add marker» | Barra de herramientas de Canción, tras «Eliminar» | RF-352, RF-353 |
| Secciones «Compresor» y «Delay» | Canal de mezcla, entre «Ecualizador» y «Reverb» | RF-450, RF-451 |
| Botón de dos estados «Limitador» / «Limiter» | Canal maestro, sobre el volumen | RF-452 |
| «Empaquetar proyecto…» / «Pack project…» | Menú Archivo, tras «Exportar audio…» | RF-033 |

Los textos y las medidas de estos elementos se fijarán al planificar la 1.1 (09).
