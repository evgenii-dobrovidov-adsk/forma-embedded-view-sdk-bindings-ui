// Manual glue for the example's `start()` export.
// The wasm-bindgen generated init handles all @forma/ui-lib FFI bindings;
// we just call the raw WASM export afterwards.
import init from './pkg/forma_ui_lib_wasm_example.js';

const wasm = await init();
wasm.start();
