# Carabo

Carabo es un DAW (estación de trabajo de audio digital) de escritorio para Windows 11 cuya virtud
principal es resultar intuitivo. Sirve para montar una base rítmica con patrones de pasos, grabar
audio encima con un micrófono, mezclar y exportar la canción a un archivo de audio.

La interfaz está escrita en TypeScript con React y se ejecuta dentro de Tauri 2; el motor de audio
es una biblioteca en Rust.

## Especificación

Toda la especificación está en [`docs/spec/`](docs/spec/). Empieza por
[`docs/spec/00-indice.md`](docs/spec/00-indice.md).

## Cómo se compila

Requisitos (Windows 11 de 64 bits): Node.js 24, Git, `rustup` con Rust estable y las «Build Tools
for Visual Studio» con la carga «Desarrollo para el escritorio con C++».

En PowerShell, en la raíz del repositorio:

| Para                                                   | Orden                                         |
| ------------------------------------------------------ | --------------------------------------------- |
| Instalar dependencias                                  | `npm ci`                                      |
| Interfaz sola, en el navegador, con el puente simulado | `npm run dev` y abrir `http://localhost:1420` |
| Aplicación completa en desarrollo                      | `npm run tauri dev`                           |
| Compilar la interfaz                                   | `npm run build`                               |
| Compilar el ejecutable                                 | `npm run tauri build`                         |
| Análisis estático                                      | `npm run lint` y `npm run typecheck`          |
| Pruebas de la interfaz                                 | `npm test`                                    |

La lista completa de órdenes está en `docs/spec/07-tecnologia-y-repositorio.md` §9.

## Licencia

MIT. Consulta [`LICENSE`](LICENSE).
