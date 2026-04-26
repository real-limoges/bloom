/* tslint:disable */
/* eslint-disable */

export class BloomEngine {
    free(): void;
    [Symbol.dispose](): void;
    adjacency_neighbors(): Uint32Array;
    adjacency_offsets(): Uint32Array;
    focus_node(node_id: number): void;
    hover(screen_x: number, screen_y: number): number | undefined;
    init_renderer(canvas: HTMLCanvasElement): Promise<void>;
    load_graph(data: Uint8Array): void;
    constructor(canvas: HTMLCanvasElement);
    node_ids(): Uint32Array;
    node_labels(): string[];
    node_pageranks(): Float32Array;
    node_screen_positions(): Float32Array;
    resize(width: number, height: number): void;
    tick(dt: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_bloomengine_free: (a: number, b: number) => void;
    readonly bloomengine_adjacency_neighbors: (a: number, b: number) => void;
    readonly bloomengine_adjacency_offsets: (a: number, b: number) => void;
    readonly bloomengine_focus_node: (a: number, b: number) => void;
    readonly bloomengine_hover: (a: number, b: number, c: number) => number;
    readonly bloomengine_init_renderer: (a: number, b: number) => number;
    readonly bloomengine_load_graph: (a: number, b: number, c: number, d: number) => void;
    readonly bloomengine_new: (a: number, b: number) => void;
    readonly bloomengine_node_ids: (a: number, b: number) => void;
    readonly bloomengine_node_labels: (a: number, b: number) => void;
    readonly bloomengine_node_pageranks: (a: number, b: number) => void;
    readonly bloomengine_node_screen_positions: (a: number, b: number) => void;
    readonly bloomengine_resize: (a: number, b: number, c: number) => void;
    readonly bloomengine_tick: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_1829: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_2521: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_1845: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
