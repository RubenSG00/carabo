# Carabo · Especificación · 10 — Riesgos, supuestos y cuestiones abiertas

> **PROMPT PARA EL AGENTE — léelo antes de seguir**
>
> Eres un agente de programación que trabaja en el repositorio de **Carabo**, un DAW de escritorio para Windows 11. Este documento reúne lo que **puede salir mal** (riesgos), lo que la especificación **da por cierto sin haberlo comprobado** (supuestos) y lo que **aún no está decidido** (cuestiones abiertas).
>
> - Léelo cuando tu tarea figure en la columna «Tareas» de algún riesgo (§1), o cuando te encuentres con algo que no encaja y sospeches que un supuesto es falso (§2).
> - Este documento **no cambia ninguna decisión** de los demás. Si un riesgo se materializa, haz lo que dice su columna «Si ocurre»; si eso no basta, aplica 00 §6 y descríbelo en la solicitud de integración.
> - **Una cuestión abierta no te bloquea nunca.** Cada una tiene un valor por defecto (§3): úsalo. Solo el autor del proyecto puede cambiarlo.
> - No resuelvas un riesgo añadiendo funciones que la especificación no pide.

---

## 1. Riesgos

Probabilidad e impacto: **A** alto, **M** medio, **B** bajo.

### 1.1 Del sonido en un equipo real

| Id | Riesgo | P | I | Qué prevé ya la especificación | Si ocurre | Tareas |
|---|---|---|---|---|---|---|
| RG-01 | **Deriva entre el reloj de la entrada y el de la salida.** El micrófono y los auriculares son dispositivos distintos y sus relojes no van exactamente a la misma velocidad: una toma larga se va desplazando poco a poco respecto a la canción | A | M | RNF-09 solo exige precisión en tomas de hasta 60 segundos. El motor no corrige la deriva (06 §14.2). PA-02 la mide al principio, en el centro y al final de la toma | Si PA-02 no pasa por la deriva y no por el retardo inicial: no se cambia el motor por cuenta propia. Se registra la medida y decide el autor (Q-09) | T-081, T-083 |
| RG-02 | **El retardo de ida y vuelta cambia entre sesiones.** Windows puede cambiar el tamaño de sus búferes, o el usuario puede enchufar el dispositivo en otro puerto, y el valor calibrado deja de valer | M | M | El retardo se guarda por pareja de dispositivos y se puede recalibrar o escribir a mano en cualquier momento (RF-506, RF-507) | Nada en el código. Si en PA-02 dos sesiones dan retardos que difieren en más de 5 ms, se anota en el registro de la prueba para que decida el autor | T-085, T-110 |
| RG-03 | **Dispositivos que no trabajan a 48 000 Hz.** La conversión de frecuencia añade retardo y es una fuente de errores sutiles (tono, clics en los cambios de bloque) | M | M | Conversión en la frontera con pruebas de tono (06 §13.4, 08 §8.2); su retardo queda dentro de la calibración; PA-05 lo prueba con dispositivos reales | Corregir el adaptador. No se pide al usuario que cambie la configuración de Windows (RF-502) | T-080, T-081 |
| RG-04 | **Cortes de audio que el programa no detecta.** En el modo compartido de Windows no siempre hay aviso de un corte, así que el contador puede quedarse corto | M | B | Se cuenta también cuando calcular un bloque tarda más que su duración (06 §13.2). PA-03 exige además que no se oigan cortes | Si en PA-03 se oyen cortes y el indicador dice «Audio: correcto», se registra como defecto de RF-508 | T-080 |
| RG-05 | **La máquina de desarrollo no tiene sonido fiable.** Los agentes no pueden oír lo que construyen; un defecto audible puede pasar todas las pruebas | A | M | El motor se prueba entero sin dispositivo (08 §8); programa `render` (06 §21); puntos de control del autor tras las fases 0, 3, 6, 8 y 10 (09 §3); PA-05 | El autor describe lo que oye; la corrección empieza por escribir una prueba sin dispositivo que reproduzca el defecto | Todas las del motor |

### 1.2 Del motor

