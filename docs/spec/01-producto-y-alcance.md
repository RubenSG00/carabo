# Carabo · Especificación · 01 — Producto y alcance

> **PROMPT PARA EL AGENTE — léelo antes de seguir**
>
> Eres un agente de programación que trabaja en el repositorio de **Carabo**, un DAW de escritorio para Windows 11. Este documento explica **para quién es el programa, qué debe conseguir y qué entra y qué no entra** en cada versión.
>
> - No hace falta leerlo entero en cada tarea: lee las secciones que tu tarea cite.
> - Lee §4 (principios de diseño) cada vez que tengas que decidir algo que los demás documentos no resuelven (00 §6).
> - Lee §6 (alcance) cada vez que dudes de si algo hay que construirlo. Si no está en la lista de tu versión, no lo construyas.
>
> Este documento no describe cómo se implementa nada. Los requisitos detallados están en `02-requisitos.md`; el significado de los términos, en `00-indice.md` §7.

---

## 1. Resumen y visión

Carabo es un DAW (estación de trabajo de audio digital) de escritorio para Windows 11. Su virtud principal es resultar **intuitivo**: una persona que toca un instrumento debe poder montar un ritmo, grabarse encima y escuchar el resultado en menos de cinco minutos, sin leer nada.

Lo que el usuario hace con Carabo:

1. **Monta una base rítmica** marcando golpes en una rejilla de pasos (un patrón) con sonidos de batería o percusión.
2. **Coloca esos patrones** a lo largo de la canción.
3. **Graba audio encima** con un micrófono: guitarra, voz, palmas.
4. **Mezcla**: ajusta el volumen y el panorama de cada pista y le pone ecualización y reverb.
5. **Exporta** la canción a un archivo WAV o MP3.

Carabo es la herramienta personal de su autor y se publica como código abierto. No compite en funciones con los DAW comerciales: hace pocas cosas y aspira a que cada una se entienda a la primera.

## 2. Usuario objetivo

El **usuario de referencia** es una sola persona tipo, y todo el diseño se decide pensando en ella:

- Toca un instrumento (guitarra) y canta, a nivel aficionado.
- Ha usado algún DAW de forma básica (FL Studio) y conoce el vocabulario: pista, patrón, mezclador, clip.
- No tiene formación en ingeniería de sonido.
- Usa un ordenador con Windows 11, un micrófono USB y unos auriculares.
- Quiere capturar ideas y canciones sobre un ritmo, no producir a nivel profesional.

Dos cosas le frenaron en otros DAW, y Carabo debe evitarlas:

| Freno | Qué significa | Cómo responde Carabo |
|---|---|---|
| **No saber por dónde empezar** | Pantallas llenas de botones y ventanas, sin un primer paso evidente | Una pestaña por herramienta con pocos controles (PD-02), todos con su nombre escrito (PD-07), y el transporte siempre a la vista (PD-03) |
| **Editar con miedo** | Cortar o mover resulta torpe, o se estropea algo y no se sabe volver atrás | Todo se puede deshacer en varios pasos y nada destruye audio (PD-08) |

## 3. Casos de uso

Cada caso de uso describe una tarea completa del usuario. Los pasos detallados de cada interacción están en `03-interfaz.md`; aquí solo se fija qué debe ser posible.

### CU-01 · Crear una base rítmica [1.0]

- **Situación inicial:** proyecto vacío recién creado, pestaña Ritmo.
- **Flujo:**
  1. El usuario crea una pista de ritmo. Aparece la rejilla de pasos con un patrón vacío.
  2. Carga un ritmo hecho o marca golpes a mano.
  3. Pulsa Reproducir y el patrón suena en bucle.
  4. Cambia el tempo y oye el cambio al momento.
- **Resultado:** existe una pista de ritmo con un patrón que suena, y ese patrón ya está colocado en la canción.
- **Requisitos:** RF-201 a RF-213, RF-101, RF-102, RF-106.

### CU-02 · Grabar una toma sobre la base [1.0]

- **Situación inicial:** existe una base rítmica; hay un micrófono conectado.
- **Flujo:**
  1. En la pestaña Canción, el usuario crea una pista de audio, que queda armada.
  2. Pulsa Grabar. Suena un compás de cuenta atrás y después la base.
  3. Toca o canta. Pulsa Parar.
  4. Aparece un clip con la grabación, colocado en su sitio aunque el equipo tenga retardo.
  5. Pulsa Reproducir y oye la base y la grabación juntas.
