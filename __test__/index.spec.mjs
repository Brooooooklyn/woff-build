import { readFile } from 'node:fs/promises'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

import test from 'ava'
import * as fontkit from '@yisibl/fontkit'

import { convertTTFToWOFF2, convertTTFToWOFF2Async, convertWOFF2ToTTF, convertWOFF2ToTTFAsync } from '../index.js'

const fixture = await readFile(join(fileURLToPath(import.meta.url), '..', './iconsfont.ttf'))
const fontawesome = await readFile(join(fileURLToPath(import.meta.url), '..', './fa-brands-400-v6.2.woff2'))

test('should be able to convert ttf to woff2', (t) => {
  const woff2Buffer = Buffer.from(convertTTFToWOFF2(fixture))
  t.is(fontkit.openSync(woff2Buffer).type, 'WOFF2')
  t.is(fontkit.openSync(woff2Buffer).directory.tag, 'wOF2')
  t.is(fontkit.openSync(woff2Buffer).stream.length, woff2Buffer.length)
})

test('should be able to convert ttf to woff2 async', async (t) => {
   const woff2Buffer = await convertTTFToWOFF2Async(fixture)

  t.is(fontkit.openSync(woff2Buffer).type, 'WOFF2')
  t.is(fontkit.openSync(woff2Buffer).directory.tag, 'wOF2')
  t.is(fontkit.openSync(woff2Buffer).stream.length, woff2Buffer.length)
})

test('should be able to convert ttf to woff2 async with params', async (t) => {
  const woff2Buffer = await convertTTFToWOFF2Async(fixture, {
    brotliQuality: 7,
  })

 t.is(fontkit.openSync(woff2Buffer).type, 'WOFF2')
 t.is(fontkit.openSync(woff2Buffer).directory.tag, 'wOF2')
 t.is(fontkit.openSync(woff2Buffer).stream.length, woff2Buffer.length)
})

test('convert woff2 to ttf', (t) => {
  const ttfBuffer = convertWOFF2ToTTF(fontawesome)

  t.is(fontkit.openSync(ttfBuffer).type, 'TTF')
  t.is(fontkit.openSync(ttfBuffer).directory.tag, '\x00\x01\x00\x00')
  t.is(fontkit.openSync(ttfBuffer).stream.length, ttfBuffer.length)
})

test('convert woff2 back to ttf', (t) => {
  const woff2Buffer = convertTTFToWOFF2(fixture)
  const ttfBuffer = convertWOFF2ToTTF(woff2Buffer)

  t.is(fontkit.openSync(ttfBuffer).type, 'TTF')
  t.is(fontkit.openSync(ttfBuffer).directory.tag, '\x00\x01\x00\x00')
  t.is(fontkit.openSync(ttfBuffer).stream.length, ttfBuffer.length)
})

test('convert woff2 back to ttf async', async (t) => {
  const woff2Buffer = await convertTTFToWOFF2Async(fixture)
  const ttfBuffer = await convertWOFF2ToTTFAsync(woff2Buffer)

  t.is(fontkit.openSync(ttfBuffer).type, 'TTF')
  t.is(fontkit.openSync(ttfBuffer).directory.tag, '\x00\x01\x00\x00')
  t.is(fontkit.openSync(ttfBuffer).stream.length, ttfBuffer.length)
})
