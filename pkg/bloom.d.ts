/* tslint:disable */
/* eslint-disable */

export class BloomEngine {
    free(): void;
    [Symbol.dispose](): void;
    focus_node(node_id: number): void;
    hover(screen_x: number, screen_y: number): number | undefined;
    init_renderer(canvas: HTMLCanvasElement): Promise<void>;
    load_graph(data: Uint8Array): void;
    constructor(canvas: HTMLCanvasElement);
    resize(width: number, height: number): void;
    tick(dt: number): void;
}