- **Resultado:** una toma grabada y sincronizada con la base.
- **Requisitos:** RF-301, RF-310 a RF-314, RF-506, RF-507.

### CU-03 · Repetir un pasaje que salió mal [1.0]

- **Situación inicial:** hay una toma grabada.
- **Flujo:**
  1. El usuario coloca el cursor antes del pasaje y vuelve a grabar en la misma pista.
  2. La toma nueva tapa a la anterior en el tramo en que coinciden.
  3. Si la nueva es peor, la borra o la deshace y la anterior reaparece intacta.
- **Resultado:** el pasaje suena con la toma elegida y no se ha perdido ninguna grabación.
- **Requisitos:** RF-309, RF-313, RF-321, RF-030.

### CU-04 · Dar estructura a la canción con varios patrones [1.0]

- **Situación inicial:** existe una pista de ritmo con un patrón.
- **Flujo:**
  1. En la pestaña Ritmo, el usuario crea un segundo patrón (por ejemplo, para el estribillo).
  2. En la pestaña Canción, añade bloques de uno y otro patrón, los mueve y los alarga para que se repitan.
  3. Más tarde cambia un golpe del primer patrón y todos sus bloques cambian a la vez.
- **Resultado:** la canción alterna patrones a lo largo del tiempo.
- **Requisitos:** RF-208, RF-307, RF-308.

### CU-05 · Importar un audio y repetirlo [1.0]

- **Situación inicial:** el usuario tiene un archivo WAV o MP3.
- **Flujo:**
  1. Lo arrastra a la pestaña Canción o usa «Importar audio…».
  2. Aparece un clip con ese audio.
  3. Lo recorta, lo parte o activa «Repetir» y lo alarga para que su contenido se repita.
- **Resultado:** el audio importado forma parte de la canción. El archivo original no se ha modificado.
- **Requisitos:** RF-315 a RF-321.

### CU-06 · Mezclar [1.0]

- **Situación inicial:** la canción tiene varias pistas.
- **Flujo:**
  1. En la pestaña Mezcla, el usuario ajusta el volumen y el panorama de cada pista mientras suena.
  2. Silencia una pista o deja una sola en solo para oírla aislada.
  3. Activa el ecualizador o la reverb de una pista y ajusta sus controles.
  4. Vigila los medidores y baja el volumen maestro si avisa de saturación.
- **Resultado:** la mezcla suena equilibrada y no satura.
- **Requisitos:** RF-400 a RF-408.

### CU-07 · Exportar la canción [1.0]

- **Flujo:**
  1. El usuario elige Archivo > Exportar audio…, el formato (WAV o MP3) y dónde guardarlo.
  2. El programa genera el archivo mostrando el progreso.
- **Resultado:** un archivo de audio que suena igual que la canción al reproducirla, sin la claqueta.
- **Requisitos:** RF-600 a RF-603.

### CU-08 · Guardar, retomar y recuperar [1.0]

- **Flujo:**
  1. El usuario guarda el proyecto con nombre. Se crea una carpeta de proyecto.
  2. Otro día lo abre desde la lista de recientes y lo encuentra como lo dejó.
  3. Si el programa se cierra de golpe con cambios sin guardar, al volver a abrirlo se le ofrece la copia de recuperación.
- **Resultado:** el trabajo persiste solo cuando el usuario guarda, y un fallo no hace perder más de dos minutos de cambios ni ninguna grabación terminada.
- **Requisitos:** RF-020 a RF-029.

### CU-09 · Afinar la guitarra [1.1]

- **Flujo:** en la pestaña Afinador, el usuario toca una cuerda y ve la nota detectada y cuánto se desvía.
- **Requisitos:** RF-800, RF-801.

## 4. Principios de diseño

Estos principios son la definición operativa de «intuitivo». Son reglas comprobables: cada una dice cómo se verifica. Cuando un documento no resuelva un caso, decide con ellos.

### PD-01 · La prueba de los cinco minutos manda

Carabo DEBE superar la prueba descrita en §5. Si un diseño la hace fallar, se cambia el diseño, aunque cumpla todo lo demás.

- **Cómo se comprueba:** ejecutando la prueba de aceptación PA-01 (08).

### PD-02 · Una pestaña por herramienta

Cada herramienta ocupa toda la zona de contenido de la ventana. NO DEBE haber ventanas flotantes, paneles acoplables, vistas divididas ni modos dentro de una pestaña. Solo se admiten cuadros de diálogo para: pantalla de inicio, abrir y guardar archivos, guardar cambios pendientes, exportar, ajustes, avisos con consecuencias y errores.

