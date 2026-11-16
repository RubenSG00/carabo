// Writes the provisional application icon, assets/icon.png (07 §13.5).
// Usage: node tools/gen-icon.mjs
// The icons in src-tauri/icons/ are then generated once with `npm run tauri icon assets/icon.png`.

import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { deflateSync } from "node:zlib";

const SIZE = 1024;
const BACKGROUND = [0x1b, 0x1d, 0x23];
const RING = [0xf2, 0xb5, 0x44];
const OUTER_RADIUS = 180;
const INNER_RADIUS = 80;
const CENTERS = [
  [332, 512],
  [692, 512],
];

/** Colour of the pixel whose centre is at (x + 0.5, y + 0.5). */
function pixelColor(x, y) {
  for (const [cx, cy] of CENTERS) {
    const dx = x + 0.5 - cx;
    const dy = y + 0.5 - cy;
    const d2 = dx * dx + dy * dy;
    if (d2 <= INNER_RADIUS * INNER_RADIUS) return BACKGROUND;
    if (d2 <= OUTER_RADIUS * OUTER_RADIUS) return RING;
  }
  return BACKGROUND;
}

const CRC_TABLE = Array.from({ length: 256 }, (_, n) => {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c >>> 0;
});

function crc32(bytes) {
  let c = 0xffffffff;
  for (const b of bytes) c = CRC_TABLE[(c ^ b) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const length = Buffer.alloc(4);
  length.writeUInt32BE(data.length);
  const typeAndData = Buffer.concat([Buffer.from(type, "ascii"), data]);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(typeAndData));
  return Buffer.concat([length, typeAndData, crc]);
}

/** Encodes the icon as an 8-bit RGB PNG. */
function renderIcon() {
  const rowLength = 1 + SIZE * 3;
  const raw = Buffer.alloc(rowLength * SIZE);
  for (let y = 0; y < SIZE; y++) {
    raw[y * rowLength] = 0; // filter type: none
    for (let x = 0; x < SIZE; x++) {
      const [r, g, b] = pixelColor(x, y);
      const offset = y * rowLength + 1 + x * 3;
      raw[offset] = r;
      raw[offset + 1] = g;
      raw[offset + 2] = b;
    }
  }
  const header = Buffer.alloc(13);
  header.writeUInt32BE(SIZE, 0);
  header.writeUInt32BE(SIZE, 4);
  header[8] = 8; // bit depth
  header[9] = 2; // colour type: RGB
  header[10] = 0; // compression
  header[11] = 0; // filter
  header[12] = 0; // no interlace
  return Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    chunk("IHDR", header),
    chunk("IDAT", deflateSync(raw, { level: 9 })),
    chunk("IEND", Buffer.alloc(0)),
  ]);
}

const target = fileURLToPath(new URL("../assets/icon.png", import.meta.url));
try {
  writeFileSync(target, renderIcon());
  console.log(`Wrote ${target} (${SIZE} × ${SIZE}).`);
} catch (error) {
  console.error(`Could not write ${target}: ${error.message}`);
  process.exit(1);
}
