/* tslint:disable */
/* eslint-disable */

export class LogIndexer {
    free(): void;
    [Symbol.dispose](): void;
    count_by_level(level: number): number;
    get_line(line_index: number): string | undefined;
    get_line_level(line_index: number): number;
    get_line_timestamp(line_index: number): number;
    constructor(data: Uint8Array);
    total_errors(): number;
    total_lines(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_logindexer_free: (a: number, b: number) => void;
    readonly logindexer_count_by_level: (a: number, b: number) => number;
    readonly logindexer_get_line: (a: number, b: number) => [number, number];
    readonly logindexer_get_line_level: (a: number, b: number) => number;
    readonly logindexer_get_line_timestamp: (a: number, b: number) => number;
    readonly logindexer_new: (a: number, b: number) => number;
    readonly logindexer_total_errors: (a: number) => number;
    readonly logindexer_total_lines: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_start: () => void;
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
