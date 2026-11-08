import { readFile } from 'node:fs/promises'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

import test from 'ava'
import * as fontkit from '@yisibl/fontkit'

import { convertTTFToWOFF2, convertTTFToWOFF2Async, convertWOFF2ToTTF, convertWOFF2ToTTFAsync } from '../index.js'

const dirname = join(fileURLToPath(import.meta.url), '..')

test('should be able to convert ttf to woff2', async (t) => {
  const fixture = await readFile(join(dirname, './iconsfont.ttf'))
  const woff2Buffer = Buffer.from(convertTTFToWOFF2(fixture))
  t.is(fontkit.openSync(woff2Buffer).type, 'WOFF2')
  t.is(fontkit.openSync(woff2Buffer).directory.tag, 'wOF2')
  t.is(fontkit.openSync(woff2Buffer).stream.length, woff2Buffer.length)
})

test('should be able to convert ttf to woff2 async', async (t) => {
  const fixture = await readFile(join(dirname, './iconsfont.ttf'))
  const woff2Buffer = await convertTTFToWOFF2Async(fixture)

  t.is(fontkit.openSync(woff2Buffer).type, 'WOFF2')
  t.is(fontkit.openSync(woff2Buffer).directory.tag, 'wOF2')
  t.is(fontkit.openSync(woff2Buffer).stream.length, woff2Buffer.length)
})

test('should be able to convert ttf to woff2 async with params', async (t) => {
  const fixture = await readFile(join(dirname, './iconsfont.ttf'))
  const woff2Buffer = await convertTTFToWOFF2Async(fixture, {
    brotliQuality: 7,
  })

  t.is(fontkit.openSync(woff2Buffer).type, 'WOFF2')
  t.is(fontkit.openSync(woff2Buffer).directory.tag, 'wOF2')
  t.is(fontkit.openSync(woff2Buffer).stream.length, woff2Buffer.length)
})

test('convert woff2 to ttf', async (t) => {
  const fontawesome = await readFile(join(dirname, './fa-brands-400-v6.2.woff2'))
  const ttfBuffer = convertWOFF2ToTTF(fontawesome)

  t.is(fontkit.openSync(ttfBuffer).type, 'TTF')
  t.is(fontkit.openSync(ttfBuffer).directory.tag, '\x00\x01\x00\x00')
  t.is(fontkit.openSync(ttfBuffer).stream.length, ttfBuffer.length)
})

test('convert woff2 back to ttf', async (t) => {
  const fixture = await readFile(join(dirname, './iconsfont.ttf'))
  const woff2Buffer = convertTTFToWOFF2(fixture)
  const ttfBuffer = convertWOFF2ToTTF(woff2Buffer)

  t.is(fontkit.openSync(ttfBuffer).type, 'TTF')
  t.is(fontkit.openSync(ttfBuffer).directory.tag, '\x00\x01\x00\x00')
  t.is(fontkit.openSync(ttfBuffer).stream.length, ttfBuffer.length)
})

test('convert woff2 back to ttf async', async (t) => {
  const fixture = await readFile(join(dirname, './iconsfont.ttf'))
  const woff2Buffer = await convertTTFToWOFF2Async(fixture)
  const ttfBuffer = await convertWOFF2ToTTFAsync(woff2Buffer)

  t.is(fontkit.openSync(ttfBuffer).type, 'TTF')
  t.is(fontkit.openSync(ttfBuffer).directory.tag, '\x00\x01\x00\x00')
  t.is(fontkit.openSync(ttfBuffer).stream.length, ttfBuffer.length)
})

// Structurally valid single-table WOFF2 with an incomplete Brotli stream.
// This checks the JS rejection paths, not the block-switch CVE regression in rust-brotli/src/lib.rs.
function malformedBrotliWoff2() {
  const font = Buffer.alloc(51)
  font.write('wOF2')
  font.writeUInt32BE(0x00010000, 4)
  font.writeUInt32BE(font.length, 8)
  font.writeUInt16BE(1, 12)
  font.writeUInt32BE(32, 16)
  font.writeUInt32BE(1, 20)
  font[48] = 0 // cmap, untransformed
  font[49] = 4 // original table length
  font[50] = 0 // incomplete Brotli stream
  return font
}

test('rejects malformed Brotli in WOFF2', (t) => {
  t.throws(() => convertWOFF2ToTTF(malformedBrotliWoff2()), { message: 'ConvertWOFF2ToTTF failed' })
})

test('rejects malformed Brotli in WOFF2 asynchronously', async (t) => {
  await t.throwsAsync(() => convertWOFF2ToTTFAsync(malformedBrotliWoff2()), { message: 'ConvertWOFF2ToTTF failed' })
})
