# Carabo · Especificación · 07 — Tecnología y repositorio

> **PROMPT PARA EL AGENTE — léelo antes de seguir**
>
> Eres un agente de programación que trabaja en el repositorio de **Carabo**, un DAW de escritorio para Windows 11. Este documento dice **con qué se construye y dónde va cada archivo**: tecnologías y versiones, árbol de carpetas, archivos de configuración, cómo se usa Tauri, convenciones de código, órdenes de compilación, integración continua, licencias y las instrucciones de Claude Code.
>
> - Léelo antes de crear un archivo, añadir una dependencia o tocar una configuración.
> - **No añadas ninguna dependencia que no figure en §2.** Si una tarea parece necesitar otra, aplica 00 §6: resuélvelo sin ella o anota la decisión.
> - Las versiones de §2 se comprobaron el 15 de noviembre de 2026. La regla de qué versión exacta instalar está en §2.1.
> - Los nombres de funciones de bibliotecas externas que aparecen aquí (Tauri, sobre todo) describen la versión comprobada. Si la versión instalada los nombra de otro modo, manda su documentación; lo que no cambia es el comportamiento que exigen 04 y 06.
> - Los módulos y sus responsabilidades están en 04; el interior del motor, en 06; las pruebas, en 08.

---

## 1. La pila en una tabla

| Capa | Tecnología | Para qué |
|---|---|---|
| Ventana y empaquetado | **Tauri 2** | Muestra la interfaz en el componente WebView2 de Windows y la une con el programa nativo |
| Interfaz | **TypeScript** + **React** + **Vite** | Todo lo que se ve. Carpeta `src/` |
| Estado de la interfaz | **Zustand** + **Immer** | Almacenes y parches del historial (04 §3.2) |
| Textos | **i18next** + **react-i18next** | Español e inglés (RF-701) |
| Validación del proyecto | **Ajv** | Esquema JSON de 05 §5.2 |
| Programa nativo | **Rust** + crate `tauri` | Comandos del puente, archivos, ajustes, registro. Carpeta `src-tauri/` |
| Motor de audio | **Rust**, crate propio `carabo-engine` | Carpeta `engine/` (06) |
| Dispositivos de audio | `cpal` (WASAPI en modo compartido) | Entrada y salida de sonido |
| Archivos de audio | `hound`, `symphonia`, `rubato`, `mp3lame-encoder` | WAV, decodificación, conversión de frecuencia, MP3 |
| Pruebas | **Vitest**, **Testing Library**, **Playwright**, `cargo test` | 08 |
| Control de versiones | **Git** + **GitHub** + **GitHub Actions** | §10 |

Decisiones que no se reabren: Tauri y Rust (DEC-23), solo Windows 11 (DEC-20), sin ASIO (DEC-19), licencia MIT (DEC-25), programado por agentes de Claude Code (DEC-24), Git y GitHub con pruebas automáticas en cada subida (DEC-34).

## 2. Versiones

### 2.1 Regla para elegir la versión exacta

1. De cada dependencia se instala **la última versión estable de la línea indicada** en §2.3 y §2.4 el día en que se añade al repositorio. «Línea 2» significa cualquier versión 2.x.y que no sea alfa, beta ni candidata.
2. La versión exacta queda fijada por los archivos de bloqueo, `package-lock.json` y `Cargo.lock`, que se guardan en Git. Nadie los regenera ni actualiza una dependencia fuera de una tarea que lo pida.
3. **No se cambia de línea.** Si ya existe una línea mayor más nueva (por ejemplo Tauri 3 o TypeScript 7), no se usa.
4. Para saber cómo se llama una función de una biblioteca, se consulta la documentación **de la versión fijada** (en `docs.rs` para los crates), no la memoria ni la última versión.
5. Si una dependencia de la tabla ya no existe o su línea no tiene versión estable, se aplica 00 §6.

### 2.2 Herramientas

| Herramienta | Versión | Dónde se fija |
|---|---|---|
| Node.js | Línea **24** (LTS) | `.nvmrc` (contenido: `24`) y `engines` en `package.json` |
| npm | El que acompaña a Node 24 | — |
| Rust | **Estable, 1.99 o posterior**. La primera tarea ejecuta `rustup update stable`, lee la versión con `rustc --version` y la escribe en `rust-toolchain.toml` | `rust-toolchain.toml` |
| Destino de Rust | `x86_64-pc-windows-msvc` | El de por defecto en Windows |
| Compilador de C | «Build Tools for Visual Studio» con la carga «Desarrollo para el escritorio con C++» | Requisito de Rust en Windows y de la compilación de LAME |
| WebView2 | El incluido en Windows 11 | — |
| Git | Cualquier versión actual | — |

### 2.3 Dependencias de la interfaz (`package.json`)

| Paquete | Línea | Tipo | Uso |
|---|---|---|---|
| `react`, `react-dom` | 19 | Producción | Componentes |
| `zustand` | 5 | Producción | Almacenes |
| `immer` | 10 | Producción | Cambios del documento y parches (`enablePatches`) |
| `i18next`, `react-i18next` | Última estable | Producción | Textos |
| `ajv` | 8 | Producción | Validación con el esquema 2020-12 (`ajv/dist/2020`) |
| `@tauri-apps/api` | 2 | Producción | Comandos, eventos y ventana. Solo lo importa `src/bridge/tauriBridge.ts` |
| `@tauri-apps/plugin-dialog` | 2 | Producción | Cuadros nativos de abrir y guardar |
| `@tauri-apps/plugin-opener` | 2 | Producción | Mostrar un archivo en el Explorador |
| `typescript` | **6.0** (no 7) | Desarrollo | Compilador |
| `vite`, `@vitejs/plugin-react` | Vite 8 y el plugin compatible | Desarrollo | Servidor de desarrollo y empaquetado |
| `vitest` | 5 | Desarrollo | Pruebas unitarias y de componentes |
| `jsdom` | Última estable | Desarrollo | Entorno de las pruebas de componentes |
| `@testing-library/react`, `@testing-library/user-event`, `@testing-library/jest-dom` | Últimas estables | Desarrollo | Pruebas de componentes |
| `@playwright/test` | 1 | Desarrollo | Pruebas de extremo a extremo |
| `@axe-core/playwright` | Última estable | Desarrollo | Comprobaciones de accesibilidad (RNF-16) |
| `eslint`, `typescript-eslint`, `eslint-plugin-react-hooks`, `eslint-plugin-jsx-a11y` | Últimas estables, con configuración plana (`eslint.config.js`) | Desarrollo | Análisis estático |
| `prettier` | 3 | Desarrollo | Formato |
| `@tauri-apps/cli` | 2 | Desarrollo | Orden `tauri` |
| `@types/react`, `@types/react-dom`, `@types/node` | Las de React 19 y Node 24 | Desarrollo | Tipos |