| Id | Riesgo | P | I | Qué prevé ya la especificación | Si ocurre | Tareas |
|---|---|---|---|---|---|---|
| RG-06 | **Código de tiempo real en Rust escrito por agentes.** Reservar memoria, usar un cerrojo o provocar un pánico en el hilo de audio produce cortes o cierres difíciles de reproducir | A | A | Reglas RT-01 a RT-10 (06 §4); prueba del asignador de memoria (08 §8.5); Clippy prohíbe `unwrap`, `expect` y `panic!` en `rt/` y `dsp/` (07 §5.5); errores provocados (08 §8.6); tareas pequeñas | No se relaja ninguna prueba. Si una tarea del motor no sale en dos sesiones, se deja abierta y el autor la divide | T-006, T-007, T-033, T-040 a T-047, T-080 a T-085, T-111 |
| RG-07 | **Todo el audio en memoria.** Un proyecto con muchos minutos de audio ocupa mucha memoria; si Windows empieza a paginar, hay cortes, y abrir el proyecto tarda | M | M | Límites de RNF-06 y RNF-07; un archivo que no cabe se trata como «Archivo no encontrado» en lugar de cerrar el programa (06 §9.1); PA-03 lo mide con la carga máxima | No se añade lectura desde disco por cuenta propia. Se registran las medidas y decide el autor (Q-08). El cambio quedaría confinado a `audio_pool.rs` y `rt/audio_track.rs` | T-044, T-045, T-112 |
| RG-08 | **Enviar la instantánea completa en cada cambio.** Durante un arrastre se envía una instantánea entera hasta 30 veces por segundo; con proyectos grandes puede pesar | M | M | Como mucho una cada 33 ms (04 §3.2); preparar una instantánea es barato porque el audio ya está en memoria (06 §5.3); `e2e/performance.spec.ts` y PA-03 | Medir primero dónde se va el tiempo. Se puede optimizar la construcción o la serialización sin cambiar el contrato. Enviar solo diferencias cambiaría AR-02: eso solo lo decide el autor | T-009, T-122 |
| RG-09 | **El motor no llega al rendimiento exigido.** | B | M | Pruebas `perf_…` en la integración continua (08 §8.7) | Optimizar conservando las huellas y la independencia del tamaño de bloque (T-112). No se cambian las fórmulas de 06 | T-112 |
| RG-10 | **El codificador de MP3 no compila en Windows.** LAME se compila desde código C | M | B | El codificador está aislado tras una interfaz propia; alternativa prevista (07 §11.3) | Aplicar la alternativa de 07 §11.3 y anotarlo. Si tampoco sirve, 00 §6 y decide el autor (Q-07) | T-100 |

### 1.3 De las herramientas

| Id | Riesgo | P | I | Qué prevé ya la especificación | Si ocurre | Tareas |
|---|---|---|---|---|---|---|
| RG-11 | **Las bibliotecas no son como el agente las recuerda.** `cpal` 0.18, `rubato` 5, Tauri 2, TypeScript 6, Vite 8 y Vitest 5 han cambiado nombres y tipos respecto a versiones anteriores | A | M | Se consulta la documentación de la versión fijada, no la memoria (07 §2.1, regla 4); la especificación fija comportamientos, no firmas de bibliotecas (06, 07) | Seguir la documentación de la versión instalada. Si una función ya no existe y cambia el comportamiento exigido, 00 §6 | T-001, T-002, T-006, T-008, T-070, T-080 |
| RG-12 | **Una línea de versiones deja de existir o de mantenerse** antes de terminar el proyecto | B | M | Los archivos de bloqueo fijan versiones exactas (07 §2.1) | No se cambia de línea por cuenta propia: 00 §6 y decide el autor | T-001, T-002 |
| RG-13 | **El arrastre de archivos o las funciones de ventana de Tauri se comportan distinto de lo descrito** (posición del archivo soltado, cierre de la ventana, título) | M | B | Todo ello está detrás del puente (04 §6) y se prueba con el puente simulado; PA-06 lo prueba en la aplicación real | Corregir solo `tauriBridge.ts`. Lo que no se pueda comprobar sin la aplicación real se anota en la solicitud de integración | T-051, T-071 |
| RG-14 | **La prueba de fluidez es inestable en la integración continua**, que no tiene tarjeta gráfica | M | B | El criterio usa la mediana y un máximo holgado (08 §6); PA-03 lo confirma en el equipo de referencia | No se relaja el criterio. Si la prueba falla solo en la integración continua y pasa en la máquina de desarrollo, 00 §6 con las medidas de ambos sitios | T-122 |
| RG-15 | **Windows advierte al ejecutar el programa en otro equipo**, porque el ejecutable no está firmado | A | B | No hay firma ni instalador, por decisión (DEC-20). Es una herramienta personal | Nada. El usuario acepta el aviso de Windows | T-123 |

### 1.4 Del producto y del proceso

