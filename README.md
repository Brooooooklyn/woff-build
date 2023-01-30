# `@napi-rs/ttf2woff2`

![CI](https://github.com/Brooooooklyn/ttf2woff2/workflows/CI/badge.svg)
[![install size](https://packagephobia.com/badge?p=@napi-rs/ttf2woff2)](https://packagephobia.com/result?p=@napi-rs/ttf2woff2)
[![Downloads](https://img.shields.io/npm/dm/@napi-rs/ttf2woff2.svg?sanitize=true)](https://npmcharts.com/compare/@napi-rs/ttf2woff2?minimal=true)

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

## Usage

```js
import { join } from 'path'
import { fileURLToPath } from 'url'

import { convertTTFToWOFF2 } from '@napi-rs/ttf2woff2'

const fixture = await readFile(join(fileURLToPath(import.meta.url), '..', './iconsfont.ttf'))

convertTTFToWOFF2(fixture)
```