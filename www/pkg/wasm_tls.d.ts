/* tslint:disable */
/* eslint-disable */
export function create_client(remote_host: string): void;
export function send_data(data: Uint8Array): void;
export function get_data(): Uint8Array;
export function process_tls(): void;
export function write_request(request: string): void;
export function read_response(): string;
export function create_server(): void;
export function server_send_data(data: Uint8Array): void;
export function server_get_data(): Uint8Array;
export function server_process_tls(): void;
export function server_read_request(): string;
export function server_write_response(response: string): void;
export function main_js(): void;
export function greet(name: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly create_client: (a: number, b: number) => void;
  readonly send_data: (a: number, b: number) => [number, number];
  readonly get_data: () => [number, number, number, number];
  readonly process_tls: () => [number, number];
  readonly write_request: (a: number, b: number) => [number, number];
  readonly read_response: () => [number, number, number, number];
  readonly ring_core_0_17_14__bn_mul_mont: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly create_server: () => [number, number];
  readonly server_send_data: (a: number, b: number) => [number, number];
  readonly server_get_data: () => [number, number, number, number];
  readonly server_process_tls: () => [number, number];
  readonly server_read_request: () => [number, number, number, number];
  readonly server_write_response: (a: number, b: number) => [number, number];
  readonly main_js: () => void;
  readonly greet: (a: number, b: number) => [number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
