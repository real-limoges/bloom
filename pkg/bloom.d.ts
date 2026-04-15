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