| Id | Riesgo | P | I | Qué prevé ya la especificación | Si ocurre | Tareas |
|---|---|---|---|---|---|---|
| RG-16 | **La prueba de los cinco minutos no se supera.** El proyecto empieza vacío y sin guías (DEC-08): alguien nuevo puede no saber por dónde empezar | M | A | Cada control tiene su nombre escrito y su explicación; los controles desactivados explican por qué (RF-008); una acción, un lugar (PD-05); la pestaña inicial es Ritmo con un único botón activo (RF-005) | No se añaden guías ni asistentes por cuenta propia. Se registra dónde se atascó cada participante y decide el autor (Q-06) | T-125 |
| RG-17 | **Los sonidos sustitutos suenan pobres** y dan mala impresión en PA-01 | A | M | Son provisionales; cambiarlos es sustituir 24 archivos, sin tocar código (05 §8.4) | Resolver Q-01 antes de T-125 | T-005, T-125 |
| RG-18 | **El tamaño del proyecto.** Son 89 tareas para la versión 1.0, con límites de uso de los agentes | A | M | La fase 0 deja algo que suena; cada fase deja algo utilizable (09 §3); la 1.1 está separada | El autor puede parar al final de cualquier fase. No se recorta el alcance de una tarea para acabar antes | Todas |
| RG-19 | **Huecos o contradicciones en la especificación**, que un agente resuelve adivinando | M | M | Regla de huecos de 00 §6 y registro en `docs/DECISIONES-DE-IMPLEMENTACION.md`, que el autor lee en cada solicitud de integración | Aplicar 00 §6. Nunca resolverlo en silencio | Todas |
| RG-20 | **Revisión por una sola persona** que no domina Rust ni TypeScript: un defecto puede pasar la revisión | M | M | La comprobación completa y la integración continua son la primera barrera; cada solicitud explica cómo se ha comprobado cada criterio (07 §10.1); trazabilidad automática (08 §13.3) | El autor puede pedir a otro agente una revisión de la solicitud antes de fusionarla | Todas |
| RG-21 | **Pérdida de trabajo del usuario** por un fallo al guardar o al recuperar | B | A | Escritura atómica (RNF-13); copia de recuperación (RF-026, RF-027); nada borra audio fuera de la limpieza al abrir (RF-029); pruebas de todo ello (08 §11) | Se trata como el defecto más grave: se corrige antes que cualquier otra tarea | T-050 a T-055 |

## 2. Supuestos

Cosas que la especificación da por ciertas. Si descubres que una es falsa, no la sortees en silencio: 00 §6 y aviso en la solicitud de integración.

| Id | Supuesto | Qué pasa si es falso |
|---|---|---|
| SUP-01 | El equipo de uso es, como mínimo, el de referencia de RNF-02: 4 núcleos, 8 GB de memoria, disco SSD, pantalla de 1920 × 1080 y Windows 11 de 64 bits | Los requisitos de rendimiento pueden no cumplirse; con menos memoria se agrava RG-07 |
| SUP-02 | El equipo de sonido es un micrófono USB y unos auriculares, dos dispositivos distintos, con el sonido estándar de Windows (DEC-19) | Con una interfaz de audio profesional o con ASIO, el programa funciona a través de Windows pero no aprovecha su baja latencia |
| SUP-03 | Windows 11 incluye el componente WebView2 | La aplicación no arranca. No se incluye ningún instalador de WebView2 |
| SUP-04 | Los dispositivos de audio trabajan en modo compartido a 44 100 o a 48 000 Hz, con muestras de 16 bits o de coma flotante | Otras frecuencias se convierten igual (06 §13.4); un formato de muestra que `cpal` no ofrezca impediría abrir el dispositivo: se trataría como «sin dispositivo» |
| SUP-05 | Un uso normal son canciones de 3 a 5 minutos con hasta unas 8 pistas de audio: menos de 500 MB de audio en memoria | Con mucho más audio, RG-07 |
| SUP-06 | Solo hay un usuario y una sola copia del programa abierta a la vez. No hay bloqueo de proyectos | Dos copias con el mismo proyecto abierto pueden pisarse al guardar |
| SUP-07 | Los proyectos están en un disco local. No en una carpeta de red ni en una carpeta sincronizada con la nube | La escritura atómica y la limpieza de archivos pueden comportarse de otro modo; un sincronizador puede bloquear archivos mientras se graba |
| SUP-08 | El repositorio es público en GitHub (DEC-25, DEC-34) | Con un repositorio privado, la integración continua en Windows consume minutos de pago, y el cumplimiento de la licencia de LAME (07 §11.3) deja de apoyarse en que el código es público |
| SUP-09 | El autor revisa y fusiona cada solicitud de integración y ejecuta las pruebas de aceptación manuales. No domina Rust ni TypeScript | Sin revisión, RG-19 y RG-20 se agravan |
| SUP-10 | Los agentes son sesiones de Claude Code que empiezan sin memoria de las anteriores, siguen los documentos al pie de la letra y no pueden preguntar | Si pudieran preguntar, nada cambia: la especificación sigue siendo la referencia |
| SUP-11 | La máquina de desarrollo es una máquina virtual con Windows 11 que puede compilar y ejecutar la aplicación, aunque sin sonido fiable | Si no pudiera ejecutar la aplicación, los criterios que piden abrir la ventana se comprobarían solo en los puntos de control del autor |
| SUP-12 | La licencia LGPL de LAME es aceptable para el autor (07 §11.3) | Q-07 |
| SUP-13 | Las versiones de 07 §2 siguen publicadas cuando se construya el programa | RG-12 |

