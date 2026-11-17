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