- **Cómo se comprueba:** en todo momento hay exactamente una pestaña visible, y el programa nunca crea una segunda ventana del sistema aparte de los cuadros nativos de archivo.

### PD-03 · Reproducir, parar y grabar siempre a la vista

La barra de reproducción DEBE verse en todas las pestañas y en todos los estados, salvo mientras hay un cuadro de diálogo abierto.

- **Cómo se comprueba:** recorriendo todas las pestañas; los controles de Reproducir y Grabar están siempre en la misma posición.

### PD-04 · Funciona sin configurar nada

Si Windows tiene un dispositivo de salida y uno de entrada por defecto, el usuario DEBE poder reproducir y grabar sin abrir Ajustes.

- **Cómo se comprueba:** con el archivo de ajustes borrado, se completa CU-01 y CU-02 sin abrir la ventana de ajustes.

### PD-05 · Una acción, un lugar

Cada acción del programa tiene **un único control visible** en toda la aplicación. Puede tener, además, un atajo de teclado y, cuando se trata de manipulación directa (arrastrar un elemento o un borde), un gesto equivalente. NO DEBE haber dos botones, dos menús o dos pestañas que hagan lo mismo.

- **Cómo se comprueba:** en la tabla de acciones del documento 03, cada acción aparece una sola vez con un único control.

### PD-06 · Casi ninguna preferencia

Lo único que el usuario puede configurar es: el idioma, el dispositivo de salida, el dispositivo de entrada y el retardo de grabación (calibrado o escrito a mano). NO DEBE existir ninguna otra opción, preferencia, tema o personalización.

- **Cómo se comprueba:** la ventana de ajustes contiene solo esos cuatro elementos (más la información «Acerca de»).

### PD-07 · Vocabulario estándar y texto visible

Los nombres son los habituales en producción musical (pista, patrón, mezclador, panorama, reverb). Todo botón, deslizador, campo y menú DEBE mostrar su nombre en texto; un icono solo no basta. Cada control DEBE tener una explicación de una frase que aparece al dejar el ratón encima o al recibir el foco del teclado.

- **Cómo se comprueba:** inspección de cada pestaña; no hay controles identificados solo por un icono, y todos muestran su explicación.

### PD-08 · Nada se rompe sin vuelta atrás

- Todo cambio del proyecto DEBE poder deshacerse y rehacerse.
- Ninguna operación de edición modifica ni borra un archivo de audio.
- Cuando dos elementos se solapan, uno tapa al otro en lugar de sustituirlo.
- Como todo se deshace, las acciones de edición NO DEBEN pedir confirmación. Solo se pide confirmación cuando se van a perder cambios sin guardar o una copia de recuperación.

- **Cómo se comprueba:** tras cualquier secuencia de ediciones, pulsar Deshacer tantas veces como ediciones devuelve el proyecto al estado inicial exacto.

### PD-09 · El usuario decide cuándo se guarda

El programa escribe el archivo de proyecto solo cuando el usuario elige Guardar o Guardar como. La copia de recuperación es un archivo distinto y nunca sustituye al proyecto sin que el usuario lo acepte.

- **Cómo se comprueba:** tras editar sin guardar y cerrar eligiendo «No guardar», el archivo de proyecto es idéntico byte a byte al anterior.

### PD-10 · Los errores dicen qué pasa y qué hacer

Todo mensaje de error DEBE decir, en este orden: qué ha pasado y qué puede hacer el usuario. Cuando exista una acción que lo resuelva, el mensaje DEBE incluir un botón que la ejecute. NO DEBEN mostrarse códigos internos ni textos técnicos del sistema.

- **Cómo se comprueba:** cada mensaje del catálogo del documento 03 tiene las dos partes.

### PD-11 · Sin contenido precargado ni guías

Un proyecto nuevo está vacío. NO DEBE haber proyectos de ejemplo que se abran solos, asistentes, recorridos guiados ni indicaciones especiales para el estado vacío. Los controles habituales, con su nombre escrito, son la única guía.

- **Cómo se comprueba:** un proyecto nuevo no contiene ninguna pista y la interfaz no muestra ningún elemento que no exista también con el proyecto lleno.

### PD-12 · Las pestañas no crecen

Una herramienta nueva entra como pestaña nueva. Una función nueva de una herramienta existente se añade dentro de su pestaña sin mover ni cambiar el comportamiento de los controles que ya había.