## 3. Cuestiones abiertas

Cada cuestión tiene un **valor por defecto**, que es el que se usa mientras el autor no diga otra cosa. Ninguna bloquea una tarea.

| Id | Cuestión | Valor por defecto | Quién decide y cuándo | Qué cambia si se decide otra cosa |
|---|---|---|---|---|
| Q-01 | ¿Qué sonidos definitivos llevan los tres kits? Deben ser de dominio público (CC0) (DEC-33) | Los sonidos sustitutos de 05 §8.4 | El autor, antes de T-125 | Se sustituyen los 24 archivos de `assets/kits/` conservando sus nombres, se ajusta la prueba que los compara con el generador (08 §10) y se regeneran las huellas (08 §8.3) |
| Q-02 | ¿Qué icono definitivo lleva el programa? | El icono provisional de 07 §13.5 | El autor, antes de T-125 | Se sustituye `assets/icon.png` y se regeneran los de `src-tauri/icons/` |
| Q-03 | ¿Qué dirección del repositorio muestra «Acerca de»? | La del remoto `origin` al hacer T-004; si no hay, ninguna (07 §7) | El autor, cuando quiera | Cambia la constante `REPOSITORY_URL` |
| Q-04 | ¿El nombre se escribe «Carabo» o «Cárabo»? | «Carabo», sin tilde | El autor, antes de T-125 | Cambian `APP_NAME`, `productName` y el título de `tauri.conf.json`, y los textos de `src/i18n/` y de `README.md` que nombran el programa. No cambian el identificador `com.carabo.app`, el nombre del ejecutable, el del repositorio ni la extensión `.carabo` |
| Q-05 | ¿Se mantiene el identificador `com.carabo.app`? | Sí | El autor, antes de T-002 | Cambia `identifier` en `tauri.conf.json` y, con él, las carpetas de datos de 05 §7. Cambiarlo después de haber usado el programa deja atrás los ajustes y los proyectos sin guardar |
| Q-06 | Si PA-01 no se supera, ¿se mantiene el proyecto vacío sin guías (DEC-08)? | Sí: no se añade ninguna guía | El autor, a la vista del registro de PA-01 | Habría que especificar las ayudas en 02 y 03 antes de construirlas |
| Q-07 | ¿Se exporta MP3 con LAME (licencia LGPL) o solo WAV? | MP3 con LAME (DEC-31, 07 §11.3) | El autor, antes de T-100 | Sin MP3: se eliminan T-100 y la opción MP3 de RF-600 y RF-601 |
| Q-08 | ¿Hace falta leer el audio desde el disco en lugar de cargarlo en memoria? | No: todo en memoria (06 §9.1) | El autor, tras PA-03 y tras un tiempo de uso real | Un cambio del motor confinado a `audio_pool.rs` y `rt/audio_track.rs`, que habría que especificar antes |
| Q-09 | ¿Hace falta corregir la deriva de relojes en tomas largas? | No (06 §14.2) | El autor, tras PA-02 | Habría que especificar cómo se estima la deriva y cómo se ajusta la toma al terminar |
| Q-10 | ¿Quiénes son los dos participantes de PA-01? | Sin decidir. El autor no puede ser uno de ellos (08 §12) | El autor, antes de T-125 | Nada en el código |

## 4. Qué vigilar en cada fase

| Fase (09 §3) | Riesgos que tocan |
|---|---|
| 0 · Esqueleto | RG-05, RG-06, RG-11, RG-12 |
| 1 · Proyecto en memoria | RG-19 |
| 2 · Interfaz base | RG-11, RG-19 |
| 3 · Ritmo | RG-05, RG-06 |
| 4 · Motor: reproducción completa | RG-06, RG-07, RG-09 |
| 5 · Archivos de proyecto | RG-13, RG-21 |
| 6 · Canción | RG-08 |
| 7 · Audio: importación y clips | RG-07, RG-11, RG-13 |
| 8 · Grabación y dispositivos | RG-01 a RG-06 |
| 9 · Mezcla | RG-08 |
| 10 · Exportación | RG-10 |
| 11 · Ajustes y robustez | RG-02, RG-06, RG-09 |
| 12 · Cierre de la 1.0 | RG-14 a RG-18 |
