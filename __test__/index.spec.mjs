import { readFile } from 'fs/promises'
import { join } from 'path'
import { fileURLToPath } from 'url'

import test from 'ava'

import wawoff from 'wawoff2'
import { convertTTFToWOFF2 } from '../index.js'

const fixture = await readFile(join(fileURLToPath(import.meta.url), '..', './iconsfont.ttf'))

test('should be able to convert ttf to woff2', async (t) => {
  t.notThrows(() => convertTTFToWOFF2(fixture))
})

test('should be same length as the wawoff2', async (t) => {
  const wawoffResult = await wawoff.compress(fixture)
  t.is(convertTTFToWOFF2(fixture).length, wawoffResult.length)
})
