# `@napi-rs/woff-build`

![CI](https://github.com/Brooooooklyn/@napi-rs/woff-build/workflows/CI/badge.svg)
[![install size](https://packagephobia.com/badge?p=@napi-rs/woff-build)](https://packagephobia.com/result?p=@napi-rs/woff-build)
[![Downloads](https://img.shields.io/npm/dm/@napi-rs/woff-build.svg?sanitize=true)](https://npmcharts.com/compare/@napi-rs/woff-build?minimal=true)

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

## Usage

```js
import { join } from 'path'
import { fileURLToPath } from 'url'

import { convertTTFToWOFF2 } from '@napi-rs/woff-build'

const fixture = await readFile(join(fileURLToPath(import.meta.url), '..', './iconsfont.ttf'))

convertTTFToWOFF2(fixture)
```

## [Example](example/index.mjs)

```shell
node example/index.mjs

✨ Done in 2460.2297090291977 ms
TTF font length  :  1654412
WOFF2 font length:  729564
```

Then open the [example HTML](example/index.html) in your browser, you can test the converted WOFF2 fonts.