### 2.4 Dependencias de Rust

`engine/Cargo.toml`:

| Crate | Línea | Uso | Licencia |
|---|---|---|---|
| `cpal` | 0.18 | Dispositivos de audio. Sin la característica `asio` | Apache-2.0 |
| `rtrb` | 0.3 | Colas sin bloqueo de un productor y un consumidor | MIT o Apache-2.0 |
| `rubato` | 5 | Conversión de frecuencia | MIT |
| `hound` | 3.5 | Leer y escribir WAV | Apache-2.0 |
| `symphonia` | 0.5, con `default-features = false` y las características `wav`, `pcm` y `mp3` | Decodificar WAV y MP3 al importar | MPL-2.0 |
| `mp3lame-encoder` | 0.2 | Codificar MP3 (compila LAME y lo enlaza de forma estática) | LGPL (§11.3) |
| `serde` (con `derive`), `serde_json` | 1 | Instantánea en JSON | MIT o Apache-2.0 |
| `thiserror` | 2 | Tipo `EngineError` | MIT o Apache-2.0 |
| `tempfile` (solo pruebas) | 3 | Carpetas temporales en las pruebas | MIT o Apache-2.0 |

`src-tauri/Cargo.toml`:

| Crate | Línea | Uso |
|---|---|---|
| `tauri` | 2 | Aplicación, comandos y eventos |
| `tauri-build` (dependencia de compilación) | 2 | Script de compilación |
| `tauri-plugin-dialog`, `tauri-plugin-opener` | 2 | Parte nativa de los dos plugins |
| `carabo-engine` | Ruta `../engine` | El motor |
| `serde` (con `derive`), `serde_json` | 1 | Argumentos y resultados de los comandos |
| `uuid` (con `v4`) | 1 | Nombres de las carpetas `unsaved/<uuid>/` |
| `sys-locale` | Última estable | Idioma de Windows para `app_info` |
| `tempfile` (solo pruebas) | 3 | Carpetas temporales en las pruebas |

### 2.5 Lo que no se usa

- Ninguna biblioteca de componentes de interfaz, ni de estilos (Tailwind, CSS-in-JS), ni de arrastrar y soltar, ni de gráficos. Los controles son los de `src/ui/common/` (03).
- Ningún enrutador: las pestañas son estado de `uiStore`.
- Ninguna biblioteca de red, de telemetría de uso ni de actualización automática (RNF-18).
- Ningún plugin de Tauri fuera de `dialog` y `opener`. En particular, no se usa el plugin `fs`: los archivos los tocan solo los comandos de Rust.
- En Rust, ningún entorno asíncrono propio: se usa el que trae Tauri para los comandos, e hilos normales en el motor.
- Ninguna biblioteca de registro: el registro lo escribe `src-tauri/src/logging.rs` (RNF-20).

## 3. Entorno de desarrollo

Windows 11 de 64 bits, con una terminal de PowerShell en la raíz del repositorio.

Instalación, una sola vez:

1. Node.js 24, Git, `rustup` y las «Build Tools for Visual Studio» (§2.2).
2. `rustup update stable`.
3. En el repositorio: `npm ci`.
4. Para las pruebas de extremo a extremo: `npx playwright install chromium`.

La máquina de desarrollo puede no tener una tarjeta de sonido fiable (es una máquina virtual). Nada de lo que comprueban las pruebas automáticas la necesita (08). Lo que exige oír o grabar se comprueba en las pruebas de aceptación manuales, en un equipo real.

## 4. Árbol del repositorio

