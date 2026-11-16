# Carabo

DAW de escritorio para Windows 11. Interfaz en TypeScript + React dentro de Tauri 2 (`src/`),
programa nativo en Rust (`src-tauri/`) y motor de audio en Rust (`engine/`, crate `carabo-engine`).

## Antes de escribir código

1. La especificación está en `docs/spec/` y es la única fuente de verdad. No la modifiques.
2. Lee entero `docs/spec/00-indice.md`.
3. Busca tu tarea (`T-nnn`) en `docs/spec/09-plan-de-implementacion.md` y lee solo las secciones
   que cita en «Leer antes». No leas los 11 documentos enteros: usa el mapa de 00 §2.

## Reglas

- Haz lo que dice la tarea y nada más. No añadas funciones, opciones ni dependencias que la
  especificación no nombre (`07-tecnologia-y-repositorio.md` §2).
- Si falta algo o hay una contradicción, aplica `00-indice.md` §6 y anótalo en
  `docs/DECISIONES-DE-IMPLEMENTACION.md`. No lo resuelvas en silencio.
- Código, identificadores, comentarios y commits en inglés. Los textos de la interfaz, solo en
  `src/i18n/es.json` y `src/i18n/en.json`.
- Respeta las dependencias entre módulos (`04-arquitectura.md` §9). Solo
  `src/bridge/tauriBridge.ts` importa `@tauri-apps/*`; `src/ui/` nunca importa `src/bridge/`.
- No toques los archivos que la tarea marca en «No tocar».
- Una tarea está terminada cuando se cumplen sus criterios de aceptación y pasa la comprobación
  completa.

## Órdenes (PowerShell, en la raíz)

- Instalar: `npm ci`
- Interfaz en el navegador con el puente simulado: `npm run dev`
- Aplicación completa: `npm run tauri dev`
- Comprobación completa, en este orden:
  `npm run lint` · `npm run typecheck` · `npm test` · `npm run test:e2e` · `npm run build` ·
  `cargo fmt --all --check` · `cargo clippy --workspace --all-targets -- -D warnings` ·
  `cargo test --workspace` · `npm run licenses`
- Oír el motor sin tarjeta de sonido:
  `cargo run -p carabo-engine --bin render -- <instantánea.json> <salida.wav>`

## Dónde está cada cosa

- `src/core/` lógica pura · `src/store/` estado e historial · `src/bridge/` puente ·
  `src/services/` operaciones · `src/ui/` componentes · `src/i18n/` textos
- `src-tauri/` comandos del puente · `engine/` motor de audio
- `assets/` kits y ritmos hechos · `fixtures/` datos de prueba · `tools/` scripts · `e2e/` pruebas

## Git

- Una rama por tarea: `t-nnn-descripcion-corta`. Commits: `T-nnn: summary in English`.
- Termina con una solicitud de integración hacia `main`. No la fusiones ni reescribas `main`.

## Entorno

Esta máquina puede no tener tarjeta de sonido. Las pruebas automáticas no la necesitan. Lo que
exige oír o grabar se comprueba en las pruebas de aceptación manuales (`08-pruebas.md`), no aquí:
no des por comprobado algo que no has podido ejecutar; dilo en la solicitud de integración.

## Compact Instructions

Al compactar la conversación, conserva: el identificador de la tarea, los criterios de aceptación
que faltan, los archivos modificados y las decisiones anotadas en
`docs/DECISIONES-DE-IMPLEMENTACION.md`.