- **Cómo se comprueba:** al implementar algo [1.1], ningún control [1.0] cambia de posición, nombre ni comportamiento.

## 5. La prueba de los cinco minutos

Es el criterio de aceptación de la versión 1.0 y la definición de «intuitivo» (PD-01). El protocolo para ejecutarla y registrar el resultado está en el documento 08 (PA-01); aquí se fija su contenido.

- **Participante:** una persona que toca un instrumento, ha usado alguna vez un DAW sencillo, no ha visto nunca Carabo y no ha leído nada sobre él.
- **Preparación:** Carabo recién arrancado en la pantalla de inicio; micrófono y auriculares ya conectados y funcionando en Windows. No se le da ninguna explicación del programa.
- **Encargo que se le lee:** «Monta un ritmo, grábate encima y enséñame cómo suena todo junto. Después, mándame la canción».

**En menos de 5 minutos**, sin ayuda, el participante DEBE haber conseguido:

1. que suene un ritmo;
2. cambiarle el tempo;
3. grabar su instrumento o su voz encima;
4. escuchar las dos cosas juntas.

**En menos de 15 minutos** desde el principio, además:

5. cambiar golpes del ritmo;
6. equilibrar el volumen de la base y de la grabación;
7. exportar un archivo de audio.

La prueba se supera si el participante completa los siete pasos dentro de los dos plazos sin recibir ninguna indicación.

## 6. Alcance

### 6.1 Versión 1.0 — lo imprescindible

La 1.0 debe servir por sí sola para los casos de uso CU-01 a CU-08.

**Aplicación y proyecto**
- Ventana única con barra superior, barra de reproducción, tres pestañas y barra de estado (RF-001 a RF-010).
- Pantalla de inicio; proyecto nuevo, abrir, recientes, guardar y guardar como (RF-020 a RF-025).
- Copia de recuperación y recuperación tras un cierre inesperado (RF-026, RF-027).
- Deshacer y rehacer de varios pasos para todo cambio del proyecto (RF-030 a RF-032).

**Transporte**
- Reproducir y parar; modo patrón y modo canción según la pestaña (RF-100 a RF-105).
- Un tempo y un compás por proyecto; claqueta; bucle (RF-106 a RF-114).

**Pestaña Ritmo**
- Rejilla de pasos de 8 filas; patrones de 1, 2 o 4 compases (RF-204 a RF-207).
- Tres kits incluidos, de 8 sonidos cada uno (RF-203).
- Quince ritmos hechos (RF-209).
- Varios patrones por pista: crear, duplicar, renombrar y eliminar (RF-208).
- Volumen y silencio por fila (RF-210).

**Pestaña Canción**
- Pistas de ritmo y de audio: crear, renombrar, reordenar y eliminar (RF-301, RF-302).
- Bloques de patrón: añadir, mover, alargar para repetir, duplicar y eliminar (RF-307, RF-308).
- Grabación de audio en una pista cada vez, con cuenta atrás y claqueta (RF-310 a RF-314).
- Importación de WAV y MP3 (RF-315).
- Edición de clips: mover, recortar los extremos, repetir, partir, duplicar y eliminar, con ajuste a la rejilla (RF-316 a RF-321).
- Microfundidos automáticos (RF-322).
- Zoom, desplazamiento y bucle (RF-305, RF-111).

**Pestaña Mezcla**
- Por pista: volumen, panorama, silenciar, solo y medidor con aviso de saturación (RF-401 a RF-404).
- Canal maestro con volumen y medidor (RF-405).
- Ecualizador de tres bandas y reverb por pista (RF-406, RF-407).

**Audio y archivos**
- Uso de los dispositivos por defecto de Windows y selección de otros (RF-500 a RF-505).
- Calibración del retardo de grabación (RF-506, RF-507).
- Exportación a WAV y MP3 (RF-600 a RF-603).
- Ajustes: idioma, dispositivos y retardo (RF-700 a RF-704).

### 6.2 Versión 1.1 — lo deseable

Se especifica entera en estos documentos, pero se construye solo cuando la 1.0 está terminada y ha superado PA-01.

- Intensidad por golpe en tres niveles y swing por patrón (RF-250, RF-251).
- Sonidos propios en una fila (RF-252).
- Fundidos ajustables y fundido cruzado entre clips (RF-350, RF-351).
- Volumen por clip y marcadores de sección (RF-352, RF-353).
- Pinchazo: regrabar solo un tramo marcado (RF-115).
- Compresor y delay por pista; limitador en el canal maestro (RF-450 a RF-452).
- Pestaña Afinador (RF-800, RF-801).
- Empaquetar el proyecto en un solo archivo comprimido (RF-033).