```text
carabo/
├── CLAUDE.md                          Instrucciones para Claude Code (§12)
├── README.md                          Qué es, cómo se compila y enlace a docs/spec/
├── LICENSE                            Texto de la licencia MIT
├── THIRD-PARTY-NOTICES.md             Avisos de licencias de terceros (§11.4)
├── package.json
├── package-lock.json
├── .nvmrc
├── tsconfig.json
├── vite.config.ts
├── eslint.config.js
├── .prettierrc.json
├── .prettierignore
├── playwright.config.ts
├── index.html                         Página única; carga src/main.tsx
├── Cargo.toml                         Espacio de trabajo de Rust: engine y src-tauri
├── Cargo.lock
├── rust-toolchain.toml
├── rustfmt.toml
├── .gitignore
├── .gitattributes
├── .github/
│   └── workflows/ci.yml               Integración continua (§10.3)
├── docs/
│   ├── spec/                          Los 11 documentos de la especificación
│   ├── pruebas/                       Registros de las pruebas de aceptación manuales (08 §12)
│   └── DECISIONES-DE-IMPLEMENTACION.md   Registro de huecos resueltos (00 §6)
├── assets/
│   ├── icon.png                       Icono provisional, 1024 × 1024 (§13.5)
│   ├── kits/
│   │   ├── acoustic/                  kit.json y 8 WAV (05 §8.3)
│   │   ├── electronic/
│   │   └── percussion/
│   └── presets/presets.json           Ritmos hechos (05 §9)
├── fixtures/                          Datos de prueba compartidos por TypeScript y Rust (08 §3)
│   ├── constants.json
│   ├── manual-coverage.json           Criterios de aceptación que se comprueban a mano (08 §13.3)
│   ├── formulas/
│   ├── bridge/
│   ├── projects/
│   ├── snapshots/
│   └── golden/
├── tools/                             Scripts de Node sin dependencias (§13), con sus pruebas *.test.mjs
│   ├── gen-placeholder-kits.mjs
│   ├── gen-icon.mjs
│   ├── gen-max-load-project.mjs
│   ├── check-licenses.mjs
│   ├── check-traceability.mjs
│   ├── measure-offset.mjs
│   └── package.mjs
├── src/                               Interfaz (04 §3)
│   ├── main.tsx                       Punto de entrada: i18n, tema y <App />
│   ├── vite-env.d.ts
│   ├── core/
│   │   ├── constants.ts
│   │   ├── model.ts  time.ts  factory.ts  ids.ts  lanes.ts  snapshot.ts
│   │   ├── validate.ts  serialize.ts  kits.ts  presets.ts  peaks.ts
│   │   ├── schema/project-1.schema.json
│   │   └── commands/  tracks.ts  patterns.ts  blocks.ts  clips.ts  mixer.ts  transport.ts
│   ├── store/      projectStore.ts  uiStore.ts  engineStore.ts  settingsStore.ts  engineSync.ts
│   ├── bridge/     bridge.ts  tauriBridge.ts  mockBridge.ts  index.ts
│   ├── services/   projectFiles.ts  playback.ts  recording.ts  importer.ts  exporter.ts
│   │               devices.ts  shortcuts.ts  messages.ts
│   ├── i18n/       index.ts  es.json  en.json
│   ├── ui/
│   │   ├── theme.css                  Variables CSS del tema (03 §2)
│   │   ├── base.css                   Estilos generales de la página
│   │   ├── app/  common/  rhythm/  song/  mix/
│   └── test/                          Utilidades de las pruebas de la interfaz (08 §3)
│       ├── setup.ts
│       ├── builders.ts
│       ├── fixtureRecipes.ts
│       └── fixtures.test.ts
├── e2e/                               Pruebas de extremo a extremo (08 §6): helpers.ts y *.spec.ts
├── engine/                            Crate carabo-engine (06 §2)
│   ├── CLAUDE.md                      Recordatorio de las reglas del hilo de audio (§12.2)
│   ├── Cargo.toml
│   ├── src/
│   └── tests/
└── src-tauri/                         Programa nativo (04 §4)
    ├── Cargo.toml
    ├── build.rs
    ├── tauri.conf.json
    ├── capabilities/default.json
    ├── icons/                         Generados a partir de assets/icon.png
    └── src/
        ├── main.rs                    Solo llama a carabo_lib::run()
        ├── lib.rs  state.rs  error.rs  paths.rs  atomic.rs  logging.rs  telemetry.rs
        └── commands/  mod.rs  project.rs  audio_files.rs  export.rs  engine.rs  devices.rs  system.rs
```

Reglas del árbol:

- Las pruebas unitarias de TypeScript van junto al archivo que prueban, con el nombre `<archivo>.test.ts` o `.test.tsx`.
- Cada componente de `src/ui/` es un archivo `Nombre.tsx` con su hoja de estilos `Nombre.css` al lado.
- No se crean carpetas de primer nivel distintas de estas.
- Carpetas generadas, que no se guardan en Git: `node_modules/`, `dist/`, `target/`, `release/`, `test-results/`, `playwright-report/` y `src-tauri/gen/`.

## 5. Archivos de configuración

Se dan aquí los contenidos que no admiten interpretación. Lo no mostrado se deja con el valor por defecto de la herramienta.

### 5.1 `package.json`

```json
{
  "name": "carabo",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "license": "MIT",
  "engines": { "node": ">=24 <25" },
  "scripts": {
    "dev": "vite",
    "build": "tsc --noEmit && vite build",
    "tauri": "tauri",
    "typecheck": "tsc --noEmit",
    "lint": "eslint . && prettier --check .",
    "format": "prettier --write .",
    "test": "vitest run",
    "test:watch": "vitest",
    "test:e2e": "playwright test",
    "kits": "node tools/gen-placeholder-kits.mjs",
    "licenses": "node tools/check-licenses.mjs",
    "traceability": "node tools/check-traceability.mjs",
    "package": "node tools/package.mjs"
  }
}
```

El campo `version` de este archivo es la **única** fuente de la versión del programa: `tauri.conf.json` la toma de aquí y «Acerca de» la muestra a través de `app_info`. La versión es `0.x.y` durante el desarrollo, `1.0.0` al cerrar la versión 1.0 y `1.1.0` al cerrar la 1.1.

