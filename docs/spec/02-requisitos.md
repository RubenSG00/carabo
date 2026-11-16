# Carabo · Especificación · 02 — Requisitos

> **PROMPT PARA EL AGENTE — léelo antes de seguir**
>
> Eres un agente de programación que trabaja en el repositorio de **Carabo**, un DAW de escritorio para Windows 11. Este documento dice **qué debe hacer el programa**: los requisitos funcionales (RF) y no funcionales (RNF), cada uno con criterios de aceptación comprobables.
>
> - Tu tarea del plan (`09-plan-de-implementacion.md`) cita los RF que debe cumplir. Lee esos RF completos y la sección «Contexto» del capítulo al que pertenecen.
> - Un requisito está cumplido solo cuando se cumplen **todos** sus criterios de aceptación (`CA1`, `CA2`…). Cada criterio debe quedar cubierto por una prueba (08).
> - No implementes requisitos marcados **[1.1]** salvo que tu tarea lo pida expresamente.
> - Este documento manda sobre los demás en cuanto a comportamiento (00 §5). Cómo se ve cada cosa está en 03; cómo se construye, en 04, 05 y 06.
> - Los términos propios del programa (bloque, clip, patrón, cursor, tapar…) tienen el significado exacto del glosario (00 §7). Los nombres en `MAYÚSCULAS` son constantes cuyo valor está en 00 §4.

---

## 1. Cómo leer un requisito

Cada requisito tiene esta forma:

> **RF-nnn [versión] · Título**
> Enunciado: qué debe ocurrir, con palabras normativas (DEBE, NO DEBE, PUEDE).
> **Criterios de aceptación:** lista `CA1…CAn` de comprobaciones concretas.

- La etiqueta de versión es **[1.0]** o **[1.1]** (00 §3.2).
- «Aviso» significa un mensaje en la barra de estado; «diálogo de error», un cuadro que exige respuesta. Ambos se definen en RF-007 y sus textos exactos están en el catálogo del documento 03.
- «Un paso de historial» significa que la operación se deshace y se rehace como una unidad (RF-030).

Numeración por áreas:

| Números reservados | Área | Capítulo |
|---|---|---|
| 001–019 | Aplicación | §2 |
| 020–049 | Proyecto e historial | §3 |
| 100–129 | Transporte | §4 |
| 200–299 | Pestaña Ritmo | §5 |
| 300–399 | Pestaña Canción | §6 |
| 400–499 | Pestaña Mezcla | §7 |
| 500–529 | Audio y dispositivos | §8 |
| 600–619 | Exportación | §9 |
| 700–719 | Ajustes, idioma y atajos | §10 |
| 800–809 | Pestaña Afinador | §11 |

No todos los números de un rango están usados: solo existen los requisitos que aparecen escritos en su capítulo. Dentro de cada rango, los requisitos [1.1] empiezan en la decena 50 (RF-250, RF-350, RF-450), salvo RF-033 y RF-115.

---

## 2. Aplicación

**Contexto.** Carabo tiene una sola ventana. De arriba abajo contiene la barra superior (menú Archivo, Deshacer, Rehacer, pestañas y Ajustes), la barra de reproducción, la zona de la pestaña activa y la barra de estado. Hay tres pestañas: Ritmo, Canción y Mezcla.

**RF-001 [1.0] · Ventana única**
Carabo DEBE tener una sola ventana principal, redimensionable, con un tamaño mínimo de `WINDOW_MIN_WIDTH` × `WINDOW_MIN_HEIGHT` píxeles. El título de la ventana DEBE ser `<nombre del proyecto> — Carabo`, con un asterisco tras el nombre cuando haya cambios sin guardar (RF-025). Un proyecto que nunca se ha guardado se llama «Sin título».
**Criterios de aceptación:**
- CA1: la ventana no se puede hacer más pequeña que 1280 × 720.
- CA2: con el proyecto «Demo» guardado, el título es `Demo — Carabo`; tras un cambio, `Demo* — Carabo`; con un proyecto nuevo, `Sin título — Carabo`.
- CA3: el programa no abre ninguna otra ventana del sistema, salvo los cuadros nativos de abrir, guardar y elegir carpeta.

**RF-002 [1.0] · Estructura de la ventana**
La ventana DEBE mostrar, de arriba abajo: barra superior, barra de reproducción, zona de pestaña (ocupa todo el espacio restante) y barra de estado. La barra superior, la de reproducción y la de estado DEBEN ser idénticas en todas las pestañas.
**Criterios de aceptación:**
- CA1: al cambiar de pestaña, ningún control de esas tres barras cambia de posición ni de tamaño.
- CA2: la barra de reproducción es visible en las tres pestañas (PD-03).

**RF-003 [1.0] · Pestañas**
Hay tres pestañas, en este orden: **Ritmo**, **Canción** y **Mezcla**. Siempre hay exactamente una activa y su contenido ocupa toda la zona de pestaña. Al crear o abrir un proyecto la pestaña activa es Ritmo. Se cambia de pestaña pulsando su nombre o con `Ctrl+1`, `Ctrl+2` y `Ctrl+3`. Cada pestaña conserva su estado (selección, desplazamiento) mientras se visita otra.
**Criterios de aceptación:**
- CA1: tras `Ctrl+2` se ve la pestaña Canción y solo ella.
- CA2: tras desplazar la línea de tiempo en Canción, ir a Mezcla y volver, el desplazamiento es el mismo.

**RF-004 [1.0] · Pantalla de inicio**
Al arrancar, después de resolver una posible recuperación (RF-027), el programa DEBE mostrar un cuadro con: el botón «Nuevo proyecto», el botón «Abrir proyecto…» y la lista «Recientes» (RF-022). Elegir una opción ejecuta RF-020, RF-021 o abre el proyecto reciente elegido. Pulsar `Esc` o cerrar el cuadro equivale a «Nuevo proyecto». El cuadro solo aparece una vez por arranque.
**Criterios de aceptación:**
- CA1: al arrancar sin recuperación pendiente, el cuadro aparece antes de poder usar ninguna pestaña.
- CA2: tras pulsar `Esc` hay un proyecto vacío «Sin título» y la pestaña activa es Ritmo.

**RF-005 [1.0] · Proyecto vacío**
Un proyecto nuevo DEBE tener: ninguna pista, tempo `TEMPO_DEFAULT`, compás 4/4, claqueta desactivada, bucle desactivado con el tramo en los compases 1 a 4, ajuste a la rejilla activado, zoom de 24 píxeles por negra y cursor en el compás 1. NO DEBE contener nada más ni mostrar guías (PD-11).
**Criterios de aceptación:**
- CA1: en un proyecto nuevo, la pestaña Ritmo solo tiene activo «Nueva pista de ritmo»; la pestaña Canción muestra una línea de tiempo sin carriles; la pestaña Mezcla muestra solo el canal maestro.
- CA2: no aparece ningún texto, flecha ni panel de ayuda que no exista también con el proyecto lleno.

**RF-006 [1.0] · Barra de estado**
La barra de estado DEBE mostrar a la izquierda el último aviso o mensaje informativo, que desaparece a los 5 segundos, y a la derecha el indicador de estado del audio (RF-508).
**Criterios de aceptación:**
- CA1: tras guardar aparece «Proyecto guardado» y desaparece a los 5 segundos.
- CA2: un aviso nuevo sustituye al anterior de inmediato.

**RF-007 [1.0] · Avisos y diálogos de error**
Todo problema se comunica de una de estas dos formas:
- **Aviso** (en la barra de estado, 5 segundos): cuando la acción pedida no se puede hacer y no se pierde nada. Ejemplo: se alcanzó el máximo de pistas.
- **Diálogo de error** (cuadro que exige respuesta): cuando una operación iniciada por el usuario ha fallado o hace falta una decisión.

En ambos casos el texto DEBE decir qué ha pasado y qué puede hacer el usuario (PD-10). Un diálogo de error DEBE tener un botón con la acción que lo resuelve, si existe, y un botón «Cerrar». NO DEBEN mostrarse códigos internos ni textos del sistema; el detalle técnico va al archivo de registro (RNF-20). Todos los textos están en el catálogo del documento 03.
**Criterios de aceptación:**
- CA1: ningún mensaje visible contiene el identificador `E-nnn`, una ruta de código ni un mensaje de excepción.
- CA2: todo diálogo de error tiene al menos el botón «Cerrar» y se cierra también con `Esc`.

**RF-008 [1.0] · Explicación de cada control**
Todo botón, deslizador, campo y menú DEBE mostrar una explicación de una frase cuando el ratón lleva 500 ms encima o cuando recibe el foco del teclado. Si el control está desactivado, la explicación DEBE decir por qué.
**Criterios de aceptación:**
- CA1: al dejar el ratón sobre «Grabar» en la pestaña Ritmo aparece «Para grabar, ve a la pestaña Canción».
- CA2: la explicación desaparece al retirar el ratón, al perder el foco o al pulsar el control.

**RF-009 [1.0] · Sin conexiones de red**
El programa NO DEBE establecer ninguna conexión de red.
**Criterios de aceptación:**
- CA1: con la red desactivada, todas las funciones se comportan igual.
- CA2: la configuración de la aplicación no incluye permisos ni componentes de red (07).

**RF-010 [1.0] · Cambios sin guardar al cerrar o cambiar de proyecto**
Si hay cambios sin guardar y el usuario cierra la ventana, crea un proyecto nuevo o abre otro, el programa DEBE preguntar «¿Guardar los cambios de «<nombre>»?» con los botones «Guardar», «No guardar» y «Cancelar».
- «Guardar» ejecuta RF-023 y, si termina bien, continúa; si el guardado se cancela o falla, la operación se abandona.
- «No guardar» continúa, descarta los cambios y borra la copia de recuperación; si el proyecto no se había guardado nunca, borra también su carpeta temporal.
- «Cancelar» no hace nada.

Si se está grabando, antes se termina la grabación (RF-313).
**Criterios de aceptación:**
- CA1: sin cambios sin guardar, no aparece ninguna pregunta.
- CA2: tras «No guardar», el archivo de proyecto es idéntico byte a byte al que había (PD-09).

---

## 3. Proyecto e historial

**Contexto.** Un proyecto es una carpeta cuyo nombre es el nombre del proyecto. Contiene el archivo de proyecto `project.carabo` (JSON) y las subcarpetas `audio/` (archivos de audio), `peaks/` (datos para dibujar las ondas) y `recovery/` (copia de recuperación). Un proyecto nuevo que todavía no se ha guardado vive en una carpeta temporal dentro de los datos de la aplicación. La estructura exacta está en 05. El guardado es manual (PD-09).

**RF-020 [1.0] · Nuevo proyecto**
«Archivo > Nuevo proyecto» (`Ctrl+N`) DEBE, tras aplicar RF-010, parar el sonido, crear un proyecto vacío (RF-005) en una carpeta temporal nueva, vaciar el historial y activar la pestaña Ritmo.
**Criterios de aceptación:**
- CA1: tras la orden, el proyecto no tiene pistas y Deshacer está desactivado.
- CA2: la carpeta temporal existe y contiene las subcarpetas `audio/`, `peaks/` y `recovery/` vacías.

**RF-021 [1.0] · Abrir proyecto**
«Archivo > Abrir proyecto…» (`Ctrl+O`) DEBE mostrar el cuadro nativo de abrir archivo filtrado por `project.carabo` y, tras aplicar RF-010, leer y validar el archivo (05).
- Si es válido: se carga, se vacía el historial, se activa la pestaña Ritmo, el cursor va al compás 1, ninguna pista queda armada y el proyecto pasa al principio de la lista de recientes.
- Si no es JSON válido o no cumple el esquema: diálogo de error; el proyecto que estaba abierto sigue abierto sin cambios.
- Si su versión de formato es mayor que `FORMAT_VERSION`: diálogo de error específico (RF-034).
- Si faltan archivos de audio: RF-028.
**Criterios de aceptación:**
- CA1: abrir el proyecto de ejemplo del documento 05 produce un estado igual al descrito allí.
- CA2: abrir un archivo con una llave de menos no cambia nada y muestra el diálogo de error.