### 6.3 Versiones posteriores — solo esbozado

Nada de esto se construye en la 1.0 ni en la 1.1. Se enumera para que la arquitectura no le cierre el paso.

**Versión 2.0: instrumentos virtuales y MIDI**
- Pistas de instrumento con editor de notas (*piano roll*) y cuantización.
- Instrumentos incluidos: un reproductor de muestras afinadas, un sintetizador sencillo y un reproductor de bibliotecas SoundFont.
- Teclados y controladores MIDI; grabación de notas.

**Más adelante, sin versión asignada**
- Tomas apiladas en una pista para elegir los mejores trozos; grabación en bucle.
- Loops de audio ajustados al tempo y cambio de velocidad sin cambio de tono.
- Cambios de tempo y de compás a lo largo de la canción.
- Grabación de varias pistas a la vez.
- Automatización de volumen y de efectos.
- Buses y envíos.
- Simulador de amplificador de guitarra y escucha de la entrada con baja latencia.
- Selección de varios elementos a la vez; mover elementos entre pistas.
- Otros sistemas operativos; instalador.

**Lo que la 1.0 deja preparado para ello**

| Preparación | Dónde |
|---|---|
| El tipo de pista es un valor extensible (`"rhythm"`, `"audio"`), no un booleano | 05 |
| Cada paso de un patrón guarda un nivel de 0 a 3, aunque la 1.0 solo use 0 y 2 | 05 |
| Los efectos de un canal son una lista ordenada de unidades con identificador, no campos sueltos | 05, 06 |
| En el motor, lo que genera sonido en una pista está detrás de una interfaz común | 06 |
| El archivo de proyecto y la instantánea del motor llevan número de versión de formato | 05 |

### 6.4 Fuera de alcance

Esto **no se construirá**. Si una tarea parece pedirlo, la tarea está mal leída.

- Plugins de terceros en cualquier formato (VST3, CLAP, AU, LV2).
- Controladores ASIO y cualquier modo de audio exclusivo.
- Dejar oír la entrada del micrófono a través del programa en las versiones 1.x.
- MIDI de cualquier tipo en las versiones 1.x.
- Versiones para web, móvil o tableta.
- Cuentas de usuario, sincronización en la nube, colaboración en línea y cualquier conexión de red.
- Telemetría, informes automáticos de errores y comprobación de actualizaciones.
- Vídeo, notación musical y partituras.
- Atajos de teclado personalizables, temas de color, disposición de pantalla configurable.
- Ventanas flotantes, paneles acoplables y vistas divididas.
- Guardado automático sobre el archivo de proyecto.
- Proyectos de ejemplo, asistentes de bienvenida y tutoriales dentro del programa.
- Frecuencias de muestreo de proyecto distintas de 48 000 Hz.
- Más de 16 pistas o canciones de más de 10 minutos.
- Instalador, firma de código y actualizaciones automáticas.

## 7. Plataforma y distribución

- **Sistema operativo:** Windows 11 de 64 bits (x86-64). No se prueba ni se promete ningún otro.
- **Forma de la aplicación:** programa de escritorio con una sola ventana.
- **Distribución:** el ejecutable y los recursos que produce la orden de compilación (07). No hay instalador, ni firma, ni actualizaciones automáticas.
- **Dónde se usa:** en el equipo real del autor, con micrófono USB y auriculares.
- **Dónde se programa:** en una máquina virtual con Windows 11. En ella el sonido pasa por un dispositivo emulado, así que **ninguna prueba automática puede depender de un dispositivo de sonido** y la escucha real la hace el autor en su equipo (08).
- **Preferencia para decisiones futuras:** entre dos soluciones equivalentes, se elige la que no ata el código a Windows, para no encarecer un posible paso a otros sistemas.

## 8. Licencia y contenidos

- **Código:** licencia MIT. El archivo `LICENSE` está en la raíz del repositorio.
- **Dependencias:** solo se admiten bibliotecas cuya licencia permita distribuir el programa bajo MIT. La lista de licencias admitidas y el caso particular del codificador de MP3 están en el documento 07.
- **Sonidos de los kits:** DEBEN ser de dominio público (CC0). Mientras el autor no elija los definitivos, el repositorio incluye sonidos sustitutos generados por un script propio, que no tienen ninguna restricción (05, 10).
- **Precio:** el programa es gratuito. No hay compras, licencias de uso ni publicidad.
