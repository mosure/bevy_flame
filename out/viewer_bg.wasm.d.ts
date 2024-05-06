/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function strftime_l(a: number, b: number, c: number, d: number, e: number): number;
export function _tzset_js(a: number, b: number, c: number, d: number): void;
export function _mktime_js(a: number): void;
export function _localtime_js(a: number, b: number): void;
export function _gmtime_js(a: number, b: number): void;
export function __libc_malloc(a: number): number;
export function __libc_calloc(a: number, b: number): number;
export function __libc_free(a: number): void;
export function posix_memalign(a: number, b: number, c: number): number;
export function realloc(a: number, b: number): number;
export function abort(): void;
export function __wasi_environ_sizes_get(a: number, b: number): number;
export function __wasi_environ_get(a: number, b: number): number;
export function __wasi_fd_seek(a: number, b: number, c: number, d: number): number;
export function __wasi_fd_read(a: number, b: number, c: number, d: number): number;
export function __wasi_fd_close(a: number): number;
export function emscripten_memcpy_js(a: number, b: number, c: number): void;
export function emscripten_get_heap_max(): number;
export function emscripten_date_now(): number;
export function _emscripten_get_now_is_monotonic(): number;
export function emscripten_builtin_malloc(a: number): number;
export function __cxa_throw(a: number, b: number, c: number): void;
export function _initialize(): void;
export function emscripten_errn(a: number, b: number): void;
export function malloc(a: number): number;
export function free(a: number): void;
export function __wasi_fd_write(a: number, b: number, c: number, d: number): number;
export function emscripten_get_now(): number;
export function main(a: number, b: number): number;
export function wgpu_compute_pass_set_push_constant(a: number, b: number, c: number, d: number): void;
export function wgpu_render_bundle_set_push_constants(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_push_constants(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_pipeline(a: number, b: number): void;
export function wgpu_render_bundle_set_vertex_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_index_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_draw(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_draw_indexed(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_bundle_draw_indirect(a: number, b: number, c: number): void;
export function wgpu_render_bundle_draw_indexed_indirect(a: number, b: number, c: number): void;
export function wgpu_render_bundle_pop_debug_group(a: number): void;
export function wgpu_render_bundle_insert_debug_marker(a: number, b: number): void;
export function wgpu_compute_pass_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_compute_pass_set_pipeline(a: number, b: number): void;
export function wgpu_compute_pass_dispatch_workgroups(a: number, b: number, c: number, d: number): void;
export function wgpu_compute_pass_dispatch_workgroups_indirect(a: number, b: number, c: number): void;
export function wgpu_compute_pass_push_debug_group(a: number, b: number, c: number): void;
export function wgpu_compute_pass_pop_debug_group(a: number): void;
export function wgpu_compute_pass_insert_debug_marker(a: number, b: number, c: number): void;
export function wgpu_compute_pass_write_timestamp(a: number, b: number, c: number): void;
export function wgpu_compute_pass_begin_pipeline_statistics_query(a: number, b: number, c: number): void;
export function wgpu_compute_pass_end_pipeline_statistics_query(a: number): void;
export function wgpu_render_pass_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_pipeline(a: number, b: number): void;
export function wgpu_render_pass_set_vertex_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_index_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_blend_constant(a: number, b: number): void;
export function wgpu_render_pass_set_stencil_reference(a: number, b: number): void;
export function wgpu_render_pass_set_viewport(a: number, b: number, c: number, d: number, e: number, f: number, g: number): void;
export function wgpu_render_pass_set_scissor_rect(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_draw(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_draw_indexed(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_draw_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_draw_indexed_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_multi_draw_indirect(a: number, b: number, c: number, d: number): void;
export function wgpu_render_pass_multi_draw_indexed_indirect(a: number, b: number, c: number, d: number): void;
export function wgpu_render_pass_multi_draw_indirect_count(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_multi_draw_indexed_indirect_count(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_push_debug_group(a: number, b: number, c: number): void;
export function wgpu_render_pass_pop_debug_group(a: number): void;
export function wgpu_render_pass_insert_debug_marker(a: number, b: number, c: number): void;
export function wgpu_render_pass_write_timestamp(a: number, b: number, c: number): void;
export function wgpu_render_pass_begin_occlusion_query(a: number, b: number): void;
export function wgpu_render_pass_end_occlusion_query(a: number): void;
export function wgpu_render_pass_begin_pipeline_statistics_query(a: number, b: number, c: number): void;
export function wgpu_render_pass_end_pipeline_statistics_query(a: number): void;
export function wgpu_render_pass_execute_bundles(a: number, b: number, c: number): void;
export function wgpu_render_bundle_push_debug_group(a: number, b: number): void;
export function __wbindgen_malloc(a: number, b: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number, d: number): number;
export const __wbindgen_export_2: WebAssembly.Table;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf7363e2d903c9908(a: number, b: number, c: number): void;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h660996f99ba3f051(a: number, b: number, c: number): void;
export function _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hbf895ccf5070c4a2(a: number, b: number): void;
export function _dyn_core__ops__function__FnMut__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0a3613c701f86f15(a: number, b: number, c: number, d: number): void;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he5d2ab9f711a8460(a: number, b: number, c: number): void;
export function _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h197900021fddcc12(a: number, b: number, c: number): void;
export function _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h37262886e36c7196(a: number, b: number): void;
export function __wbindgen_free(a: number, b: number, c: number): void;
export function __wbindgen_exn_store(a: number): void;
export function __wbindgen_start(): void;