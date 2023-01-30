import { readFile } from 'fs/promises'
import { join } from 'path'
import { fileURLToPath } from 'url'

import test from 'ava'

import { convertTtfToWoff2 } from '../index.js'

const fixture = await readFile(join(fileURLToPath(import.meta.url), '..', './iconsfont.ttf'))

test('should be able to convert ttf to woff2', async (t) => {
  t.notThrows(() => convertTtfToWoff2(fixture))
})