**RF-022 [1.0] · Proyectos recientes**
El programa DEBE recordar los `RECENTS_MAX` proyectos abiertos o guardados más recientemente, del más reciente al más antiguo, y mostrarlos en la pantalla de inicio y en «Archivo > Abrir reciente». Cada entrada muestra el nombre del proyecto y la ruta de su carpeta. Elegir una la abre como RF-021. Si su archivo de proyecto ya no existe, DEBE mostrarse un diálogo de error con el botón «Quitar de la lista».
**Criterios de aceptación:**
- CA1: tras abrir 11 proyectos distintos, la lista tiene 10 y no contiene el primero.
- CA2: abrir un proyecto que ya estaba en la lista lo mueve al principio sin duplicarlo.

**RF-023 [1.0] · Guardar**
«Archivo > Guardar» (`Ctrl+S`):
- Si el proyecto no se ha guardado nunca, ejecuta RF-024.
- Si ya tiene carpeta, escribe el archivo de proyecto de forma atómica (RNF-13), marca que no hay cambios sin guardar, borra la copia de recuperación y muestra el aviso «Proyecto guardado».
- Si la escritura falla, muestra un diálogo de error; el archivo anterior queda intacto y los cambios siguen marcados como no guardados.

La orden está desactivada durante la grabación.
**Criterios de aceptación:**
- CA1: tras guardar, cerrar y abrir, el proyecto es igual al que se guardó (igualdad profunda de los datos).
- CA2: con el archivo de proyecto protegido contra escritura, aparece el diálogo de error y el archivo no cambia.

**RF-024 [1.0] · Guardar como**
«Archivo > Guardar como…» (`Ctrl+Mayús+S`) DEBE mostrar un cuadro con:
- **Nombre del proyecto**: de 1 a `NAME_MAX_CHARS` caracteres; no admite `\ / : * ? " < > |`, ni empezar o terminar con espacio o punto.
- **Ubicación**: por defecto `Documentos\Carabo`, que el programa crea si no existe; el botón «Cambiar…» abre el cuadro nativo de elegir carpeta.

