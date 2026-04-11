/* @ts-self-types="./bloom.d.ts" */

import * as wasm from "./bloom_bg.wasm";
import { __wbg_set_wasm } from "./bloom_bg.js";
__wbg_set_wasm(wasm);

export {
    BloomEngine
} from "./bloom_bg.js";