### 5.2 `tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "lib": ["ES2022", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "moduleResolution": "bundler",
    "jsx": "react-jsx",
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "noImplicitOverride": true,
    "noFallthroughCasesInSwitch": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "skipLibCheck": true,
    "noEmit": true,
    "types": ["vite/client", "vitest/globals"]
  },
  "include": ["src", "e2e", "tools", "vite.config.ts", "playwright.config.ts"]
}
```

### 5.3 `vite.config.ts`

```ts
import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  build: { target: "chrome110", outDir: "dist", sourcemap: true },
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: ["src/test/setup.ts"],
    include: ["src/**/*.test.{ts,tsx}", "tools/**/*.test.mjs"],
  },
});
```

### 5.4 ESLint y Prettier

`eslint.config.js` (configuración plana) activa:

- las reglas recomendadas con información de tipos de `typescript-eslint`;
- `react-hooks/rules-of-hooks` y `react-hooks/exhaustive-deps` como error;
- las reglas recomendadas de `jsx-a11y` (RNF-16);
- `@typescript-eslint/no-explicit-any`, `@typescript-eslint/no-floating-promises` y `@typescript-eslint/no-non-null-assertion` como error;
- `no-restricted-imports`, con un bloque por carpeta, para hacer cumplir la tabla de dependencias de 04 §9:

| Archivos | No pueden importar |
|---|---|
| `src/core/**` | `react`, `zustand`, `@tauri-apps/*`, y nada de `src/store`, `src/bridge`, `src/services`, `src/ui`, `src/i18n` |
| `src/bridge/**`, salvo `tauriBridge.ts` | `@tauri-apps/*`, y nada de `src/store`, `src/services`, `src/ui` |
| `src/store/**` | `@tauri-apps/*`, y nada de `src/services`, `src/ui` |
| `src/services/**` | `@tauri-apps/*`, y nada de `src/ui` |
| `src/ui/**` | `@tauri-apps/*`, y nada de `src/bridge` |
| `src/i18n/**` | Nada del proyecto |

En los archivos `*.test.ts(x)`, `src/test/**` y `e2e/**` no se aplica `no-non-null-assertion`.

`.prettierrc.json`: `{ "printWidth": 100 }`. `.prettierignore`: `dist`, `target`, `release`, `src-tauri/gen`, `fixtures`, `assets`, `docs`, `package-lock.json`.

### 5.5 Rust

`Cargo.toml` de la raíz:

```toml
[workspace]
members = ["engine", "src-tauri"]
resolver = "2"

[workspace.package]
edition = "2021"
license = "MIT"

[profile.dev]
opt-level = 1

[profile.dev.package.carabo-engine]
opt-level = 3

[profile.release]
lto = "thin"
```

- El motor se compila optimizado también en desarrollo, porque sin optimizar no calcula a tiempo.
- NO DEBE ponerse `panic = "abort"` en ningún perfil: el motor captura los pánicos para no cerrar el programa (06 §20).

`rust-toolchain.toml`:

```toml
[toolchain]
channel = "<versión estable anotada según §2.2, por ejemplo 1.99.0>"
components = ["rustfmt", "clippy"]
```

`rustfmt.toml`: `max_width = 100`.

Nombres de los paquetes: el motor es `carabo-engine` (biblioteca `carabo_engine`); el programa nativo es `carabo`, con la biblioteca `carabo_lib` (sección `[lib]` con `name = "carabo_lib"` en `src-tauri/Cargo.toml`) y el ejecutable `carabo.exe`. Los dos paquetes heredan `edition` y `license` del espacio de trabajo.

Al principio de `engine/src/lib.rs` y de `src-tauri/src/lib.rs`: `#![forbid(unsafe_code)]`. Al principio de `engine/src/rt/mod.rs` y de `engine/src/dsp/mod.rs`: `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` (06, RT-06).

### 5.6 `src-tauri/tauri.conf.json`

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Carabo",
  "version": "../package.json",
  "identifier": "com.carabo.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "Carabo",
        "width": 1440,
        "height": 900,
        "minWidth": 1280,
        "minHeight": 720,
        "resizable": true,
        "dragDropEnabled": true
      }
    ],
    "security": {
      "csp": "default-src 'self'; connect-src 'self' ipc: http://ipc.localhost; img-src 'self' data: blob:; style-src 'self' 'unsafe-inline'; font-src 'self'",
      "devCsp": "default-src 'self' 'unsafe-inline' 'unsafe-eval' ipc: http://ipc.localhost ws://localhost:1420 data: blob:"
    }
  },
  "bundle": { "active": false, "icon": ["icons/icon.ico"] }
}
```

- `identifier` decide las carpetas de datos de la aplicación (05 §7).
- `bundle.active` es `false`: no se genera instalador (01 §7). La orden de compilación produce solo el ejecutable.
- `csp` es la política de la aplicación compilada: no permite cargar nada remoto (RNF-18). `devCsp` se usa solo en desarrollo; si el servidor de desarrollo necesita algo más, se ajusta solo `devCsp`.

### 5.7 `src-tauri/capabilities/default.json`

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Permisos de la ventana principal",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:allow-set-title",
    "core:window:allow-destroy",
    "dialog:allow-open",
    "dialog:allow-save",
    "opener:allow-reveal-item-in-dir"
  ]
}
```

No se añade ningún permiso más. En particular, ninguno de red, de ejecución de programas ni de acceso libre a archivos (RF-009, RNF-18).

### 5.8 `.gitignore` y `.gitattributes`

`.gitignore`: `node_modules/`, `dist/`, `target/`, `release/`, `test-results/`, `playwright-report/`, `src-tauri/gen/`, `*.log`.

`.gitattributes`:

```text
* text=auto eol=lf
*.wav binary
*.peaks binary
*.png binary
*.ico binary
```

Todos los archivos de texto se guardan con fin de línea LF, también en Windows, para que los datos de prueba sean idénticos en todas partes.

## 6. Cómo se usa Tauri

### 6.1 Arranque del programa nativo

`src-tauri/src/main.rs` contiene solo la llamada `carabo_lib::run()` y, en su primera línea, `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`, para que el programa compilado no abra una ventana de consola. `src-tauri/build.rs` contiene solo `tauri_build::build()`.

`src-tauri/src/lib.rs` define `pub fn run()`, que:

1. registra los plugins `tauri_plugin_dialog` y `tauri_plugin_opener`;
2. en `setup`: calcula las rutas (`paths.rs`), abre el registro (`logging.rs`), arranca el motor con `Engine::start(EngineConfig { kits_dir })`, guarda el `AppState` con `app.manage(…)` y lanza el hilo de telemetría (`telemetry.rs`);
3. registra **todos** los comandos de 04 §6.2 con `tauri::generate_handler!`;
4. ejecuta la aplicación con `tauri::generate_context!()`.

Si `Engine::start` falla, el error se escribe en el registro y el programa termina con un cuadro nativo de error con el texto `Carabo could not start its audio engine. See the log file.` y la ruta del registro. Es el único texto visible que no pasa por los archivos de traducción, porque la interfaz aún no existe.

**Carpeta de kits** (`kits_dir`), en este orden:

1. la carpeta `kits` que esté junto al ejecutable, si existe;
2. si no, y solo en compilaciones de desarrollo (`cfg!(debug_assertions)`), `assets/kits` del repositorio, cuya ruta se fija al compilar a partir de `env!("CARGO_MANIFEST_DIR")`.

### 6.2 Comandos

- Cada comando de 04 §6.2 es una función `#[tauri::command]` **`async`** en el módulo de `src-tauri/src/commands/` que indica 04 §4. Se declaran `async` para que no se ejecuten en el hilo principal.
- Un comando que puede bloquear más de 50 ms (importar, exportar, calibrar, copiar o borrar carpetas, leer picos) hace su trabajo dentro de `tauri::async_runtime::spawn_blocking`.
- Los argumentos llegan desde TypeScript en `camelCase` y Tauri los entrega a los parámetros de Rust en `snake_case`: `invoke("project_write", { dir, json })`, `invoke("audio_import", { srcPath, projectDir })`.
- Todo tipo que cruza el puente deriva `serde::Serialize` o `Deserialize` con `#[serde(rename_all = "camelCase")]`.
- Todo comando devuelve `Result<T, BridgeError>`. `BridgeError` (`error.rs`) deriva `Serialize` y tiene los campos `code` (los textos exactos de 04 §6.3) y `detail`. Hay una conversión de `std::io::Error` y otra de `EngineError`; un error de E/S cuyo tipo es «no encontrado» se convierte en `NOT_FOUND`, y cualquier otro, en `IO`.
- `peaks_get` devuelve los bytes con `tauri::ipc::Response`, de modo que la interfaz recibe un `ArrayBuffer`.
- El estado compartido (`AppState`, 04 §4) se recibe con `tauri::State<'_, AppState>`.

### 6.3 Eventos

- El programa nativo emite con `app_handle.emit("<nombre>", contenido)` (rasgo `tauri::Emitter`). Los nombres son exactamente los de 04 §6.4, por ejemplo `engine://telemetry`.
- `telemetry.rs` lanza un hilo llamado `carabo-telemetry` que, cada `1000 / TELEMETRY_HZ` milisegundos, llama a `EngineHandle::telemetry()` y lo emite, y después emite un evento por cada elemento de `poll_events()`.
- `tauriBridge.ts` se suscribe con `listen` de `@tauri-apps/api/event`. Como `listen` devuelve una promesa y los métodos `on…` del puente devuelven la función de baja al momento, `tauriBridge` guarda la promesa y, al darse de baja, espera a que se resuelva y llama a la función que entrega.
- Los eventos de progreso (`import://progress`, `export://progress`) se emiten como mucho 10 veces por segundo y siempre una última vez con `percent: 100`.

### 6.4 Funciones de Tauri que usa `tauriBridge.ts` sin comando propio

| Método del puente | Función de Tauri |
|---|---|
| `pickProjectFile` | `open` de `@tauri-apps/plugin-dialog`, con `multiple: false` y el filtro `{ name: "Carabo", extensions: ["carabo"] }` |
| `pickFolder` | `open` con `directory: true` y `defaultPath: startDir` |
| `pickAudioFile` | `open` con `multiple: false` y el filtro `{ name: "Audio", extensions: ["wav", "mp3"] }` |
| `pickExportPath` | `save` de `@tauri-apps/plugin-dialog`, con `defaultPath` igual a la carpeta de `defaultProjectsDir()` más `defaultName` y la extensión del formato, y un filtro con esa extensión |
| `revealInExplorer` | `revealItemInDir` de `@tauri-apps/plugin-opener` |
| `setWindowTitle` | `getCurrentWindow().setTitle(title)` de `@tauri-apps/api/window` |
| `onCloseRequested` | `getCurrentWindow().onCloseRequested(…)`; el manejador llama siempre a `event.preventDefault()` y después al `callback` |
| `closeWindow` | `getCurrentWindow().destroy()`. No se usa `close()`, que volvería a disparar la petición de cierre |
| `onFileDrop` | `getCurrentWebview().onDragDropEvent(…)` de `@tauri-apps/api/webview`. Solo se atiende el suceso de tipo `drop`: sus `paths` y su `position`, que llega en píxeles físicos y se divide por `await getCurrentWindow().scaleFactor()` para entregarla en píxeles lógicos (04 §6.4) |

**Arrastres dentro de la ventana.** Con `dragDropEnabled` activado, que hace falta para recibir archivos del Explorador, la API de arrastrar y soltar de HTML5 no funciona dentro de la ventana en Windows. Todo arrastre de la interfaz (clips, bloques, bordes, deslizadores, tramo de bucle) se implementa con eventos de puntero: `pointerdown`, `pointermove`, `pointerup` y `setPointerCapture`. NO DEBEN usarse el atributo `draggable` ni los eventos `dragstart`, `dragover` o `drop`.

### 6.5 Qué puente se usa

`src/bridge/index.ts` exporta `tauriBridge` si `"__TAURI_INTERNALS__" in window`, y `mockBridge` en caso contrario. Así, `npm run dev` abierto en un navegador funciona con el puente simulado, y las pruebas también (08). `index.ts` carga `tauriBridge.ts` con una importación dinámica, para que el navegador no intente cargar los paquetes de Tauri.

### 6.6 Rutas

`paths.rs` obtiene todas las carpetas de Tauri, nunca de variables de entorno:

| Carpeta | Función | Resultado en Windows |
|---|---|---|
| Configuración | `app.path().app_config_dir()` | `%APPDATA%\com.carabo.app\` |
| Datos locales | `app.path().app_local_data_dir()` | `%LOCALAPPDATA%\com.carabo.app\` |
| Documentos | `app.path().document_dir()` | `Documentos` del usuario; `default_projects_dir` le añade `Carabo` |

## 7. Dónde vive lo que comparten varias partes

| Qué | Dónde | Regla |
|---|---|---|
| Constantes de 00 §4 | `src/core/constants.ts` y `engine/src/constants.rs`, con los mismos nombres | Una prueba de cada lado compara sus valores con `fixtures/constants.json` (08 §7). Ningún otro archivo repite un valor de 00 §4 |
| Dirección del repositorio | `REPOSITORY_URL` en `src/core/constants.ts` | Es la dirección HTTPS del remoto `origin` de Git al crear el repositorio. Si no hay remoto, es la cadena vacía y «Acerca de» no muestra esa línea (10) |
| Fórmulas de tiempo de 05 §2 | `src/core/time.ts` y `engine/src/time.rs` | Una prueba de contrato comprueba que dan lo mismo (08 §7) |
| Esquema del proyecto | `src/core/schema/project-1.schema.json` | Es, literalmente, el esquema de 05 §5.2. `core/validate.ts` lo importa |
| Tema visual | `src/ui/theme.css` | Define las variables CSS de 03 §2 en `:root`. Ningún otro archivo contiene un color, un tamaño de letra ni una medida de 03 §2 escritos a mano |
| Textos | `src/i18n/es.json` y `src/i18n/en.json` | Objetos planos: cada clave de 03 es una clave del objeto, con sus puntos (`"transport.play.label": "Reproducir"`). `i18n/index.ts` configura i18next con `keySeparator: false` y `nsSeparator: false` |
| Kits y ritmos hechos | `assets/kits/` y `assets/presets/presets.json` | La interfaz importa los `kit.json` y `presets.json` al compilar (`core/kits.ts`, `core/presets.ts`). El motor lee los kits en tiempo de ejecución (§6.1) |
| Datos de prueba | `fixtures/` | 08 §3 |
| Huecos resueltos | `docs/DECISIONES-DE-IMPLEMENTACION.md` | Formato de 00 §6 |

## 8. Convenciones de código

### 8.1 Generales

- **Idioma** (DEC-35). Código, identificadores, comentarios, mensajes de commit y textos de las pruebas, en inglés. Especificación y documentos de `docs/`, en español. Los textos que ve el usuario están solo en `src/i18n/`.
- **Nombres.** Los del glosario (00 §7, columna «En código») y los de 04, 05 y 06. No se inventa un sinónimo para algo que ya tiene nombre.
- **Comentarios.** Explican el porqué, no el qué. Donde el código implementa una regla de la especificación, se cita: `// RF-309: the covering element wins`.
- **Pendientes.** No se deja código a medias. Un `TODO` solo se admite con la tarea que lo resolverá: `// TODO(T-123): …`.
- **Tamaño.** Un archivo de más de 400 líneas o una función de más de 60 son señal de que hay que partir.
- **Nada fuera de la especificación.** No se añaden opciones, parámetros de configuración ni código «por si acaso».

### 8.2 TypeScript y React

- Solo exportaciones con nombre (`export function`, `export const`). Las exportaciones por defecto se usan solo donde una herramienta las exige (archivos de configuración).
- Sin `any`, sin `as` salvo al leer JSON ya validado, y sin `!` fuera de las pruebas.
- Componentes como funciones con *hooks*. Un componente por archivo, con el mismo nombre que el archivo.
- Un componente lee los almacenes con selectores (`useProjectStore((s) => s.doc.tempoBpm)`) y nunca guarda en su estado local una copia de datos del proyecto.
- Los componentes no contienen lógica del proyecto: llaman a funciones de `core/`, a acciones de `projectStore` o a un servicio (04 §3.5).
- `core/` es puro: mismas entradas, mismas salidas. Sin `Date`, sin `Math.random`, sin acceso al DOM (04 §9).
- Las funciones de `core/commands/` no lanzan excepciones por situaciones previstas: devuelven el motivo (04 §10).
- Toda promesa se espera o se trata; nada de promesas sueltas.
- Los identificadores nuevos se crean solo con `core/ids.ts`.
- Los números se muestran con `Intl.NumberFormat` del idioma activo (RNF-17).

### 8.3 Estilos

- CSS normal, en un archivo por componente, importado desde el componente.
- Las clases siguen el patrón `NombreDelComponente-parte` y `is-estado`: `.TransportBar-button`, `.is-active`.
- Todo color, tamaño de letra, radio y espaciado sale de una variable de `theme.css`.
- Las posiciones y anchuras que dependen de los datos (bloques, clips, cursor) se ponen con el atributo `style`.
- Las ondas y los medidores se dibujan en `<canvas>` (03).

### 8.4 Rust

- Edición 2021. `cargo fmt` y `cargo clippy -- -D warnings` sin avisos.
- Sin `unsafe` (§5.5). Las pruebas de integración pueden usarlo donde 08 lo indica.
- Los errores se devuelven con `Result`. `unwrap` y `expect` solo en pruebas y en valores que no pueden fallar por construcción, con un comentario que lo justifique; en `rt/` y `dsp/` están prohibidos (§5.5).
- Sin `println!` ni `eprintln!` en las bibliotecas. El motor no escribe registro: comunica eventos (06 §18.2). El programa nativo escribe con `logging.rs`.
- Los hilos tienen nombre: `carabo-control`, `carabo-loader`, `carabo-writer`, `carabo-telemetry`.
- La API pública del motor lleva comentarios de documentación (`///`).
- Las pruebas unitarias van en un módulo `#[cfg(test)] mod tests` al final de su archivo; las de integración, en `engine/tests/` y `src-tauri/tests/`.

## 9. Órdenes

Todas se ejecutan en la raíz del repositorio.

| Para | Orden |
|---|---|
| Instalar dependencias | `npm ci` |
| Interfaz sola, en un navegador, con el puente simulado | `npm run dev` y abrir `http://localhost:1420` |
| Aplicación completa en desarrollo | `npm run tauri dev` |
| Compilar la interfaz | `npm run build` |
| Compilar el ejecutable | `npm run tauri build` (deja `target/release/carabo.exe`; ese archivo solo, sin la carpeta `kits` a su lado, no arranca: §6.1) |
| Preparar la carpeta que se distribuye | `npm run package` (§13.4) |
| Análisis estático y formato | `npm run lint`, `npm run typecheck`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` |
| Pruebas de la interfaz | `npm test` |
| Pruebas de extremo a extremo | `npm run test:e2e` |
| Pruebas de Rust | `cargo test --workspace` |
| Pruebas de rendimiento del motor | `cargo test -p carabo-engine --release -- --ignored perf_` |
| Pruebas del motor con un dispositivo de audio real (solo en un equipo real) | `cargo test -p carabo-engine -- --ignored device_` |
| Calcular una instantánea sin dispositivo | `cargo run -p carabo-engine --bin render -- <instantánea.json> <salida.wav>` |
| Regenerar los sonidos sustitutos | `npm run kits` |
| Comprobar licencias | `npm run licenses` |
| Informe de qué criterios de aceptación tienen prueba | `npm run traceability` (08 §13.3) |

`cargo test --workspace` y `cargo clippy` compilan también `src-tauri`, que necesita que exista la carpeta `dist/`: hay que haber ejecutado antes `npm run build` al menos una vez.

**Comprobación completa** (lo que una tarea debe dejar pasando, y lo que ejecuta la integración continua), en este orden: `npm run lint`, `npm run typecheck`, `npm test`, `npm run test:e2e`, `npm run build`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm run licenses`.

## 10. Git y GitHub

### 10.1 Ramas y commits

- La rama principal es `main` y siempre pasa la comprobación completa.
- Cada tarea del plan (09) se hace en una rama `t-nnn-descripcion-corta` creada desde `main`, por ejemplo `t-014-step-grid`.
- Mensajes de commit: `T-nnn: <resumen en inglés, en imperativo>`. Un commit puede ser parcial; la rama entera cumple la tarea.
- Cada tarea termina en una solicitud de integración (*pull request*) hacia `main` cuyo texto dice: tarea, qué se ha hecho, cómo se ha comprobado cada criterio de aceptación y qué entradas se han añadido a `docs/DECISIONES-DE-IMPLEMENTACION.md`.
- El autor del proyecto revisa y fusiona. Un agente no fusiona su propia solicitud ni reescribe la historia de `main`.

### 10.2 Qué se guarda en Git

Todo lo necesario para compilar y probar sin generar nada antes: código, especificación, `package-lock.json`, `Cargo.lock`, los WAV de `assets/kits/`, `assets/icon.png`, `src-tauri/icons/` y `fixtures/`. No se guardan las carpetas generadas (§4) ni proyectos de usuario.

### 10.3 Integración continua

`.github/workflows/ci.yml`:

```yaml
name: CI
on:
  push:
    branches: [main]
  pull_request:

jobs:
  check:
    runs-on: windows-latest
    timeout-minutes: 60
    steps:
      - uses: actions/checkout@v5
      - uses: actions/setup-node@v4
        with:
          node-version-file: .nvmrc
          cache: npm
      - name: Install Rust toolchain from rust-toolchain.toml
        run: rustup toolchain install
      - uses: Swatinem/rust-cache@v2
      - run: npm ci
      - run: npm run lint
      - run: npm run typecheck
      - run: npm test
      - run: npx playwright install chromium
      - run: npm run test:e2e
      - run: npm run build
      - run: cargo fmt --all --check
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - run: cargo test --workspace
      - run: cargo test -p carabo-engine --release -- --ignored perf_
      - run: npm run licenses
      - run: npm run traceability
```

El ejecutor de GitHub no tiene tarjeta de sonido: ninguna prueba automática puede depender de un dispositivo de audio (08 §2).

## 11. Licencias

### 11.1 Licencia del proyecto

MIT (DEC-25). El archivo `LICENSE` contiene el texto de la licencia con el año de creación del repositorio y el nombre que figure en `git config user.name`.

### 11.2 Licencias admitidas en dependencias que se distribuyen

Se distribuye lo que acaba dentro del ejecutable: las dependencias de producción de `package.json` y las dependencias normales y de compilación del paquete `carabo`, con todas las suyas.

| Admitidas |
|---|
| MIT, MIT-0, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, 0BSD, BSL-1.0, Unicode-3.0, Unicode-DFS-2016, CC0-1.0, Unlicense |
| MPL-2.0, siempre que la dependencia se use sin modificar su código |

Las herramientas de desarrollo (compiladores, bibliotecas de pruebas, analizadores) no se distribuyen y no están sujetas a esta lista.

### 11.3 El codificador de MP3

- LAME tiene licencia LGPL. Se admite como **única excepción**, para los paquetes `mp3lame-encoder` y `mp3lame-sys`, porque todo el código de Carabo es público y cualquiera puede recompilarlo con otra versión de LAME. El aviso correspondiente figura en `THIRD-PARTY-NOTICES.md`, que se distribuye junto al ejecutable (§13.4).
- El codificador queda aislado detrás de la interfaz de `engine/src/files/mp3.rs` (06 §16.3).
- **Si `mp3lame-encoder` no compila** en el entorno de desarrollo o en la integración continua: se sustituye, solo dentro de `files/mp3.rs`, por el crate `shine-rs` (codificador en Rust puro, también LGPL), se añade ese nombre a las excepciones del script de licencias y se anota en `docs/DECISIONES-DE-IMPLEMENTACION.md`. El resto del programa no cambia. Si tampoco `shine-rs` sirve, se aplica 00 §6.

### 11.4 `THIRD-PARTY-NOTICES.md`

Lo crea la tarea que añade el codificador de MP3 y contiene: una frase que indica que Carabo incluye software de terceros; el aviso de LAME (nombre, dirección del proyecto, licencia LGPL y la indicación de que el código fuente de Carabo es público y permite recompilar el programa con otra versión de LAME), seguido del texto de esa licencia tal como figura en el archivo `COPYING` del código de LAME que trae el crate; y una frase que remite a `Cargo.lock` y `package-lock.json` para la lista completa de dependencias.

### 11.5 Sonidos

Los sonidos de los kits incluidos son los sustitutos generados por el propio repositorio (05 §8.4), sin derechos de terceros. Los definitivos deberán tener licencia CC0 (DEC-33, 10).

### 11.6 Comprobación automática

`npm run licenses` ejecuta `tools/check-licenses.mjs` (§13.3) y falla si alguna dependencia distribuida tiene una licencia no admitida. Forma parte de la comprobación completa (RNF-19).

## 12. Claude Code

Los agentes que programan Carabo son sesiones de Claude Code (DEC-24). Claude Code carga al empezar cada sesión el archivo `CLAUDE.md` de la raíz del repositorio, y carga el `CLAUDE.md` de una subcarpeta cuando trabaja con archivos de esa subcarpeta. Estos dos archivos son breves a propósito: la especificación no se copia en ellos, se señala.

### 12.1 `CLAUDE.md` de la raíz

Contenido exacto:

```markdown
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
```

### 12.2 `engine/CLAUDE.md`

Contenido exacto:

```markdown
# Motor de audio (`carabo-engine`)

Lee `docs/spec/06-motor-de-audio.md` §1 a §7 antes de tocar esta carpeta.

- En `src/rt/` y `src/dsp/`, y en las funciones de `src/io/` que llama `cpal`, rigen las reglas
  RT-01 a RT-10 (06 §4): sin reservar ni liberar memoria, sin cerrojos, sin archivos, sin registro
  y sin pánicos.
- Las fórmulas de 06 son exactas. Las pruebas comparan muestra a muestra.
- El motor no depende de Tauri ni escribe registro: comunica eventos (06 §18.2).
- Comprobar: `cargo test -p carabo-engine` y `cargo test -p carabo-engine --release -- --ignored perf_`.
```

### 12.3 Lo que no se crea

No se crean subagentes, *skills*, *hooks* ni un archivo `.claude/settings.json` en el repositorio. Si el autor quiere ajustar permisos de Claude Code, lo hace en su configuración personal, fuera del repositorio.

## 13. Scripts de `tools/`

Son módulos de Node (`.mjs`) sin dependencias externas. Cada uno escribe en la salida estándar lo que hace y termina con código distinto de 0 si falla.

### 13.1 `gen-placeholder-kits.mjs`

Genera los 24 WAV de `assets/kits/` con las recetas de 05 §8.4, en 16 bits, mono, 48 000 Hz. Es determinista: dos ejecuciones producen archivos idénticos byte a byte. Los archivos generados se guardan en Git. No toca los `kit.json`, que se escriben a mano según 05 §8.2 y §8.3.

### 13.2 `gen-max-load-project.mjs`

`node tools/gen-max-load-project.mjs <carpeta de destino> [--minutes N]` crea una carpeta de proyecto con el proyecto de carga máxima de 08 §9. Por defecto `N` es 10; ocupa entonces unos 1,4 GB.

### 13.3 `check-licenses.mjs`

1. Obtiene las dependencias de Rust que se distribuyen con `cargo tree -p carabo -e normal,build --prefix none --format "{p}|{l}"` y elimina las líneas repetidas.
2. Obtiene las de la interfaz con `npm ls --omit=dev --all --parseable` y lee el campo `license` del `package.json` de cada carpeta.
3. Evalúa cada expresión de licencia: `A OR B` (o `A/B`) se admite si se admite alguna; `A AND B`, si se admiten todas; los paréntesis se respetan; `WITH <excepción>` se ignora.
4. Admite las licencias de §11.2 y, por nombre de paquete, las excepciones de §11.3. Los paquetes propios (`carabo`, `carabo-engine`) se omiten.
5. Escribe la lista de paquetes rechazados y termina con código 1 si hay alguno. Un paquete sin licencia declarada se rechaza.

### 13.4 `package.mjs`

1. Ejecuta `npm run tauri build`.
2. Crea `release/Carabo-<versión>-win64/` (borrándola antes si existe) con: `Carabo.exe` (copia de `target/release/carabo.exe`), la carpeta `kits/` (copia de `assets/kits/`), `LICENSE` y `THIRD-PARTY-NOTICES.md`.

Esa carpeta es lo que se distribuye (01 §7): se copia entera al equipo de destino y se ejecuta `Carabo.exe`. No hay instalador.

### 13.5 `gen-icon.mjs`

Escribe `assets/icon.png`: 1024 × 1024 píxeles, fondo `#1B1D23`, y dos círculos de color `#F2B544` y radio 180 con centro en (332, 512) y (692, 512), cada uno con un círculo interior de color `#1B1D23` y radio 80 con el mismo centro. Es un icono provisional (10). Los archivos de `src-tauri/icons/` se generan una vez con `npm run tauri icon assets/icon.png` y se guardan en Git.

### 13.6 `check-traceability.mjs`

Comprueba que cada criterio de aceptación de un requisito [1.0] tiene una prueba. Su funcionamiento exacto y su opción `--strict` están en 08 §13.3. Sin `--strict` solo informa y termina con código 0.

### 13.7 `measure-offset.mjs`

Herramienta de las pruebas de aceptación manuales PA-02 y PA-05 (08 §12). Lee archivos WAV de 24 bits exportados por Carabo.

- `node tools/measure-offset.mjs <referencia.wav> <toma.wav>`: mezcla cada archivo a mono; localiza en la referencia el principio de cada golpe (primer frame cuyo valor absoluto supera 0,05 tras al menos 4 800 frames por debajo de 0,01); para cada golpe calcula, por correlación cruzada de los 2 400 frames siguientes con la toma, el desplazamiento entre −960 y +960 frames que da el máximo; y escribe, en milisegundos, el desfase medio de los golpes de los 5 primeros segundos, de los 5 centrales y de los 5 últimos, y el mayor desfase en valor absoluto. Un valor positivo significa que la toma va retrasada.
- `node tools/measure-offset.mjs --tone <archivo.wav>`: escribe la frecuencia del tono dominante del archivo, en Hz con un decimal, contando los cruces por cero ascendentes de su segundo central.
