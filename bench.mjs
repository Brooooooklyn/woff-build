import { promises as fs } from 'fs'
import path, { join } from 'path'
import { fileURLToPath } from 'url'

import b from 'benny'

import { convertTTFToWOFF2, convertTTFToWOFF2Async } from './index.js'
import ttf2woff2 from 'ttf2woff2'
import woff2 from 'woff2'
import woff2Next from 'woff2-next'
import wawoff from 'wawoff2'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.resolve(path.dirname(__filename))

async function run() {
  // TTF
  const font1 = await fs.readFile(join(__dirname, '/example/fa-brands-400-v6.2.ttf'))

  // OTF
  const font2 = await fs.readFile(join(__dirname, '/example/SourceSans3-Regular.otf'))

  await b.suite(
    'TTF to WOFF2 (Use Font Awesome)',

    b.add('ttf2woff2(Rust)', () => {
      convertTTFToWOFF2(font1)
    }),

    b.add('ttf2woff2 Async(Rust)', async () => {
      await convertTTFToWOFF2Async(font1)
    }),

    b.add('woff2(node-gyp)', () => {
      woff2.encode(font1)
    }),

    b.add('woff2-next(node-gyp)', () => {
      woff2Next.encode(font1)
    }),

    b.add('ttf2woff2(node-gyp)', () => {
      ttf2woff2(font1)
    }),

    b.add('wawoff2(Wasm)', async () => {
      await wawoff.compress(font1)
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'TTF to WOFF2 (SourceSans3-Regular)',

    b.add('ttf2woff2(Rust)', () => {
      convertTTFToWOFF2(font2)
    }),

    b.add('ttf2woff2 Async(Rust)', async () => {
      await convertTTFToWOFF2Async(font2)
    }),

    b.add('woff2(node-gyp)', () => {
      woff2.encode(font2)
    }),

    b.add('woff2-next(node-gyp)', () => {
      woff2Next.encode(font2)
    }),

    b.add('ttf2woff2(node-gyp)', () => {
      ttf2woff2(font2)
    }),

    b.add('wawoff2(Wasm)', async () => {
      await wawoff.compress(font2)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
