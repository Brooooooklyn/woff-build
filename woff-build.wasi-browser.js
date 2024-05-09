import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
  createOnMessage as __wasmCreateOnMessageForFsProxy,
} from '@napi-rs/wasm-runtime'

import __wasmUrl from './woff-build.wasm32-wasi.wasm?url'

const __wasi = new __WASI({
  version: 'preview1',
})

const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })
    
    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    __napi_rs_initialize_modules(instance)
  },
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__convert_ttf_to_woff2_0']?.()
  __napiInstance.exports['__napi_register__ConvertTTFToWOFF2Task_impl_1']?.()
  __napiInstance.exports['__napi_register__Woff2Params_struct_2']?.()
  __napiInstance.exports['__napi_register__convert_ttf_to_woff2_async_3']?.()
  __napiInstance.exports['__napi_register__ConvertWOFF2ToTTFTask_impl_4']?.()
  __napiInstance.exports['__napi_register__convert_woff2_to_ttf_5']?.()
  __napiInstance.exports['__napi_register__convert_woff2_to_ttf_async_6']?.()
}
export const convertTTFToWOFF2 = __napiModule.exports.convertTTFToWOFF2
export const convertTTFToWOFF2Async = __napiModule.exports.convertTTFToWOFF2Async
export const convertWOFF2ToTTF = __napiModule.exports.convertWOFF2ToTTF
export const convertWOFF2ToTTFAsync = __napiModule.exports.convertWOFF2ToTTFAsync