Al aceptar, el programa crea la carpeta `<ubicación>\<nombre>\`, copia en ella los archivos de `audio/` y `peaks/` que el estado actual utiliza, escribe el archivo de proyecto y pasa a trabajar sobre la carpeta nueva, que se añade a los recientes. Si ya existe un archivo o carpeta con ese nombre en la ubicación, el cuadro muestra un error junto al campo y no escribe nada. Si el proyecto no se había guardado nunca, su carpeta temporal se borra al terminar bien. Si ya estaba guardado en otra carpeta, esa carpeta no se toca.
**Criterios de aceptación:**
- CA1: tras «Guardar como» con nombre «Demo», existe `Demo\project.carabo` y el título es `Demo — Carabo`.
- CA2: la carpeta nueva contiene solo los archivos de audio a los que se refiere el estado actual.
- CA3: el nombre `a/b` se rechaza con un mensaje junto al campo.

**RF-025 [1.0] · Indicador de cambios sin guardar**
Hay cambios sin guardar cuando la posición actual del historial no es la que había al guardar o abrir por última vez. El asterisco del título (RF-001) DEBE reflejarlo en todo momento. Los cambios del estado de vista (RF-031) no cuentan.
**Criterios de aceptación:**
- CA1: tras un cambio y un Deshacer, el asterisco desaparece.
- CA2: activar la claqueta o cambiar el zoom no hace aparecer el asterisco.

**RF-026 [1.0] · Copia de recuperación**
Mientras haya cambios sin guardar, el programa DEBE escribir el estado completo del proyecto en `recovery/project.recovery.carabo`, dentro de la carpeta del proyecto: cada `RECOVERY_INTERVAL_S` segundos y, además, justo al terminar cada grabación (RF-313). La escritura es atómica y no se hace durante un gesto continuo ni durante una grabación: espera a que terminen. La copia NO DEBE modificar `project.carabo`. Se borra al guardar bien y al descartar los cambios (RF-010).
**Criterios de aceptación:**
- CA1: 120 segundos después de un cambio sin guardar existe el archivo de la copia y su contenido es el estado actual.
- CA2: tras guardar, el archivo de la copia no existe.
- CA3: sin cambios sin guardar no se escribe ninguna copia.

**RF-027 [1.0] · Recuperación tras un cierre inesperado**
El programa DEBE anotar en sus datos qué carpeta de proyecto tiene abierta y si se cerró correctamente. Al arrancar, si la última sesión no se cerró correctamente y esa carpeta contiene una copia de recuperación válida, DEBE mostrar, antes de la pantalla de inicio, un cuadro con el texto «Carabo se cerró de forma inesperada. Hay una copia de recuperación de «<nombre>» del <fecha y hora>.» y los botones «Recuperar» y «Descartar».
- «Recuperar» abre el proyecto con el estado de la copia, marcado con cambios sin guardar, y no muestra la pantalla de inicio.
- «Descartar» borra la copia (y la carpeta temporal, si el proyecto no se había guardado nunca) y muestra la pantalla de inicio.

Una copia que no se puede leer o validar se trata como inexistente: se borra y se anota en el registro.
**Criterios de aceptación:**
- CA1: tras matar el proceso con cambios sin guardar y una copia escrita, el siguiente arranque ofrece recuperarla.
- CA2: tras «Recuperar», el estado es el de la copia y el título lleva asterisco.
- CA3: tras un cierre normal, el siguiente arranque no ofrece nada.

**RF-028 [1.0] · Archivos de audio no encontrados**
Si al abrir un proyecto falta o no se puede leer algún archivo de audio, el proyecto DEBE abrirse igualmente. Los clips que lo usan se muestran como «Archivo no encontrado» (RF-323) y no suenan; se pueden editar y eliminar como cualquier otro. Un diálogo DEBE enumerar los archivos que faltan. Lo mismo se aplica si, con el proyecto ya abierto, el motor no consigue cargar en memoria un archivo de audio (06 §9.1): sus clips pasan a mostrarse como «Archivo no encontrado» y el diálogo enumera esos archivos.
**Criterios de aceptación:**
- CA1: tras borrar un archivo de `audio/` y abrir, el proyecto se abre, el diálogo nombra ese archivo y sus clips aparecen marcados.
- CA2: reproducir y exportar funcionan; esos clips son silencio.

**RF-029 [1.0] · Limpieza de archivos sin uso**
Al abrir un proyecto, una vez resuelta la recuperación, el programa DEBE borrar los archivos de `audio/` y `peaks/` a los que el estado cargado no se refiere. NO DEBE borrar archivos de audio en ningún otro momento.
**Criterios de aceptación:**
- CA1: tras grabar una toma, deshacerla, guardar, cerrar y abrir, el archivo de esa toma ya no existe.
- CA2: durante una sesión, deshacer una grabación no borra su archivo y Rehacer la recupera con sonido.

**RF-030 [1.0] · Deshacer y rehacer**
«Deshacer» (`Ctrl+Z`) y «Rehacer» (`Ctrl+Y`) están en la barra superior. Cada paso de historial se deshace y se rehace como una unidad. Se conservan los últimos `HISTORY_LIMIT` pasos. Rehacer está disponible hasta que se hace un cambio nuevo. La explicación de cada botón nombra el paso («Deshacer: mover clip»). Ambos están desactivados cuando no hay nada que deshacer o rehacer y durante la grabación. El historial se vacía al crear o abrir un proyecto, no al guardar.
**Criterios de aceptación:**
- CA1: tras N cambios y N Deshacer, el proyecto es igual (igualdad profunda) al inicial; tras N Rehacer, igual al final (PD-08).
- CA2: tras 201 cambios solo se pueden deshacer 200.
- CA3: deshacer durante la reproducción cambia lo que suena sin pararla.

**RF-031 [1.0] · Qué entra en el historial**
Entran en el historial, y cuentan como cambios sin guardar, todos los cambios de: pistas (crear, eliminar, renombrar, reordenar), kit, volumen y silencio de fila, patrones (crear, duplicar, renombrar, eliminar, golpes, longitud, carga de un ritmo hecho), bloques y clips (crear, mover, redimensionar, repetir, partir, duplicar, eliminar), importaciones, grabaciones, mezcla (volumen, panorama, silenciar, solo, ecualizador, reverb), tempo y compás.

NO entran en el historial ni cuentan como cambios: pestaña activa, selección, posición del cursor, desplazamiento, pista armada y ajustes del programa. Tampoco entra el **estado de vista** —claqueta, bucle y su tramo, ajuste a la rejilla y zoom—, que sí se guarda en el archivo de proyecto al guardar.
**Criterios de aceptación:**
- CA1: mover el deslizador de volumen de una pista y pulsar Deshacer lo devuelve a su valor anterior.
- CA2: cambiar el zoom y pulsar Deshacer no cambia el zoom.

**RF-032 [1.0] · Un gesto continuo es un solo paso**
Mientras se arrastra un deslizador, un clip, un bloque o un borde, el cambio se aplica al momento (se ve y se oye) pero solo genera un paso de historial al soltar. `Esc` durante el arrastre cancela el gesto, restaura el valor anterior y no genera ningún paso. Un campo numérico genera un paso al confirmar (`Intro` o al perder el foco) solo si el valor ha cambiado.
**Criterios de aceptación:**
- CA1: arrastrar un deslizador de volumen de 0 a −10 dB pasando por 40 valores intermedios y pulsar Deshacer una vez lo devuelve a 0 dB.
- CA2: arrastrar un clip y pulsar `Esc` antes de soltar lo deja donde estaba y Deshacer no cambia de estado.

**RF-033 [1.1] · Empaquetar proyecto**
«Archivo > Empaquetar proyecto…» DEBE crear, en la ruta que elija el usuario, un archivo `<nombre>.carabo.zip` que contenga la carpeta `<nombre>/` con `project.carabo` y los archivos de `audio/` que el proyecto utiliza. Si hay cambios sin guardar, antes pide guardar. Muestra el progreso y permite cancelar.
**Criterios de aceptación:**
- CA1: descomprimir el archivo en otra carpeta y abrir su `project.carabo` produce el mismo proyecto.

**RF-034 [1.0] · Versión del formato**
El archivo de proyecto lleva el campo `formatVersion` con el valor `FORMAT_VERSION`. Un archivo con un valor mayor NO DEBE abrirse: se muestra un diálogo de error que dice que el proyecto se creó con una versión más reciente de Carabo.
**Criterios de aceptación:**
- CA1: un archivo con `formatVersion: 2` no se abre y muestra ese diálogo.

---

## 4. Transporte

**Contexto.** La barra de reproducción es común a todas las pestañas. Hay dos modos de reproducción: en **modo patrón** suena en bucle el patrón seleccionado en la pestaña Ritmo; en **modo canción** suena la canción desde el cursor. No existe un interruptor de modo: lo decide la pestaña activa al pulsar Reproducir (DEC-05). El tempo se mide siempre en negras por minuto. La canción no puede pasar de 10 minutos.

**RF-100 [1.0] · Barra de reproducción**
La barra DEBE contener, de izquierda a derecha: Reproducir/Parar, Grabar, indicador de qué suena, posición, Tempo, Compás, Claqueta y Bucle.
**Criterios de aceptación:**
- CA1: los ocho elementos están presentes y en ese orden en las tres pestañas.

**RF-101 [1.0] · Reproducir y parar**
Un único botón alterna entre «Reproducir» (cuando está parado) y «Parar» (cuando reproduce, cuenta atrás o graba). Su atajo es `Espacio`. Al pulsar Reproducir, el sonido empieza en menos de 100 ms en el modo que indique RF-102. Al pulsar Parar, el sonido se apaga con un fundido de 5 ms y se aplica RF-105. Si no hay dispositivo de salida, se aplica RF-503 y no se reproduce nada.
**Criterios de aceptación:**
- CA1: con un campo de texto enfocado, `Espacio` escribe un espacio y no reproduce.
- CA2: al parar no se oye ningún clic.

**RF-102 [1.0] · Modo patrón y modo canción**
El modo se decide en el momento de pulsar Reproducir:
- **Pestaña Ritmo → modo patrón.** Suena en bucle el patrón seleccionado de la pista de ritmo seleccionada, a través del canal de esa pista (efectos, volumen y panorama) y del canal maestro. Se ignoran el «Silenciar» de esa pista y el «Solo» de las demás. No suena nada más, salvo la claqueta si está activada.
- **Pestañas Canción o Mezcla → modo canción.** Suena la canción desde el cursor.

Cambiar de pestaña durante la reproducción no cambia el modo ni interrumpe el sonido. Para cambiar de modo hay que parar y volver a reproducir. En modo patrón, seleccionar otro patrón u otra pista de ritmo cambia al momento lo que suena, conservando el paso actual módulo la nueva longitud. Si en la pestaña Ritmo no hay ninguna pista de ritmo, Reproducir está desactivado.
**Criterios de aceptación:**
- CA1: con una pista de ritmo silenciada en Mezcla, Reproducir en la pestaña Ritmo la hace sonar.
- CA2: empezar a reproducir en Ritmo y pasar a Canción no cambia lo que suena.

**RF-103 [1.0] · Indicador de qué suena**
La barra de reproducción DEBE mostrar un texto con el estado: «Parado», «Suena: patrón «<nombre>»», «Suena: canción», «Cuenta atrás» o «Grabando».
**Criterios de aceptación:**
- CA1: cada uno de los cinco estados muestra su texto.

**RF-104 [1.0] · Cursor de reproducción**
El cursor es una posición de la canción en ticks. Se dibuja como una línea vertical en la línea de tiempo de la pestaña Canción. Estando parado, un clic en la regla lo coloca en ese punto: redondeado al paso más cercano si el ajuste a la rejilla está activado, o en el tick que corresponde al píxel si no. La tecla `Inicio` lo lleva al compás 1. El cursor no puede estar antes del principio ni después del límite de la canción (RF-112).
**Criterios de aceptación:**
- CA1: con el ajuste activado, un clic entre dos pasos coloca el cursor en el más cercano.
- CA2: tras `Inicio`, la posición mostrada es `1.1`.

**RF-105 [1.0] · Al parar, el cursor vuelve**
Al parar una reproducción en modo canción o una grabación, el cursor DEBE volver a la posición desde la que empezó. En modo patrón el cursor de la canción no se mueve, y la siguiente reproducción del patrón empieza en su primer paso.
**Criterios de aceptación:**
- CA1: reproducir desde el compás 3 y parar en el 7 deja el cursor en el compás 3.

**RF-106 [1.0] · Tempo**
Campo numérico entero entre `TEMPO_MIN` y `TEMPO_MAX`, con botones «−» y «+» que restan y suman 1. Un valor escrito fuera de rango se ajusta al límite al confirmar y muestra un aviso; un texto que no es un número restaura el valor anterior. El cambio se oye al momento, también durante la reproducción; en modo canción las pistas de audio pueden sufrir un corte de hasta 100 ms. El control está desactivado durante la cuenta atrás y la grabación. Cada cambio confirmado es un paso de historial.
**Criterios de aceptación:**
- CA1: escribir 300 y pulsar `Intro` deja 240.
- CA2: en modo patrón, cambiar el tempo no produce ningún corte.

**RF-107 [1.0] · Aviso al cambiar el tempo con audio**
Si el proyecto contiene algún clip de audio, el primer cambio de tempo desde que se abrió o se creó el proyecto DEBE mostrar antes un cuadro que explique que el audio grabado o importado no cambia de velocidad y dejará de coincidir con los ritmos, con los botones «Cambiar el tempo» y «Cancelar». Los cambios posteriores de esa sesión se aplican sin cuadro. Al cambiar el tempo, cada clip conserva su posición inicial en ticks y su longitud en frames.
**Criterios de aceptación:**
- CA1: con un clip en el compás 5, tras cambiar el tempo el clip sigue empezando en el compás 5 y dura los mismos segundos.
- CA2: «Cancelar» deja el tempo como estaba y no genera paso de historial.

**RF-108 [1.0] · Compás**
Lista desplegable con `4/4`, `3/4` y `6/8`. Está desactivada mientras se reproduce o se graba. El cambio aplica RF-109 y es un paso de historial.
**Criterios de aceptación:**
- CA1: la lista contiene exactamente esas tres opciones.

**RF-109 [1.0] · Efecto de cambiar el compás**
Al cambiar el compás:
1. Cada patrón se convierte compás a compás. Si el compás nuevo tiene menos pasos (de 16 a 12), cada compás conserva sus 12 primeros pasos y pierde los demás. Si tiene más (de 12 a 16), cada compás conserva sus pasos y añade pasos vacíos al final. Entre 3/4 y 6/8 no cambia nada.
2. Cada bloque conserva su compás inicial y su longitud en compases.
3. Cada clip conserva su posición inicial en ticks.
4. El tramo de bucle conserva sus compases.
5. El cursor va al compás 1.

Si algún patrón tiene golpes o existe algún clip, antes de aplicar DEBE mostrarse siempre un cuadro que explique los puntos 1 y 3, con los botones «Cambiar el compás» y «Cancelar». Deshacer restaura exactamente todo, incluidos los pasos perdidos.
**Criterios de aceptación:**
- CA1: un patrón de 1 compás en 4/4 con golpes en los pasos 0, 4, 8 y 12 queda en 3/4 con golpes en 0, 4 y 8.
- CA2: tras Deshacer, vuelve a tener golpes en 0, 4, 8 y 12.

**RF-110 [1.0] · Claqueta**
Botón de dos estados. Activada, suena un clic en cada pulso durante la reproducción (en los dos modos) y la grabación: en las negras en 4/4 y 3/4, y en las corcheas en 6/8. El primer pulso de cada compás suena con un clic más agudo. La claqueta pasa por el volumen maestro y NO DEBE aparecer nunca en una exportación. La cuenta atrás (RF-311) suena aunque la claqueta esté desactivada.
**Criterios de aceptación:**
- CA1: en 6/8 suenan seis clics por compás, el primero más agudo.
- CA2: una exportación con la claqueta activada no contiene clics.

**RF-111 [1.0] · Bucle**
Botón de dos estados «Bucle» y un tramo `[compás inicial, compás final)` dibujado en la franja de bucle de la regla. El tramo se edita en esa franja: arrastrar sobre una zona vacía crea un tramo nuevo; arrastrar un borde lo ajusta. Siempre son compases enteros, con un mínimo de uno y dentro del límite de la canción. Con el bucle activado, en modo canción, cuando el cursor llega al final del tramo salta a su principio sin que el sonido se interrumpa. Si la reproducción empieza en el final del tramo o después, no hay salto. El bucle no actúa durante la grabación ni en modo patrón.
**Criterios de aceptación:**
- CA1: con el tramo en los compases 1 a 4 y reproducción desde el compás 1, tras el final del compás 4 suena de nuevo el compás 1, sin silencio entre medias.
- CA2: con reproducción desde el compás 6, la reproducción sigue sin saltar.

**RF-112 [1.0] · Límite de la canción**
Nada puede colocarse, moverse, alargarse ni grabarse de modo que termine después de `MAX_SONG_FRAMES` desde el principio de la canción. Un intento se limita al máximo permitido y muestra un aviso. Un cambio de tempo o de compás que dejaría algún bloque o clip terminando después del límite no se aplica: se muestra un aviso y nada cambia. La reproducción y la grabación se paran solas al llegar al límite. Antes del límite la reproducción no se para sola (DEC-37). Si tras un cambio de tempo o de compás el cursor o el tramo de bucle quedan más allá del límite, se llevan al último punto válido.
**Criterios de aceptación:**
- CA1: arrastrar un clip más allá del límite lo deja terminando exactamente en el límite.
- CA2: una grabación que llega a los 10 minutos termina como si se hubiera pulsado Parar.
- CA3: a 120 negras por minuto en 4/4, con un clip que empieza en el compás 200, cambiar el tempo a 60 no se aplica y muestra el aviso (el clip empezaría pasados los 13 minutos).

**RF-113 [1.0] · Presentación de la posición**
La barra de reproducción DEBE mostrar la posición del cursor (parado) o de la reproducción (en modo canción) en dos formatos: `compás.pulso`, ambos desde 1 (por ejemplo `5.3`), y `minutos:segundos.décimas` (por ejemplo `0:09.5`). Durante la reproducción se actualiza al menos 15 veces por segundo. En modo patrón muestra el compás y el pulso dentro del patrón.
**Criterios de aceptación:**
- CA1: a 120 negras por minuto en 4/4, el principio del compás 5 se muestra como `5.1` y `0:08.0`.

**RF-114 [1.0] · Mover el cursor durante la reproducción**
En modo canción, sin estar grabando, un clic en la regla DEBE llevar la reproducción a ese punto, con el mismo redondeo que RF-104; el sonido continúa desde allí en menos de 100 ms. Ese punto pasa a ser la posición a la que vuelve el cursor al parar (RF-105).
**Criterios de aceptación:**
- CA1: durante la grabación, un clic en la regla no hace nada.

**RF-115 [1.1] · Pinchazo**
Botón de dos estados «Pinchazo» en la barra de reproducción, a la derecha de Bucle. Activado, al pulsar Grabar:
1. La reproducción empieza un compás antes del principio del tramo de bucle, con la canción sonando. Si el tramo empieza en el compás 1, se usa la cuenta atrás normal.
2. Solo se graba entre el principio y el final del tramo.
3. Al llegar al final del tramo, la grabación y la reproducción terminan solas.

El resultado es un clip que ocupa el tramo y tapa lo que hubiera (RF-309).
**Criterios de aceptación:**
- CA1: con el tramo en los compases 5 a 6, el clip resultante empieza en el compás 5 y termina al final del 6.

---

## 5. Pestaña Ritmo

**Contexto.** En esta pestaña se crean y editan los patrones. Un patrón pertenece a una pista de ritmo, que tiene un kit de 8 sonidos. La rejilla de pasos tiene 8 filas (una por sonido) y tantos pasos como semicorcheas duran sus compases: 16 por compás en 4/4 y 12 en 3/4 y 6/8. Las pistas de ritmo solo se crean aquí (DEC-28). Al crear una, su primer patrón se coloca solo en la canción (DEC-05). Los kits y los ritmos hechos se definen en 05.

**RF-200 [1.0] · Contenido de la pestaña Ritmo**
La pestaña DEBE contener: una barra de herramientas («Nueva pista de ritmo», selector «Pista», selector «Kit», «Cargar ritmo hecho…», «Borrar golpes» y selector «Compases»), la lista de patrones con sus órdenes («Nuevo patrón», «Duplicar», «Renombrar» y «Eliminar») y la rejilla de pasos. Si no hay ninguna pista de ritmo, solo «Nueva pista de ritmo» está activo.
**Criterios de aceptación:**
- CA1: en un proyecto vacío, todos los controles salvo «Nueva pista de ritmo» están desactivados y lo explican (RF-008).

**RF-201 [1.0] · Crear una pista de ritmo**
«Nueva pista de ritmo» DEBE crear, como un solo paso de historial:
- una pista de ritmo llamada «Ritmo N», donde N es el menor entero positivo que no deja el nombre repetido, añadida al final de la lista de pistas, con el kit «Batería acústica» y su canal con los valores iniciales;
- un patrón vacío «Patrón 1» de 1 compás;
- un bloque de ese patrón en la canción, desde el compás 1 y de `FIRST_BLOCK_BARS` compases (o los compases enteros que quepan antes del límite de la canción).

La pista nueva queda seleccionada en la pestaña Ritmo. Con `MAX_TRACKS` pistas el botón está desactivado.
**Criterios de aceptación:**
- CA1: tras la orden en un proyecto vacío, la pestaña Canción muestra la pista «Ritmo 1» con un bloque de «Patrón 1» en los compases 1 a 16.
- CA2: un solo Deshacer elimina la pista, el patrón y el bloque.

**RF-202 [1.0] · Pista de ritmo seleccionada**
El selector «Pista» enumera las pistas de ritmo por nombre, en el orden de las pistas. La elegida determina lo que muestran el selector de kit, la lista de patrones y la rejilla. Al abrir un proyecto queda seleccionada la primera pista de ritmo y su primer patrón. Si la pista seleccionada se elimina, queda seleccionada la primera que quede, o ninguna.
**Criterios de aceptación:**
- CA1: con dos pistas de ritmo, cambiar la selección cambia la lista de patrones y la rejilla.

**RF-203 [1.0] · Kits**
Hay tres kits: «Batería acústica», «Batería electrónica» y «Percusión». Cada uno tiene `KIT_ROWS` sonidos en el mismo orden de funciones (05), de modo que un mismo patrón tiene sentido con cualquiera. El selector «Kit» cambia el kit de la pista seleccionada: sus patrones conservan los golpes y suenan con los sonidos nuevos. Es un paso de historial.
**Criterios de aceptación:**
- CA1: tras cambiar de kit, los nombres de las 8 filas son los del kit nuevo y los golpes no han cambiado.

**RF-204 [1.0] · Rejilla de pasos**
La rejilla DEBE mostrar el patrón seleccionado: 8 filas, cada una con el nombre de su sonido, por `compases × STEPS_PER_BAR` pasos. Los pasos se agrupan a la vista por pulsos y por compases (03). Cada paso muestra si tiene golpe. Si no caben todos, la rejilla se desplaza en horizontal y los nombres de fila permanecen fijos.
**Criterios de aceptación:**
- CA1: un patrón de 2 compases en 4/4 muestra 32 pasos por fila; en 3/4, 24.

**RF-205 [1.0] · Marcar y quitar golpes**
Un clic en un paso vacío crea un golpe de nivel 2; un clic en un paso con golpe lo quita. Cada clic es un paso de historial. Con el teclado, las flechas mueven el foco entre pasos y `Espacio` alterna el paso enfocado. Estando parado, al crear un golpe su sonido suena una vez.
**Criterios de aceptación:**
- CA1: dos clics seguidos en un paso lo dejan como estaba y generan dos pasos de historial.
- CA2: con el foco en la rejilla, `Espacio` alterna el paso y no inicia la reproducción.

**RF-206 [1.0] · Escuchar un sonido**
Un clic en el nombre de una fila DEBE hacer sonar ese sonido una vez, a través del canal de la pista, tanto parado como reproduciendo.
**Criterios de aceptación:**
- CA1: el sonido se oye en menos de 100 ms y no modifica el proyecto.

**RF-207 [1.0] · Longitud del patrón**
El selector «Compases» ofrece los valores de `PATTERN_BARS`. Aumentar la longitud añade compases vacíos al final; reducirla quita los compases finales con sus golpes. Es un paso de historial. Los bloques de ese patrón conservan su longitud en compases.
**Criterios de aceptación:**
- CA1: pasar de 2 compases a 1 y Deshacer restaura los golpes del segundo compás.

**RF-208 [1.0] · Lista de patrones**
La lista muestra los patrones de la pista seleccionada en orden de creación. Siempre hay exactamente uno seleccionado, cuyo contenido muestra la rejilla.
- **«Nuevo patrón»**: crea un patrón vacío de 1 compás llamado «Patrón N» (N, el menor entero positivo que no repite nombre en la pista) y lo selecciona. Con `MAX_PATTERNS_PER_TRACK` patrones está desactivado.
- **«Duplicar»**: crea una copia del seleccionado llamada «<nombre> copia» (recortado a `NAME_MAX_CHARS`) y la selecciona.
- **«Renombrar»** (`F2`): permite escribir un nombre de 1 a `NAME_MAX_CHARS` caracteres; un nombre vacío restaura el anterior.
- **«Eliminar»**: elimina el patrón seleccionado y todos sus bloques de la canción en un solo paso de historial, muestra un aviso con el número de bloques eliminados y selecciona el patrón anterior de la lista. Si la pista solo tiene un patrón, está desactivado.
**Criterios de aceptación:**
- CA1: eliminar un patrón con 3 bloques los quita de la canción; un solo Deshacer restaura el patrón y los 3 bloques.
- CA2: con un único patrón, «Eliminar» está desactivado y lo explica.

**RF-209 [1.0] · Ritmos hechos**
«Cargar ritmo hecho…» DEBE abrir una lista con los ritmos hechos (05) que corresponden al compás del proyecto. Hay 15: 11 para 4/4, 2 para 3/4 y 2 para 6/8. Elegir uno sustituye el contenido del patrón seleccionado: su longitud pasa a ser la del ritmo hecho y sus golpes, los del ritmo hecho, todos de nivel 2. El nombre del patrón no cambia. Es un paso de historial. Cancelar no cambia nada.
**Criterios de aceptación:**
- CA1: en 3/4 la lista muestra exactamente 2 ritmos.
- CA2: tras cargar uno y pulsar Deshacer, el patrón recupera su contenido y su longitud anteriores.

**RF-210 [1.0] · Volumen y silencio por fila**
Cada fila tiene un deslizador de volumen entre `ROW_GAIN_MIN_DB` y `ROW_GAIN_MAX_DB` (valor inicial 0 dB, pasos de 1 dB) y un botón «Silenciar» de dos estados. Pertenecen a la pista y valen para todos sus patrones. Una fila silenciada conserva sus golpes a la vista pero no suena. El historial sigue RF-032.
**Criterios de aceptación:**
- CA1: silenciar una fila en un patrón la silencia también en los demás patrones de la pista.

**RF-211 [1.0] · Reproducción del patrón**
En modo patrón, el patrón seleccionado suena en bucle sin pausa entre repeticiones y la columna del paso que suena se resalta. Los cambios de golpes, volumen o silencio de fila, kit, longitud y tempo se oyen como muy tarde en el paso siguiente. Al cambiar la longitud, la posición pasa a ser la actual módulo la nueva longitud.
**Criterios de aceptación:**
- CA1: cada golpe empieza en su frame exacto con un error máximo de 1 frame (RNF-10).
- CA2: el resaltado avanza sincronizado con el sonido, con un desfase visible menor de 50 ms.

**RF-212 [1.0] · Borrar golpes**
«Borrar golpes» vacía todos los pasos del patrón seleccionado, en un paso de historial. Está desactivado si el patrón ya está vacío.
**Criterios de aceptación:**
- CA1: tras la orden y un Deshacer, el patrón es igual al anterior.

**RF-213 [1.0] · Grupos de corte y límite de voces**
Los sonidos de un kit que comparten grupo de corte (el charles cerrado y el abierto, cuando el kit lo define) se cortan entre sí: un golpe de uno apaga con un fundido de 5 ms cualquier golpe del otro que siga sonando en esa pista. Un golpe nuevo de una fila no corta al anterior de la misma fila. Si una pista supera `MAX_VOICES` sonidos simultáneos, se apaga el más antiguo con un fundido de 5 ms.
**Criterios de aceptación:**
- CA1: un charles abierto seguido de uno cerrado un paso después deja de sonar al empezar el cerrado.

**RF-250 [1.1] · Intensidad por golpe**
La barra de herramientas añade el selector «Intensidad» con «Suave», «Normal» y «Fuerte» (niveles 1, 2 y 3; inicial «Normal»). Un clic en un paso vacío crea un golpe con el nivel elegido; en un golpe de otro nivel, le pone el nivel elegido; en un golpe del mismo nivel, lo quita. Cada nivel suena con la ganancia de `LEVEL_GAINS` y se distingue a la vista por algo más que el color (03).
**Criterios de aceptación:**
- CA1: un golpe «Suave» suena con ganancia 0,45 y uno «Fuerte» con 1,0, respecto al mismo sonido.

**RF-251 [1.1] · Swing**
Cada patrón tiene un deslizador «Swing» de 0 a 100 % (inicial 0). Retrasa los pasos de índice impar (el segundo, el cuarto…) en `swing / 100 × TICKS_PER_STEP / 3` ticks. Actúa en los dos modos de reproducción.
**Criterios de aceptación:**
- CA1: con swing del 100 %, el paso de índice 1 suena 80 ticks después de su posición nominal.

**RF-252 [1.1] · Sonidos propios**
Cada fila añade las órdenes «Cambiar sonido…» y «Restaurar sonido del kit». La primera abre un archivo WAV o MP3, toma como máximo sus 5 primeros segundos, lo convierte a mono y 48 000 Hz, lo guarda en `audio/` y lo usa como sonido de esa fila en esa pista. El sonido propio se conserva al cambiar de kit.
**Criterios de aceptación:**
- CA1: tras «Restaurar sonido del kit», la fila vuelve a sonar con el sonido del kit actual.

---

## 6. Pestaña Canción

**Contexto.** En esta pestaña se dispone la canción en el tiempo. A la izquierda están las cabeceras de pista; a la derecha, la línea de tiempo, con la regla arriba y un carril por pista. Los carriles de las pistas de ritmo contienen bloques de patrón, que se colocan y miden en compases enteros. Los carriles de las pistas de audio contienen clips, que se colocan en ticks y miden en frames. Un clip no contiene audio: apunta a un archivo y dice qué región suena. Solo se puede seleccionar un elemento a la vez (DEC-29) y los elementos no cambian de pista. Las pistas de audio solo se crean aquí (DEC-28).

**RF-300 [1.0] · Contenido de la pestaña Canción**
La pestaña DEBE contener: una barra de herramientas («Nueva pista de audio», «Importar audio…», «Ajuste a la rejilla», «Repetir», «Partir», «Duplicar», «Eliminar», «Alejar» y «Acercar»), la columna de cabeceras de pista y la línea de tiempo con la regla, la franja de bucle, los carriles y el cursor.
**Criterios de aceptación:**
- CA1: los nueve controles de la barra de herramientas están presentes y en ese orden.

**RF-301 [1.0] · Crear una pista de audio**
«Nueva pista de audio» DEBE crear una pista de audio llamada «Audio N» (N, el menor entero positivo que no repite nombre), con el carril vacío y el canal con los valores iniciales, añadida al final. Es un paso de historial. La pista queda seleccionada y, si hay dispositivo de entrada, armada (RF-310); si no lo hay, queda sin armar y no se muestra ningún error. Con `MAX_TRACKS` pistas el botón está desactivado.
**Criterios de aceptación:**
- CA1: con un micrófono conectado, tras la orden la pista muestra «Armar» activado y su medidor de entrada.
- CA2: Deshacer elimina la pista y deja el proyecto sin pista armada.

**RF-302 [1.0] · Renombrar, reordenar y eliminar pistas**
Cada cabecera tiene un menú «Pista» con:
- **«Renombrar»**: nombre de 1 a `NAME_MAX_CHARS` caracteres; vacío restaura el anterior.
- **«Subir»** y **«Bajar»**: intercambian la pista con la vecina; están desactivados en los extremos. El orden es el mismo en los carriles, en los canales de Mezcla y en el selector «Pista» de Ritmo.
- **«Eliminar pista»**: elimina la pista con sus bloques, clips y patrones. Los archivos de audio siguen en disco. No pide confirmación (PD-08).

Cada orden es un paso de historial.
**Criterios de aceptación:**
- CA1: eliminar una pista de ritmo con 2 patrones y 5 bloques y pulsar Deshacer la restaura completa y en la misma posición.
- CA2: «Subir» en la segunda pista la deja primera también en la pestaña Mezcla.

**RF-303 [1.0] · Cabecera de pista**
Cada cabecera DEBE mostrar el nombre, el tipo escrito («Ritmo» o «Audio») y el menú «Pista». Además:
- las pistas de audio, el botón «Armar» y, mientras están armadas, el medidor de entrada (RF-505);
- las pistas de ritmo, el selector «Patrón» con sus patrones y el botón «Añadir bloque» (RF-307).

Un clic en una cabecera, o en una zona vacía de su carril, selecciona esa pista. La pista seleccionada es independiente del elemento seleccionado y es el destino de «Importar audio…» (RF-315).
**Criterios de aceptación:**
- CA1: las cabeceras no contienen controles de volumen, panorama, silenciar ni solo (DEC-28).

**RF-304 [1.0] · Regla y rejilla**
La regla DEBE mostrar los números de compás desde 1 y, con zoom de 24 píxeles por negra o más, marcas de pulso. Los carriles muestran una línea vertical en cada compás y, con zoom de 48 o más, en cada pulso. Bajo los números está la franja de bucle (RF-111).
**Criterios de aceptación:**
- CA1: en 3/4 hay una línea de compás cada 3 negras.

**RF-305 [1.0] · Zoom y desplazamiento**
«Alejar» (`Ctrl+−`) y «Acercar» (`Ctrl++`) recorren los valores de `ZOOM_LEVELS`; en los extremos, el botón correspondiente está desactivado. Al cambiar el zoom, el cursor conserva su posición horizontal en pantalla si estaba visible; si no, se conserva el borde izquierdo. `Ctrl` + rueda cambia el zoom igual que los botones. El desplazamiento horizontal se hace con la barra de desplazamiento o con `Mayús` + rueda; el vertical, con la rueda. Durante la reproducción en modo canción y la grabación, cuando el cursor sale por la derecha la vista avanza hasta dejarlo a un 10 % del borde izquierdo, salvo que el usuario esté arrastrando algo.
**Criterios de aceptación:**
- CA1: seis pulsaciones de «Acercar» desde el zoom mínimo llegan al máximo y desactivan el botón.

**RF-306 [1.0] · Selección**
Como máximo hay un elemento (bloque o clip) seleccionado. Un clic en un elemento lo selecciona y deselecciona el anterior; un clic en una zona vacía de un carril deselecciona. El elemento seleccionado se distingue por un contorno más grueso, no solo por el color. «Repetir», «Partir», «Duplicar» y «Eliminar» actúan sobre él y están desactivados cuando no hay selección o no son aplicables.
**Criterios de aceptación:**
- CA1: no existe ninguna forma de seleccionar dos elementos a la vez.

**RF-307 [1.0] · Añadir un bloque de patrón**
En la cabecera de una pista de ritmo, «Añadir bloque» DEBE crear un bloque del patrón elegido en el selector «Patrón» (por defecto, el primero), que empieza al principio del compás que contiene el cursor y dura lo que dura el patrón. El bloque queda seleccionado. Es un paso de historial. Si se solapa con otros, se aplica RF-309. Si no cabe antes del límite de la canción, se acorta a los compases enteros que quepan; si no cabe ninguno, se muestra un aviso y no se crea.
**Criterios de aceptación:**
- CA1: con el cursor en `7.3` y un patrón de 2 compases, el bloque ocupa los compases 7 y 8.

**RF-308 [1.0] · Mover, alargar, duplicar y eliminar bloques**
Un bloque muestra el nombre de su patrón y una marca en cada repetición.
- **Mover**: arrastrarlo en horizontal; la posición se redondea a compases enteros; no cambia de pista.
- **Alargar o acortar**: arrastrar su borde derecho; la longitud se redondea a compases enteros, con un mínimo de 1. El patrón se repite desde su primer paso a partir del principio del bloque; si la longitud no es múltiplo de la del patrón, la última repetición queda cortada. El borde izquierdo no se puede arrastrar.
- **«Duplicar»** (`Ctrl+D`) y **«Eliminar»** (`Supr`): RF-320 y RF-321.

Un bloque hace sonar siempre el contenido actual de su patrón. No puede empezar antes del compás 1 ni terminar después del límite (RF-112).
**Criterios de aceptación:**
- CA1: un bloque de 3 compases de un patrón de 2 hace sonar el patrón entero y después su primer compás.
- CA2: cambiar un golpe del patrón en la pestaña Ritmo cambia lo que suena en todos sus bloques.

**RF-309 [1.0] · Regla de tapado**
En un mismo carril los elementos pueden solaparse. En el solape solo suena y solo se dibuja el que está **encima**, que es el último creado, movido o redimensionado. El de debajo queda intacto: cuando el de encima se elimina, se aparta o se acorta, el de debajo vuelve a sonar y a verse en ese tramo. Un elemento tapado por completo no se ve ni se puede seleccionar mientras siga tapado. La regla es la misma para bloques y para clips.
**Criterios de aceptación:**
- CA1: con un clip A en los compases 1 a 4 y un clip B grabado después en los compases 2 a 3, suena A en el compás 1, B en los compases 2 y 3, y A en el 4.
- CA2: tras eliminar B, A suena entero y sus datos no han cambiado.

**RF-310 [1.0] · Armar una pista**
Cada pista de audio tiene un botón «Armar» de dos estados. Como máximo hay una pista armada: armar una desarma la otra. Mientras hay una pista armada, el programa abre el dispositivo de entrada y la cabecera muestra el medidor de entrada; al desarmar, lo cierra. El armado no se guarda en el proyecto ni entra en el historial. Si no hay dispositivo de entrada, se aplica RF-503 y la pista no se arma.
**Criterios de aceptación:**
- CA1: al armar una segunda pista, la primera queda desarmada.
- CA2: sin ninguna pista armada, el dispositivo de entrada está cerrado.

**RF-311 [1.0] · Empezar a grabar**
«Grabar» (`R`) solo está activo estando parado y en las pestañas Canción o Mezcla. Al pulsarlo:
1. Si no hay ninguna pista de audio: diálogo de error con el botón «Crear pista de audio», que ejecuta RF-301 sin empezar a grabar.
2. Si hay pistas de audio pero ninguna armada: diálogo de error con el botón «Armar «<nombre>»», referido a la pista de audio seleccionada o, si no hay, a la primera. El botón arma esa pista sin empezar a grabar.
3. Si no hay dispositivo de entrada: RF-503.
4. En otro caso empieza la **cuenta atrás**: `COUNT_IN_BARS` compás de clics al tempo del proyecto, con el primero acentuado. Durante la cuenta atrás la canción no suena y el cursor no se mueve.
5. Al terminar la cuenta atrás empiezan a la vez, desde el cursor, la reproducción en modo canción y la grabación.

Pulsar Parar durante la cuenta atrás la cancela sin grabar ni cambiar nada.
**Criterios de aceptación:**
- CA1: en 4/4 la cuenta atrás tiene 4 clics; en 6/8, 6.
- CA2: en la pestaña Ritmo, «Grabar» está desactivado y lo explica.

**RF-312 [1.0] · Mientras se graba**
Durante la grabación:
- La canción suena con su mezcla, salvo la pista armada, que no suena.
- La claqueta suena si está activada.
- En el carril de la pista armada crece un bloque rojo con el texto «Grabando», desde el punto de inicio hasta el cursor.
- La entrada no se envía a la salida (RF-509).
- Solo están activos «Parar» y el cambio de pestaña. Todo control que modifique el proyecto, y también Ajustes, Guardar y Exportar, están desactivados.
- Al llegar al límite de la canción, la grabación termina sola (RF-112).
**Criterios de aceptación:**
- CA1: durante la grabación, los controles de la pestaña Mezcla están desactivados.

**RF-313 [1.0] · Terminar de grabar**
Al pulsar Parar, o al terminar sola, la grabación y la reproducción se detienen y:
1. Existe en `audio/` una toma nueva: un archivo mono a 48 000 Hz con lo grabado.
2. Se crea un clip en la pista armada que empieza en la posición donde empezó la grabación. Su región descarta los primeros `latencyFrames` frames de la toma (RF-507), de modo que lo tocado a tiempo con lo que se oía queda colocado a tiempo.
3. El clip queda encima (RF-309) y seleccionado.
4. Todo ello es un paso de historial llamado «Grabar toma».
5. Se escribe de inmediato la copia de recuperación (RF-026).
6. El cursor vuelve al punto de inicio (RF-105).

Si la región resultante mide menos de `MIN_CLIP_FRAMES`, no se crea clip ni paso de historial, el archivo se borra y se muestra un aviso.
**Criterios de aceptación:**
- CA1: con el equipo calibrado, un clic grabado por el micrófono queda a menos de 5 ms de su posición original (RNF-09).
- CA2: Deshacer quita el clip; Rehacer lo recupera con su sonido.

**RF-314 [1.0] · Errores durante la grabación**
Si durante la grabación se desconecta el dispositivo de entrada o falla la escritura en disco, la grabación termina, lo capturado hasta entonces se conserva y se convierte en clip según RF-313, y un diálogo de error explica lo ocurrido. Si se desconecta la salida, se aplica además RF-504.
**Criterios de aceptación:**
- CA1: desconectar el micrófono a los 10 segundos de grabación deja un clip de unos 10 segundos.

**RF-315 [1.0] · Importar audio**
Se importa con «Importar audio…» (cuadro nativo, un solo archivo) o arrastrando un archivo desde el Explorador de Windows a la pestaña Canción.
- **Formatos**: WAV (PCM de 8, 16, 24 o 32 bits, o coma flotante de 32 o 64 bits; de 8 000 a 192 000 Hz) y MP3. Si tiene más de 2 canales, se usan los 2 primeros.
- **Conversión**: el archivo se convierte a un archivo de audio del proyecto (48 000 Hz, 1 o 2 canales) en `audio/`. El original no se modifica ni se vuelve a usar. Solo se importan los 10 primeros minutos; si se recorta, se muestra un aviso.
- **Destino con el botón**: la pista de audio seleccionada, en el cursor. Si no hay ninguna pista de audio seleccionada, se crea una nueva con el nombre del archivo sin extensión (recortado a `NAME_MAX_CHARS`), sin armar.
- **Destino al arrastrar**: si se suelta sobre el carril de una pista de audio, esa pista, en el punto donde se suelta (redondeado al paso si el ajuste a la rejilla está activado). Si se suelta en otro lugar de la pestaña, una pista de audio nueva, con el clip en la posición horizontal del punto si cae sobre la línea de tiempo o en el compás 1 si no.

El clip nuevo queda seleccionado y encima. Toda la operación, incluida la creación de la pista, es un paso de historial. El progreso se muestra en la barra de estado y, hasta que termina, no se admite otra importación. Un archivo no admitido o dañado produce un diálogo de error y no crea nada. Si hace falta una pista nueva y ya hay `MAX_TRACKS`, se muestra un aviso y no se crea nada.
**Criterios de aceptación:**
- CA1: importar un MP3 estéreo de 44 100 Hz crea un archivo de audio estéreo de 48 000 Hz cuya duración difiere de la original en menos de 1 ms por minuto.
- CA2: un archivo `.txt` renombrado a `.wav` produce el diálogo de error y no cambia el proyecto.
- CA3: soltar un archivo sobre la pestaña Ritmo o Mezcla no hace nada.

**RF-316 [1.0] · Mover un clip**
Un clip se mueve arrastrándolo en horizontal dentro de su pista. Con el ajuste a la rejilla activado, su principio se redondea al paso más cercano; desactivado, al tick que corresponde al píxel. No puede empezar antes del compás 1 ni terminar después del límite. Al moverlo queda encima (RF-309). «Ajuste a la rejilla» es un botón de dos estados de la barra de herramientas, inicialmente activado, y forma parte del estado de vista.
**Criterios de aceptación:**
- CA1: con el ajuste activado, un clip soltado entre dos pasos empieza exactamente en el más cercano.

**RF-317 [1.0] · Recortar los extremos de un clip**
En un clip sin «Repetir», arrastrar un borde cambia qué parte del archivo suena, sin desplazar el audio en el tiempo:
- borde izquierdo hacia la derecha: oculta el principio; hacia la izquierda: descubre audio oculto, hasta el principio del archivo;
- borde derecho hacia la izquierda: oculta el final; hacia la derecha: descubre audio oculto, hasta el final del archivo.

El ajuste a la rejilla se aplica a la posición del borde. La longitud mínima es `MIN_CLIP_FRAMES`. Es no destructivo: el borde se puede devolver a su sitio en cualquier momento.
**Criterios de aceptación:**
- CA1: recortar 1 segundo del principio y volver a arrastrar el borde a su sitio deja el clip igual que al principio.
- CA2: un borde no se puede arrastrar más allá del principio o del final del archivo.

**RF-318 [1.0] · Repetir un clip**
«Repetir» es un botón de dos estados que actúa sobre el clip seleccionado.
- **Al activarlo**, la región actual del clip pasa a ser el contenido que se repite. Arrastrar el borde derecho más allá de la longitud de la región repite la región tantas veces como haga falta; la última repetición puede quedar incompleta. El clip muestra una marca en cada repetición. El borde izquierdo no se puede arrastrar. La longitud mínima es `MIN_CLIP_FRAMES`.
- **Al desactivarlo**, la longitud del clip vuelve a ser la de una región.

Activar y desactivar son, cada uno, un paso de historial.
**Criterios de aceptación:**
- CA1: un clip de 2 compases con «Repetir» alargado a 5 compases suena dos veces entero y después su primera mitad.
- CA2: al desactivar «Repetir», el clip vuelve a medir 2 compases.

**RF-319 [1.0] · Partir un clip**
«Partir» (`S`) divide el clip seleccionado en el cursor en dos clips que, juntos, suenan exactamente como el original. El de la derecha queda seleccionado. Es un paso de historial. Está desactivado, y lo explica, cuando: no hay clip seleccionado, lo seleccionado es un bloque, el clip tiene «Repetir» activado, o el cursor no está dentro del clip a más de `MIN_CLIP_FRAMES` de cada borde.
**Criterios de aceptación:**
- CA1: la exportación antes y después de partir un clip es idéntica, salvo los 5 ms de microfundido a cada lado del corte.

**RF-320 [1.0] · Duplicar**
«Duplicar» (`Ctrl+D`) crea una copia del clip o bloque seleccionado que empieza donde termina el original, en la misma pista. La copia queda seleccionada y encima. Es un paso de historial. Si no cabe antes del límite de la canción, se muestra un aviso y no se crea.
**Criterios de aceptación:**
- CA1: duplicar un bloque de los compases 1 a 4 crea otro en los compases 5 a 8 con el mismo patrón.

**RF-321 [1.0] · Eliminar**
«Eliminar» (`Supr`) elimina el clip o bloque seleccionado, en un paso de historial y sin confirmación. El archivo de audio sigue en disco.
**Criterios de aceptación:**
- CA1: tras eliminar un clip y pulsar Deshacer, el clip vuelve con su sonido.

**RF-322 [1.0] · Microfundidos**
El motor DEBE aplicar un fundido lineal de entrada de `MICROFADE_FRAMES` al principio y uno de salida al final de cada tramo audible de un clip: en sus bordes, en los puntos donde otro clip lo tapa o lo destapa y en las uniones entre repeticiones. Si el tramo mide menos del doble, cada fundido dura la mitad del tramo. Los microfundidos no se ven ni se editan. Los sonidos de un bloque no se cortan al terminar el bloque: se dejan sonar hasta su final.
**Criterios de aceptación:**
- CA1: en una exportación, la primera y la última muestra de cada tramo audible de un clip valen 0.

**RF-323 [1.0] · Aspecto de los clips**
Un clip DEBE mostrar: la forma de onda de su región (repetida si tiene «Repetir»), el nombre de su archivo de audio («Toma N» o el nombre del archivo importado) y las marcas de repetición. Mientras los datos de la onda no están listos, se muestra un rectángulo liso. Si el archivo falta, el clip muestra un rayado y el texto «Archivo no encontrado».
**Criterios de aceptación:**
- CA1: recortar un clip actualiza la onda sin volver a leer el archivo de audio.

**RF-324 [1.0] · Edición con el teclado**
Con un elemento seleccionado y el foco en la línea de tiempo:
- `←` y `→` lo mueven: un clip, un paso si el ajuste a la rejilla está activado o 30 ticks si no; un bloque, un compás.
- `Mayús+←` y `Mayús+→` mueven su borde derecho la misma unidad.
- `Alt+←` y `Alt+→` mueven el borde izquierdo de un clip sin «Repetir».
- `Tab` y `Mayús+Tab` seleccionan el elemento siguiente o anterior del carril; `↑` y `↓`, el más cercano del carril de arriba o de abajo.

Cada pulsación que cambia el proyecto es un paso de historial.
**Criterios de aceptación:**
- CA1: toda operación de RF-308 y RF-316 a RF-321 se puede hacer sin ratón.

**RF-350 [1.1] · Fundidos ajustables**
Cada clip tiene un fundido de entrada y uno de salida, lineales, de entre 0 y la mitad de la longitud del clip, que se ajustan arrastrando un tirador en cada esquina superior y se dibujan como una diagonal. Con valor 0 se aplica el microfundido (RF-322).
**Criterios de aceptación:**
- CA1: con un fundido de entrada de 1 segundo, la ganancia a los 0,5 segundos es 0,5.

**RF-351 [1.1] · Fundido cruzado**
Cuando un clip con fundido de entrada mayor que 0 tapa a otro, durante ese fundido el clip tapado sigue sonando con el fundido complementario; lo mismo ocurre con el fundido de salida.
**Criterios de aceptación:**
- CA1: con dos clips del mismo tono continuo y un fundido cruzado, la suma de ganancias es 1 durante todo el cruce.

**RF-352 [1.1] · Volumen por clip**
La barra de herramientas añade el deslizador «Volumen del clip», de −24 a +24 dB (inicial 0), que actúa sobre el clip seleccionado.
**Criterios de aceptación:**
- CA1: un clip a −6 dB suena con ganancia 0,501.

**RF-353 [1.1] · Marcadores de sección**
«Añadir marcador» crea un marcador llamado «Sección N» en el principio del compás que contiene el cursor. Los marcadores se dibujan en la regla, se pueden renombrar, arrastrar a otro compás y eliminar, y entran en el historial. Un clic en un marcador lleva el cursor a su compás.
**Criterios de aceptación:**
- CA1: los marcadores se guardan con el proyecto.

---

## 7. Pestaña Mezcla

**Contexto.** Esta pestaña muestra un canal por pista, en el orden de las pistas, y el canal maestro a la derecha. Es el único lugar donde están el volumen, el panorama, silenciar, solo, los efectos y los medidores de las pistas (DEC-28). Los efectos de cada canal son fijos y van en un orden fijo (DEC-40). Todo el procesamiento es estéreo: una fuente mono se trata como dos canales iguales.

**RF-400 [1.0] · Contenido de la pestaña Mezcla**
La pestaña DEBE mostrar un canal por pista, de izquierda a derecha en el orden de las pistas, y el canal maestro fijo en el extremo derecho. Si los canales no caben, se desplazan en horizontal y el maestro permanece visible. Cada canal contiene, de arriba abajo: nombre y tipo de la pista, sección «Ecualizador», sección «Reverb», «Panorama», «Silenciar» y «Solo», y «Volumen» con su medidor.
**Criterios de aceptación:**
- CA1: con 16 pistas, el canal maestro sigue visible sin desplazar.

**RF-401 [1.0] · Volumen**
Deslizador vertical entre `FADER_MIN_DB` y `FADER_MAX_DB`, con valor inicial 0 dB y pasos de 0,5 dB. En la posición más baja la ganancia es 0 y el valor se muestra como «−∞». El valor se muestra en dB con un decimal. Los cambios se aplican sin clics, con una rampa de 10 ms.
**Criterios de aceptación:**
- CA1: a −6,0 dB la señal sale con ganancia 0,501 (±0,001).
- CA2: en la posición más baja, la pista aporta silencio exacto.

**RF-402 [1.0] · Panorama**
Deslizador horizontal de −100 a +100 en pasos de 1, con valor inicial 0. Se muestra como «C» en 0, «I n» a la izquierda y «D n» a la derecha. La ganancia del canal izquierdo es 1 si `pan ≤ 0` y `1 − pan/100` si no; la del derecho es 1 si `pan ≥ 0` y `1 + pan/100` si no.
**Criterios de aceptación:**
- CA1: con panorama +100, el canal izquierdo de la pista es silencio exacto.
- CA2: con panorama −50, el canal derecho sale a la mitad de amplitud.

**RF-403 [1.0] · Silenciar y solo**
Cada canal de pista tiene los botones de dos estados «Silenciar» y «Solo». En modo canción y al exportar, una pista suena si y solo si no está silenciada y, además, ninguna pista tiene «Solo» o ella lo tiene.
**Criterios de aceptación:**
- CA1: con «Solo» en la pista 2, solo suena la pista 2.
- CA2: una pista con «Silenciar» y «Solo» a la vez no suena.

**RF-404 [1.0] · Medidores y aviso de saturación**
Cada canal y el canal maestro DEBEN mostrar un medidor de pico estéreo (dos barras, izquierda y derecha) de −60 a +6 dBFS, actualizado `TELEMETRY_HZ` veces por segundo, con una marca que retiene el pico máximo durante 1 segundo. El medidor de una pista mide después de su volumen y su panorama; el del maestro, después del volumen maestro. Un indicador con el texto «SAT» se enciende cuando el pico alcanza o supera 0 dBFS y sigue encendido hasta que se pulsa o hasta que empieza una nueva reproducción.
**Criterios de aceptación:**
- CA1: un tono de amplitud 0,5 marca −6,0 dBFS (±0,2).
- CA2: el indicador «SAT» sigue encendido después de parar.

**RF-405 [1.0] · Canal maestro**
El canal maestro tiene un deslizador de volumen (como RF-401), un medidor estéreo y el indicador «SAT». No tiene panorama, silenciar, solo ni efectos. Después del volumen maestro, la salida se limita a ±1,0.
**Criterios de aceptación:**
- CA1: bajar el maestro 6 dB baja 6 dB todos los canales de la salida.

**RF-406 [1.0] · Ecualizador**
Cada canal de pista tiene un botón de dos estados «Ecualizador», inicialmente desactivado, y tres deslizadores entre `EQ_GAIN_MIN_DB` y `EQ_GAIN_MAX_DB` en pasos de 0,5 dB, con valor inicial 0:
- «Graves»: realce o recorte en estante por debajo de `EQ_LOW_HZ`;
- «Medios»: realce o recorte en campana centrado en `EQ_MID_HZ`;
- «Agudos»: realce o recorte en estante por encima de `EQ_HIGH_HZ`.

Desactivado, la señal pasa sin cambios y los valores se conservan.
**Criterios de aceptación:**
- CA1: con «Graves» a +12 dB, un tono de 50 Hz sale 12 dB (±0,5) más fuerte y uno de 5 000 Hz, igual (±0,5).
- CA2: con los tres valores a 0 y el ecualizador activado, la salida es igual a la entrada (±0,001).

**RF-407 [1.0] · Reverb**
Cada canal de pista tiene un botón de dos estados «Reverb», inicialmente desactivado, y dos deslizadores de 0 a 100 % en pasos de 1: «Cantidad» (inicial 20), que regula cuánta reverberación se suma a la señal original, y «Tamaño» (inicial 50), que regula lo que tarda en apagarse. La señal original pasa siempre sin cambios. Al desactivarla, la reverberación se apaga con un fundido de 20 ms.
**Criterios de aceptación:**
- CA1: con «Cantidad» a 0, la salida es igual a la entrada.
- CA2: un golpe seco con «Tamaño» a 100 tarda más en caer 60 dB que con «Tamaño» a 0.

**RF-408 [1.0] · Orden de la señal**
En cada pista el orden es fijo: fuente (clips, o sonidos del kit con el volumen de su fila) → Ecualizador → Reverb → Volumen → Panorama → medidor → suma con las demás pistas. En el canal maestro: suma → Volumen → limitación a ±1,0 → medidor → salida. En la 1.1, Compresor y Delay se insertan entre Ecualizador y Reverb, y el Limitador, después del volumen maestro.
**Criterios de aceptación:**
- CA1: con el volumen de una pista a «−∞», su reverb tampoco se oye.

**RF-450 [1.1] · Compresor**
Sección «Compresor» en cada canal de pista: botón de dos estados (inicial desactivado), «Umbral» (−40 a 0 dB, inicial −18), «Ratio» (1 a 10, inicial 3) y «Compensación» (0 a +12 dB, inicial 0). Ataque de 10 ms y liberación de 150 ms, fijos. Muestra en texto la reducción de ganancia actual en dB.
**Criterios de aceptación:**
- CA1: con umbral −20 dB y ratio 4, un tono de −8 dBFS sale a −17 dBFS (±1).

**RF-451 [1.1] · Delay**
Sección «Delay» en cada canal de pista: botón de dos estados (inicial desactivado), «Tiempo» (negra, corchea con puntillo, corchea o semicorchea; inicial corchea), «Realimentación» (0 a 80 %, inicial 30) y «Cantidad» (0 a 100 %, inicial 25). El tiempo sigue al tempo del proyecto.
**Criterios de aceptación:**
- CA1: a 120 negras por minuto con «Tiempo» en corchea, el primer eco llega 250 ms después.

**RF-452 [1.1] · Limitador**
El canal maestro añade el botón de dos estados «Limitador» (inicial desactivado). Activado, impide que la salida supere −0,3 dBFS, con una anticipación de 5 ms. El medidor y «SAT» del maestro miden después del limitador.
**Criterios de aceptación:**
- CA1: con una mezcla que alcanzaría +6 dBFS, la salida no supera −0,3 dBFS.

---

## 8. Audio y dispositivos

**Contexto.** El motor trabaja siempre a 48 000 Hz y usa el sistema de sonido estándar de Windows en modo compartido, sin ASIO (DEC-19). El programa no deja oír la entrada mientras se graba (DEC-17): el músico se oye al natural y el programa compensa el retardo del equipo al colocar la grabación. Ese retardo de ida y vuelta se mide con una calibración. El micrófono solo se abre cuando hace falta (DEC-38).

**RF-500 [1.0] · Dispositivos al arrancar**
Al arrancar, el programa DEBE usar los dispositivos de salida y de entrada guardados en los ajustes si existen; si no hay guardados o ya no existen, usa los predeterminados de Windows. Si un dispositivo guardado ya no existe, muestra un aviso que lo nombra.
**Criterios de aceptación:**
- CA1: con el archivo de ajustes borrado, el programa reproduce por el dispositivo predeterminado de Windows sin abrir Ajustes (PD-04).

**RF-501 [1.0] · Elegir dispositivos**
En Ajustes hay una lista «Salida» y una lista «Entrada» con los dispositivos que ofrece Windows y, en primer lugar, la opción «Predeterminado de Windows». El cambio se aplica al momento, para la reproducción si la hay y se guarda en los ajustes.
**Criterios de aceptación:**
- CA1: tras elegir otra salida, la siguiente reproducción suena por ella.

**RF-502 [1.0] · Conversión de frecuencia**
Si un dispositivo trabaja a una frecuencia distinta de 48 000 Hz, el programa DEBE convertir el audio en la frontera con el dispositivo. NO DEBE pedir al usuario que cambie la configuración de Windows.
**Criterios de aceptación:**
- CA1: con la salida a 44 100 Hz, un tono de 440 Hz se oye a 440 Hz.
- CA2: con la entrada a 44 100 Hz, un tono de 440 Hz grabado mide 440 Hz (±1) en la toma.

**RF-503 [1.0] · Sin dispositivo**
- **Sin salida**: el proyecto se puede editar y guardar. Reproducir y Grabar muestran un diálogo de error que lo dice, con el botón «Abrir ajustes». El indicador de estado del audio muestra «Audio: sin salida».
- **Sin entrada**: Armar y Grabar muestran un diálogo de error que pide conectar un micrófono, con los botones «Reintentar» (vuelve a buscar dispositivos) y «Abrir ajustes».
**Criterios de aceptación:**
- CA1: sin salida, todas las órdenes de edición y de archivo funcionan.

**RF-504 [1.0] · Desconexión de un dispositivo en uso**
- Si desaparece la **salida** en uso: la reproducción o la grabación terminan (conservando la toma, RF-314); el programa pasa a la salida predeterminada de Windows, si existe, y muestra un diálogo que nombra el dispositivo perdido y el nuevo; si no existe, queda en el estado «sin salida» de RF-503.
- Si desaparece la **entrada** con una pista armada: la pista se desarma y se muestra un aviso.

Cuando un dispositivo desconectado vuelve, el programa no cambia a él por su cuenta.
**Criterios de aceptación:**
- CA1: tras desconectar los auriculares USB durante la reproducción, el programa sigue respondiendo y se puede guardar.

**RF-505 [1.0] · Medidor de entrada**
La cabecera de la pista armada DEBE mostrar un medidor de pico mono de la entrada, de −60 a 0 dBFS, actualizado `TELEMETRY_HZ` veces por segundo, y un indicador «SAT» que se enciende cuando la entrada alcanza −0,1 dBFS. Su explicación indica que hay que bajar el nivel del micrófono en Windows.
**Criterios de aceptación:**
- CA1: en silencio, el medidor marca por debajo de −40 dBFS con un micrófono normal; hablando, se mueve.

**RF-506 [1.0] · Calibración del retardo**
En Ajustes, «Calibrar retardo» DEBE mostrar la instrucción de acercar los auriculares al micrófono y subir el volumen y, al confirmar, emitir 8 clics, uno cada 0,5 segundos, por la salida mientras graba la entrada; después calcula el retardo de ida y vuelta (06).
- Si los clics se detectan con claridad: muestra «Retardo medido: N ms» y guarda el valor para la pareja de dispositivos de entrada y salida en uso.
- Si no: muestra un diálogo de error que pide acercar los auriculares y subir el volumen; el valor no cambia.

Dura menos de 6 segundos. Si se está reproduciendo, antes se para.
**Criterios de aceptación:**
- CA1: con un cable que une la salida con la entrada, dos calibraciones seguidas difieren en menos de 2 ms.
- CA2: con el micrófono silenciado, la calibración falla con el diálogo de error y el valor anterior se conserva.

**RF-507 [1.0] · Retardo manual y valor inicial**
Ajustes muestra el campo «Retardo de grabación (ms)», entero de 0 a 500, con el valor en uso para la pareja de dispositivos actual. Se puede escribir a mano. Mientras una pareja no tenga valor calibrado ni escrito, se usa `DEFAULT_LATENCY_FRAMES` (50 ms). La primera vez que termina una grabación con una pareja sin valor propio, DEBE mostrarse una sola vez un cuadro que avise de que la grabación puede quedar desplazada, con los botones «Calibrar ahora» (abre Ajustes) y «Más tarde».
**Criterios de aceptación:**
- CA1: con el retardo a 0 ms y a 100 ms, el mismo sonido grabado queda en posiciones que difieren en 100 ms (±1).
- CA2: el cuadro de aviso no vuelve a aparecer para esa pareja de dispositivos.

**RF-508 [1.0] · Indicador de estado del audio**
A la derecha de la barra de estado se muestra uno de estos textos: «Audio: correcto», «Audio: sin salida» o «Audio: N cortes», donde N es el número de cortes de audio desde que empezó la última reproducción. Su explicación dice qué es un corte y sugiere cerrar otros programas.
**Criterios de aceptación:**
- CA1: al empezar una nueva reproducción, el contador vuelve a 0.

**RF-509 [1.0] · La entrada no se oye**
La señal de entrada NO DEBE enviarse nunca a la salida.
**Criterios de aceptación:**
- CA1: con una pista armada y hablando al micrófono, la salida del programa es silencio.

---

## 9. Exportación

**Contexto.** Exportar genera un archivo con la mezcla de la canción. No se hace reproduciendo en tiempo real: el motor calcula el sonido tan rápido como puede (DEC-27). Solo hay dos formatos y ninguna otra opción (DEC-31).

**RF-600 [1.0] · Exportar audio**
«Archivo > Exportar audio…» (`Ctrl+E`) DEBE mostrar un cuadro con el formato —«WAV (48 kHz, 24 bits)», elegido por defecto, o «MP3 (320 kbps)»— y, al aceptar, el cuadro nativo de guardar archivo, con el nombre del proyecto como nombre propuesto y `Documentos\Carabo` como carpeta inicial (el programa la crea si no existe). Está desactivado durante la grabación; si se está reproduciendo, antes se para. Si la canción no tiene ningún bloque ni clip, muestra un diálogo de error y no exporta.
**Criterios de aceptación:**
- CA1: el cuadro no ofrece ninguna otra opción (PD-06).

**RF-601 [1.0] · Qué se exporta**
El archivo DEBE contener la canción desde el compás 1 hasta el final del último bloque o clip más `EXPORT_TAIL_FRAMES`, en estéreo, exactamente como suena en modo canción con la mezcla actual (volúmenes, panoramas, silenciar, solo, efectos y volumen maestro), sin claqueta y con independencia del bucle y del cursor. Los valores fuera de ±1,0 se recortan.
- WAV: 48 000 Hz, PCM de 24 bits, estéreo.
- MP3: 48 000 Hz, 320 kbps constantes, estéreo.
**Criterios de aceptación:**
- CA1: un proyecto con un bloque de 4 compases a 120 negras por minuto en 4/4 produce un archivo de 10,0 segundos (8 de canción y 2 de cola).
- CA2: exportar dos veces el mismo proyecto a WAV produce dos archivos idénticos (RNF-14).
- CA3: el WAV exportado coincide, muestra a muestra y con un error menor que 1/8 388 608, con la salida del motor en modo canción.

**RF-602 [1.0] · Progreso y cancelación**
Durante la exportación se muestra un cuadro con el porcentaje y el botón «Cancelar». Al cancelar, el archivo incompleto se borra. Al terminar, el cuadro muestra «Exportación terminada» con los botones «Abrir carpeta» (abre el Explorador en la carpeta del archivo) y «Cerrar». Si no se puede escribir en el destino, se muestra un diálogo de error.
**Criterios de aceptación:**
- CA1: tras cancelar no queda ningún archivo en el destino.

**RF-603 [1.0] · Aviso de saturación al exportar**
Si durante la exportación se ha recortado alguna muestra, el cuadro final DEBE añadir un aviso que diga que la mezcla satura y que conviene bajar el volumen maestro y volver a exportar.
**Criterios de aceptación:**
- CA1: una mezcla cuyo pico es +3 dBFS muestra el aviso; una cuyo pico es −1 dBFS, no.

---

## 10. Ajustes, idioma y atajos

**Contexto.** Carabo casi no tiene preferencias (PD-06). La ventana de ajustes contiene solo el idioma, los dispositivos de audio, el retardo de grabación y la información del programa. Los ajustes son del programa, no del proyecto, y no entran en el historial.

**RF-700 [1.0] · Ventana de ajustes**
El botón «Ajustes» de la barra superior abre un cuadro con tres secciones:
- **Idioma** (RF-701).
- **Audio**: «Salida» y «Entrada» (RF-501), «Probar salida» (emite un tono de 440 Hz a −12 dBFS durante 1 segundo), medidor de entrada, «Calibrar retardo» (RF-506) y «Retardo de grabación (ms)» (RF-507).
- **Acerca de** (RF-704).

Los cambios se aplican al momento; el cuadro solo tiene el botón «Cerrar». Mientras está abierto, la entrada está abierta para alimentar su medidor. Está desactivado durante la grabación.
**Criterios de aceptación:**
- CA1: el cuadro no contiene ningún otro control configurable (PD-06).

**RF-701 [1.0] · Idioma**
Se elige entre «Español» y «English». El valor inicial es español si el idioma de Windows es español e inglés en cualquier otro caso. El cambio se aplica al momento a todos los textos, sin reiniciar. Los nombres que el programa pone por defecto («Ritmo 1», «Patrón 1», «Audio 1») usan el idioma activo al crearlos y no cambian después.
**Criterios de aceptación:**
- CA1: tras cambiar a inglés no queda ningún texto visible en español, salvo los nombres que el usuario o el programa pusieron antes.

**RF-702 [1.0] · Persistencia de los ajustes**
Los ajustes se guardan en un archivo JSON en la carpeta de configuración de la aplicación (05), con escritura atómica, y se leen al arrancar. Si no se puede leer, se usan los valores iniciales y se anota en el registro.
**Criterios de aceptación:**
- CA1: tras cambiar el idioma y reiniciar, el programa arranca en el idioma elegido.

**RF-703 [1.0] · Atajos de teclado**
Los atajos son fijos y no se pueden cambiar. Los atajos de una sola tecla (`Espacio`, `R`, `S`, `Supr`, `Inicio`, `F2`, flechas) no actúan mientras un campo de texto tiene el foco.

| Atajo | Acción | Requisito |
|---|---|---|
| `Espacio` | Reproducir o parar | RF-101 |
| `R` | Grabar | RF-311 |
| `Inicio` | Llevar el cursor al compás 1 | RF-104 |
| `Ctrl+Z` / `Ctrl+Y` | Deshacer / Rehacer | RF-030 |
| `Ctrl+N` / `Ctrl+O` | Nuevo proyecto / Abrir proyecto | RF-020, RF-021 |
| `Ctrl+S` / `Ctrl+Mayús+S` | Guardar / Guardar como | RF-023, RF-024 |
| `Ctrl+E` | Exportar audio | RF-600 |
| `Ctrl+1` / `Ctrl+2` / `Ctrl+3` | Pestaña Ritmo / Canción / Mezcla | RF-003 |
| `F2` | Renombrar el patrón seleccionado (pestaña Ritmo) | RF-208 |
| `S` | Partir el clip seleccionado en el cursor | RF-319 |
| `Ctrl+D` | Duplicar el elemento seleccionado | RF-320 |
| `Supr` | Eliminar el elemento seleccionado | RF-321 |
| `←` `→`, con `Mayús` o `Alt` | Mover o redimensionar el elemento seleccionado | RF-324 |
| `Ctrl++` / `Ctrl+−` | Acercar / Alejar (pestaña Canción) | RF-305 |
| `Esc` | Cerrar el cuadro abierto o cancelar el gesto en curso | RF-032 |
| `Alt+F4` | Salir (atajo estándar de Windows para cerrar la ventana) | RF-010 |

**Criterios de aceptación:**
- CA1: no existe ningún atajo que no figure en esta tabla (en la 1.1 se añade `Ctrl+4` para la pestaña Afinador).
- CA2: escribir «rs» en el campo de nombre de un patrón no graba ni parte nada.

**RF-704 [1.0] · Acerca de**
La sección muestra el nombre del programa, su versión, la licencia (MIT) y la dirección del repositorio como texto.
**Criterios de aceptación:**
- CA1: la versión mostrada coincide con la del paquete compilado.

---

## 11. Pestaña Afinador

**Contexto.** Es la cuarta pestaña y llega en la versión 1.1 (DEC-07). Usa el dispositivo de entrada para mostrar qué nota suena.

**RF-800 [1.1] · Pestaña Afinador**
La pestaña «Afinador» (`Ctrl+4`) DEBE mostrar, mientras está activa: el nombre de la nota detectada con su octava (en español, «Mi 2»; en inglés, «E2»), la desviación en céntimos de −50 a +50 con una aguja, y un texto: «Bajo», «Afinado» (dentro de ±5 céntimos) o «Alto». Sin una nota clara muestra «—». La referencia es La 4 = 440 Hz y no se puede cambiar. Mientras la pestaña está activa, la entrada está abierta. «Grabar» está desactivado en esta pestaña, como en Ritmo.
**Criterios de aceptación:**
- CA1: un tono de 110 Hz muestra «La 2», 0 céntimos y «Afinado».

**RF-801 [1.1] · Detección de la nota**
La detección DEBE cubrir de 70 a 1 000 Hz, con un error máximo de ±2 céntimos para un tono estable, al menos 10 lecturas por segundo y una respuesta en menos de 200 ms.
**Criterios de aceptación:**
- CA1: un tono de 82,41 Hz se muestra como «Mi 2» con una desviación entre −2 y +2 céntimos.

---

## 12. Requisitos no funcionales

**Contexto.** Los requisitos de rendimiento se miden en el **equipo de referencia** (RNF-02) con el **proyecto de carga máxima**: 4 pistas de ritmo, cada una con un patrón de 4 compases con golpes en todos los pasos de sus 8 filas y un bloque que dura los 10 minutos, y 12 pistas de audio, cada una con un clip de 10 minutos; ecualizador y reverb activados en las 16 pistas. El documento 08 dice cómo generarlo.

| Id | Requisito | Cómo se comprueba |
|---|---|---|
| **RNF-01** Plataforma | Windows 11 de 64 bits. Usa el componente WebView2 incluido en Windows 11. No necesita permisos de administrador. | Arranque en una instalación limpia de Windows 11. |
| **RNF-02** Equipo de referencia | Procesador de 4 núcleos, 8 GB de memoria, disco SSD y pantalla de 1920 × 1080. Todo requisito de rendimiento se mide en un equipo igual o inferior. | — |
| **RNF-03** Reproducción sin cortes | El proyecto de carga máxima suena 10 minutos seguidos en modo canción con 0 cortes de audio. | PA-03 (08). |
| **RNF-04** Carga del motor | El motor calcula el proyecto de carga máxima, sin dispositivo, al menos 4 veces más rápido que el tiempo real. | Prueba de rendimiento automática (08). |
| **RNF-05** Respuesta de la interfaz | Toda pulsación produce una respuesta visible en menos de 100 ms. Los arrastres y el cursor se dibujan a 30 imágenes por segundo o más con el proyecto de carga máxima. | PA-03 (08). |
| **RNF-06** Arranque y apertura | La pantalla de inicio aparece en menos de 3 segundos. El proyecto de carga máxima se puede editar y reproducir en menos de 3 segundos desde que se abre; las ondas pueden aparecer después, y el audio de los clips empieza a sonar cuando termina de cargarse en memoria: en menos de 10 segundos con el proyecto de carga máxima (06 §9.1). | PA-03 (08). |
| **RNF-07** Memoria | El programa ocupa menos de 700 MB sin contar el audio de los clips. El audio que suena se carga entero en memoria (06 §9.1) y ocupa 11,5 MB por minuto de audio mono y 23 MB por minuto estéreo: con el proyecto de carga máxima, cuyos clips son mono, el total queda por debajo de 2,1 GB. | PA-03 (08). |
| **RNF-08** Velocidad de exportación | El proyecto de carga máxima se exporta a WAV en menos de 150 segundos. | Prueba de rendimiento automática (08). |
| **RNF-09** Precisión de la grabación | Con la pareja de dispositivos calibrada, un clic grabado por el micrófono queda a menos de 5 ms de su posición original en tomas de hasta 60 segundos. | PA-02 (08). |
| **RNF-10** Precisión del ritmo | Cada golpe y cada principio de clip empieza en el frame que da la fórmula del documento 06, con un error máximo de 1 frame. | Pruebas automáticas del motor (08). |
| **RNF-11** Reglas del hilo de audio | En el hilo de audio no se reserva ni se libera memoria, no se usan cerrojos, no se lee ni se escribe en disco, consola o red, y no se hace ninguna llamada que pueda bloquear (06). | Revisión de código y prueba con un asignador de memoria que falla si se usa en ese hilo (08). |
| **RNF-12** Robustez | Un error interno del motor no cierra el programa: el motor lo comunica, el sonido se para y la interfaz sigue respondiendo, de modo que se puede guardar. | Pruebas automáticas de inyección de errores (08). |
| **RNF-13** Integridad de los archivos | El archivo de proyecto, la copia de recuperación y los ajustes se escriben de forma atómica: primero en un archivo temporal de la misma carpeta y después se sustituye el definitivo. Un corte de corriente deja el archivo anterior o el nuevo, completos. | Prueba automática que interrumpe la escritura (08). |
| **RNF-14** Determinismo | Calcular dos veces la misma instantánea produce exactamente las mismas muestras, en el mismo equipo y con el mismo programa compilado. | Prueba automática (08). |
| **RNF-15** Usabilidad | Se supera la prueba de los cinco minutos (01 §5) con dos participantes. | PA-01 (08). |
| **RNF-16** Accesibilidad | Toda orden se puede ejecutar con el teclado; el foco es siempre visible; el contraste cumple WCAG 2.1 AA (4,5:1 en texto y 3:1 en el resto); ningún dato se comunica solo con color; todo control tiene nombre y estado accesibles. No se exige compatibilidad completa con lectores de pantalla. | Pruebas automáticas de accesibilidad y PA-04 (08). |
| **RNF-17** Idiomas | El 100 % de los textos visibles existe en español y en inglés en los archivos de traducción. No hay textos visibles dentro del código. Los números se formatean según el idioma. | Prueba automática que compara las claves de ambos idiomas (08). |
| **RNF-18** Privacidad y seguridad | Ninguna conexión de red. El micrófono solo se abre con una pista armada, en la calibración, en Ajustes y en el afinador. La aplicación solo accede a las carpetas de proyecto, a sus propios datos y a los archivos que el usuario elige. No se carga contenido remoto. | Revisión de la configuración (07). |
| **RNF-19** Licencias | Todas las dependencias y los sonidos incluidos son compatibles con la licencia MIT del proyecto (01 §8). | Comprobación automática de licencias (07). |
| **RNF-20** Registro | El programa escribe un archivo de registro en sus datos, con los errores y avisos y su fecha y hora, que rota al llegar a 1 MB y conserva 3 archivos. Nunca se escribe desde el hilo de audio. No contiene audio. | Revisión y prueba automática (08). |
| **RNF-21** Cobertura | Cada criterio de aceptación de un RF [1.0] está cubierto por una prueba automática o por un paso de una prueba de aceptación manual. | Tabla de trazabilidad (08). |
| **RNF-22** Límites | Las constantes de 00 §4 se hacen cumplir en la interfaz y, de nuevo, en el motor. | Pruebas automáticas (08). |

---

## 13. Requisitos por versión

- **[1.0]**: todos los RF de este documento salvo los enumerados a continuación, y todos los RNF.
- **[1.1]**: RF-033, RF-115, RF-250, RF-251, RF-252, RF-350, RF-351, RF-352, RF-353, RF-450, RF-451, RF-452, RF-800 y RF-801.
