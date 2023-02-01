import { promises as fs } from 'fs'
import path, { join } from 'path'
import { fileURLToPath } from 'url'
import { performance } from 'perf_hooks'

import { convertTTFToWOFF2 } from '../index.js'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.resolve(path.dirname(__filename))

async function main() {
  const ttfFont = await fs.readFile(join(__dirname, '/RobotoFlex-VF.ttf'))

  const t = performance.now()
  const woff2Font = convertTTFToWOFF2(ttfFont)
  console.info('✨ Done in', performance.now() - t, 'ms')
  console.info('TTF font length  : ', ttfFont.length)
  console.info('WOFF2 font length: ', woff2Font.length)

  await fs.writeFile(join(__filename, '..', './RobotoFlex-VF.woff2'), woff2Font)
}

main()
