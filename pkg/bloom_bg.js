export class BloomEngine {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BloomEngineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bloomengine_free(ptr, 0);
    }
    /**
     * @returns {Uint32Array}
     */
    adjacency_neighbors() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bloomengine_adjacency_neighbors(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {Uint32Array}
     */
    adjacency_offsets() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bloomengine_adjacency_offsets(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} node_id
     */
    focus_node(node_id) {
        wasm.bloomengine_focus_node(this.__wbg_ptr, node_id);
    }
    /**
     * @param {number} screen_x
     * @param {number} screen_y
     * @returns {number | undefined}
     */
    hover(screen_x, screen_y) {
        const ret = wasm.bloomengine_hover(this.__wbg_ptr, screen_x, screen_y);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {HTMLCanvasElement} canvas
     * @returns {Promise<void>}
     */
    init_renderer(canvas) {
        const ret = wasm.bloomengine_init_renderer(this.__wbg_ptr, addHeapObject(canvas));
        return takeObject(ret);
    }
    /**
     * @param {Uint8Array} data
     */
    load_graph(data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.bloomengine_load_graph(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {HTMLCanvasElement} canvas
     */
    constructor(canvas) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bloomengine_new(retptr, addHeapObject(canvas));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            BloomEngineFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {Uint32Array}
     */
    node_ids() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bloomengine_node_ids(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string[]}
     */
    node_labels() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bloomengine_node_labels(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {Float32Array}
     */
    node_pageranks() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bloomengine_node_pageranks(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {Float32Array}
     */
    node_screen_positions() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.bloomengine_node_screen_positions(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} width
     * @param {number} height
     */
    resize(width, height) {
        wasm.bloomengine_resize(this.__wbg_ptr, width, height);
    }
    /**
     * @param {number} dt
     */
    tick(dt) {
        wasm.bloomengine_tick(this.__wbg_ptr, dt);
    }
}
if (Symbol.dispose) BloomEngine.prototype[Symbol.dispose] = BloomEngine.prototype.free;
export function __wbg_Window_7b2011a6368164ef(arg0) {
    const ret = getObject(arg0).Window;
    return addHeapObject(ret);
}
export function __wbg_WorkerGlobalScope_4bddbcb12b3f5a28(arg0) {
    const ret = getObject(arg0).WorkerGlobalScope;
    return addHeapObject(ret);
}
export function __wbg___wbindgen_debug_string_0bc8482c6e3508ae(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_is_function_0095a73b8b156f76(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
}
export function __wbg___wbindgen_is_null_ac34f5003991759a(arg0) {
    const ret = getObject(arg0) === null;
    return ret;
}
export function __wbg___wbindgen_is_undefined_9e4d92534c42d778(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
}
export function __wbg___wbindgen_throw_be289d5034ed271b(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg__wbg_cb_unref_d9b87ff7982e3b21(arg0) {
    getObject(arg0)._wbg_cb_unref();
}
export function __wbg_beginRenderPass_599b98d9a6ba5692() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).beginRenderPass(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_buffer_26d0910f3a5bc899(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
}
export function __wbg_call_389efe28435a9388() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_call_4708e0c13bdc8e95() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_configure_bee5e0250d8526d5() { return handleError(function (arg0, arg1) {
    getObject(arg0).configure(getObject(arg1));
}, arguments); }
export function __wbg_createBindGroupLayout_f543b79f894eed2e() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).createBindGroupLayout(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_createBindGroup_06db01d96df151a7(arg0, arg1) {
    const ret = getObject(arg0).createBindGroup(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_createBuffer_6e69283608e8f98f() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).createBuffer(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_createCommandEncoder_88e8ef64b19cdb2c(arg0, arg1) {
    const ret = getObject(arg0).createCommandEncoder(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_createPipelineLayout_0f960a922b66be56(arg0, arg1) {
    const ret = getObject(arg0).createPipelineLayout(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_createRenderPipeline_725209221f17f288() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).createRenderPipeline(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_createSampler_36aca895fb724d8b(arg0, arg1) {
    const ret = getObject(arg0).createSampler(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_createShaderModule_714b17aece65828e(arg0, arg1) {
    const ret = getObject(arg0).createShaderModule(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_createTexture_63195fd0d63c3a24() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).createTexture(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_createView_79f49fbd3fb5f94f() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).createView(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_debug_46a93995fc6f8820(arg0, arg1, arg2, arg3) {
    console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}
export function __wbg_document_ee35a3d3ae34ef6c(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_drawIndexed_c47b56e3bafadecb(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).drawIndexed(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
}
export function __wbg_draw_3f782f0d09a907da(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).draw(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_end_8bb194afb9988691(arg0) {
    getObject(arg0).end();
}
export function __wbg_error_794d0ffc9d00d5c3(arg0, arg1, arg2, arg3) {
    console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}
export function __wbg_error_9a7fe3f932034cde(arg0) {
    console.error(getObject(arg0));
}
export function __wbg_finish_08e2d7b08c066b25(arg0, arg1) {
    const ret = getObject(arg0).finish(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_finish_5ebfba3167b3092c(arg0) {
    const ret = getObject(arg0).finish();
    return addHeapObject(ret);
}
export function __wbg_getContext_2966500392030d63() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_getContext_2a5764d48600bc43() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_getCurrentTexture_6dc4d0ea8555e374() { return handleError(function (arg0) {
    const ret = getObject(arg0).getCurrentTexture();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_getMappedRange_3cb6354f7963e27e() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).getMappedRange(arg1, arg2);
    return addHeapObject(ret);
}, arguments); }
export function __wbg_getPreferredCanvasFormat_06854455b835cf40(arg0) {
    const ret = getObject(arg0).getPreferredCanvasFormat();
    return (__wbindgen_enum_GpuTextureFormat.indexOf(ret) + 1 || 96) - 1;
}
export function __wbg_get_d8db2ad31d529ff8(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_gpu_653e59c6ae8028a8(arg0) {
    const ret = getObject(arg0).gpu;
    return addHeapObject(ret);
}
export function __wbg_height_38750dc6de41ee75(arg0) {
    const ret = getObject(arg0).height;
    return ret;
}
export function __wbg_info_9e602cf10c5c690b(arg0, arg1, arg2, arg3) {
    console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}
export function __wbg_instanceof_GpuAdapter_b2c1300e425af95c(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof GPUAdapter;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_GpuCanvasContext_c9b75b4b7dc7555e(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof GPUCanvasContext;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_Window_ed49b2db8df90359(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_label_f279af9fe090b53f(arg0, arg1) {
    const ret = getObject(arg1).label;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_length_32ed9a279acd054c(arg0) {
    const ret = getObject(arg0).length;
    return ret;
}
export function __wbg_log_24aba2a6d8990b35(arg0, arg1, arg2, arg3) {
    console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}
export function __wbg_mapAsync_e89ffbd0722e6025(arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).mapAsync(arg1 >>> 0, arg2, arg3);
    return addHeapObject(ret);
}
export function __wbg_navigator_43be698ba96fc088(arg0) {
    const ret = getObject(arg0).navigator;
    return addHeapObject(ret);
}
export function __wbg_navigator_4478931f32ebca57(arg0) {
    const ret = getObject(arg0).navigator;
    return addHeapObject(ret);
}
export function __wbg_new_361308b2356cecd0() {
    const ret = new Object();
    return addHeapObject(ret);
}
export function __wbg_new_3eb36ae241fe6f44() {
    const ret = new Array();
    return addHeapObject(ret);
}
export function __wbg_new_b5d9e2fb389fef91(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wasm_bindgen_func_elem_2522(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        const ret = new Promise(cb0);
        return addHeapObject(ret);
    } finally {
        state0.a = state0.b = 0;
    }
}
export function __wbg_new_from_slice_a3d2629dc1826784(arg0, arg1) {
    const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
    return addHeapObject(ret);
}
export function __wbg_new_no_args_1c7c842f08d00ebb(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}
export function __wbg_new_with_byte_offset_and_length_aa261d9c9da49eb1(arg0, arg1, arg2) {
    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
}
export function __wbg_onSubmittedWorkDone_babe5ab237e856ff(arg0) {
    const ret = getObject(arg0).onSubmittedWorkDone();
    return addHeapObject(ret);
}
export function __wbg_prototypesetcall_bdcdcc5842e4d77d(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
}
export function __wbg_push_8ffdcb2063340ba5(arg0, arg1) {
    const ret = getObject(arg0).push(getObject(arg1));
    return ret;
}
export function __wbg_querySelectorAll_1283aae52043a951() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).querySelectorAll(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_queueMicrotask_0aa0a927f78f5d98(arg0) {
    const ret = getObject(arg0).queueMicrotask;
    return addHeapObject(ret);
}
export function __wbg_queueMicrotask_5bb536982f78a56f(arg0) {
    queueMicrotask(getObject(arg0));
}
export function __wbg_queue_13a5c48e3c54a28c(arg0) {
    const ret = getObject(arg0).queue;
    return addHeapObject(ret);
}
export function __wbg_requestAdapter_cc9a9924f72519ab(arg0, arg1) {
    const ret = getObject(arg0).requestAdapter(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_requestDevice_295504649d1da14c(arg0, arg1) {
    const ret = getObject(arg0).requestDevice(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_resolve_002c4b7d9d8f6b64(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
}
export function __wbg_setBindGroup_bf7233e51ee0fd56(arg0, arg1, arg2) {
    getObject(arg0).setBindGroup(arg1 >>> 0, getObject(arg2));
}
export function __wbg_setBindGroup_c532d9e80c3b863a() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).setBindGroup(arg1 >>> 0, getObject(arg2), getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
}, arguments); }
export function __wbg_setIndexBuffer_d5812b7c5ff15c50(arg0, arg1, arg2, arg3) {
    getObject(arg0).setIndexBuffer(getObject(arg1), __wbindgen_enum_GpuIndexFormat[arg2], arg3);
}
export function __wbg_setIndexBuffer_d6851b016152211a(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setIndexBuffer(getObject(arg1), __wbindgen_enum_GpuIndexFormat[arg2], arg3, arg4);
}
export function __wbg_setPipeline_b632e313f54b1cb1(arg0, arg1) {
    getObject(arg0).setPipeline(getObject(arg1));
}
export function __wbg_setVertexBuffer_71f6b6b9f7c32e99(arg0, arg1, arg2, arg3) {
    getObject(arg0).setVertexBuffer(arg1 >>> 0, getObject(arg2), arg3);
}
export function __wbg_setVertexBuffer_c8234139ead62a61(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setVertexBuffer(arg1 >>> 0, getObject(arg2), arg3, arg4);
}
export function __wbg_set_25cf9deff6bf0ea8(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
}
export function __wbg_set_6cb8631f80447a67() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
}, arguments); }
export function __wbg_set_a_e87a2053d5fccb4c(arg0, arg1) {
    getObject(arg0).a = arg1;
}
export function __wbg_set_access_69d91e9d4e4ceac2(arg0, arg1) {
    getObject(arg0).access = __wbindgen_enum_GpuStorageTextureAccess[arg1];
}
export function __wbg_set_address_mode_u_17e91ba6701d7cdf(arg0, arg1) {
    getObject(arg0).addressModeU = __wbindgen_enum_GpuAddressMode[arg1];
}
export function __wbg_set_address_mode_v_83cff33885b49fd0(arg0, arg1) {
    getObject(arg0).addressModeV = __wbindgen_enum_GpuAddressMode[arg1];
}
export function __wbg_set_address_mode_w_2445963d0feae757(arg0, arg1) {
    getObject(arg0).addressModeW = __wbindgen_enum_GpuAddressMode[arg1];
}
export function __wbg_set_alpha_a7a68e5ec04efe77(arg0, arg1) {
    getObject(arg0).alpha = getObject(arg1);
}
export function __wbg_set_alpha_mode_60f87267fa3d95d0(arg0, arg1) {
    getObject(arg0).alphaMode = __wbindgen_enum_GpuCanvasAlphaMode[arg1];
}
export function __wbg_set_alpha_to_coverage_enabled_67782b8fff854d06(arg0, arg1) {
    getObject(arg0).alphaToCoverageEnabled = arg1 !== 0;
}
export function __wbg_set_array_layer_count_2bd74e56899b603a(arg0, arg1) {
    getObject(arg0).arrayLayerCount = arg1 >>> 0;
}
export function __wbg_set_array_stride_acb85bd3848529a6(arg0, arg1) {
    getObject(arg0).arrayStride = arg1;
}
export function __wbg_set_aspect_82ca9caa27a4c533(arg0, arg1) {
    getObject(arg0).aspect = __wbindgen_enum_GpuTextureAspect[arg1];
}
export function __wbg_set_aspect_b78bd0b34ebfe19b(arg0, arg1) {
    getObject(arg0).aspect = __wbindgen_enum_GpuTextureAspect[arg1];
}
export function __wbg_set_attributes_4d5de6c80e3a7e73(arg0, arg1) {
    getObject(arg0).attributes = getObject(arg1);
}
export function __wbg_set_b_87725d82ac69a631(arg0, arg1) {
    getObject(arg0).b = arg1;
}
export function __wbg_set_base_array_layer_064977086530f2e7(arg0, arg1) {
    getObject(arg0).baseArrayLayer = arg1 >>> 0;
}
export function __wbg_set_base_mip_level_845abe28a57bd901(arg0, arg1) {
    getObject(arg0).baseMipLevel = arg1 >>> 0;
}
export function __wbg_set_beginning_of_pass_write_index_18bb7ab9fb16de02(arg0, arg1) {
    getObject(arg0).beginningOfPassWriteIndex = arg1 >>> 0;
}
export function __wbg_set_bind_group_layouts_db65f9787380e242(arg0, arg1) {
    getObject(arg0).bindGroupLayouts = getObject(arg1);
}
export function __wbg_set_binding_35fa28beda49ff83(arg0, arg1) {
    getObject(arg0).binding = arg1 >>> 0;
}
export function __wbg_set_binding_3b4abee15b11f6ec(arg0, arg1) {
    getObject(arg0).binding = arg1 >>> 0;
}
export function __wbg_set_blend_21337ec514ad2280(arg0, arg1) {
    getObject(arg0).blend = getObject(arg1);
}
export function __wbg_set_buffer_a9223dfcc0e34853(arg0, arg1) {
    getObject(arg0).buffer = getObject(arg1);
}
export function __wbg_set_buffer_d49e95bb5349d827(arg0, arg1) {
    getObject(arg0).buffer = getObject(arg1);
}
export function __wbg_set_buffers_68609a5d48c31b27(arg0, arg1) {
    getObject(arg0).buffers = getObject(arg1);
}
export function __wbg_set_bytes_per_row_4a52bbf4cdbfe78b(arg0, arg1) {
    getObject(arg0).bytesPerRow = arg1 >>> 0;
}
export function __wbg_set_clear_value_8fc3623594df71b2(arg0, arg1) {
    getObject(arg0).clearValue = getObject(arg1);
}
export function __wbg_set_code_20093e29960281f8(arg0, arg1, arg2) {
    getObject(arg0).code = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_color_64a633bf7b4cf6fe(arg0, arg1) {
    getObject(arg0).color = getObject(arg1);
}
export function __wbg_set_color_attachments_4d4c71d7eeba8e2f(arg0, arg1) {
    getObject(arg0).colorAttachments = getObject(arg1);
}
export function __wbg_set_compare_0376672b0c0bbfd8(arg0, arg1) {
    getObject(arg0).compare = __wbindgen_enum_GpuCompareFunction[arg1];
}
export function __wbg_set_compare_f3fb77a9bf3f0f7e(arg0, arg1) {
    getObject(arg0).compare = __wbindgen_enum_GpuCompareFunction[arg1];
}
export function __wbg_set_count_8cf9a3dd1ffc7b7d(arg0, arg1) {
    getObject(arg0).count = arg1 >>> 0;
}
export function __wbg_set_cull_mode_41c12526410d3e05(arg0, arg1) {
    getObject(arg0).cullMode = __wbindgen_enum_GpuCullMode[arg1];
}
export function __wbg_set_depth_bias_31554aeaaa675954(arg0, arg1) {
    getObject(arg0).depthBias = arg1;
}
export function __wbg_set_depth_bias_clamp_8cf5f4f0d80e8cba(arg0, arg1) {
    getObject(arg0).depthBiasClamp = arg1;
}
export function __wbg_set_depth_bias_slope_scale_310ae406f2d3a055(arg0, arg1) {
    getObject(arg0).depthBiasSlopeScale = arg1;
}
export function __wbg_set_depth_clear_value_8760aafb583d5312(arg0, arg1) {
    getObject(arg0).depthClearValue = arg1;
}
export function __wbg_set_depth_compare_8831904ce3173063(arg0, arg1) {
    getObject(arg0).depthCompare = __wbindgen_enum_GpuCompareFunction[arg1];
}
export function __wbg_set_depth_fail_op_62ec602580477afc(arg0, arg1) {
    getObject(arg0).depthFailOp = __wbindgen_enum_GpuStencilOperation[arg1];
}
export function __wbg_set_depth_load_op_102d57f3ddf95461(arg0, arg1) {
    getObject(arg0).depthLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
}
export function __wbg_set_depth_or_array_layers_d7b93db07c5da69d(arg0, arg1) {
    getObject(arg0).depthOrArrayLayers = arg1 >>> 0;
}
export function __wbg_set_depth_read_only_aebc24a542debafd(arg0, arg1) {
    getObject(arg0).depthReadOnly = arg1 !== 0;
}
export function __wbg_set_depth_stencil_5627e73aaf33912c(arg0, arg1) {
    getObject(arg0).depthStencil = getObject(arg1);
}
export function __wbg_set_depth_stencil_attachment_04b936535778e362(arg0, arg1) {
    getObject(arg0).depthStencilAttachment = getObject(arg1);
}
export function __wbg_set_depth_store_op_610b0a50dbb00eb8(arg0, arg1) {
    getObject(arg0).depthStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
}
export function __wbg_set_depth_write_enabled_f94217df9ff2d60c(arg0, arg1) {
    getObject(arg0).depthWriteEnabled = arg1 !== 0;
}
export function __wbg_set_device_dab18ead7bfc077b(arg0, arg1) {
    getObject(arg0).device = getObject(arg1);
}
export function __wbg_set_dimension_2a75a794a0bfcc94(arg0, arg1) {
    getObject(arg0).dimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
}
export function __wbg_set_dimension_a3c50fb6d43f6cec(arg0, arg1) {
    getObject(arg0).dimension = __wbindgen_enum_GpuTextureDimension[arg1];
}
export function __wbg_set_dst_factor_cf872fec841747ac(arg0, arg1) {
    getObject(arg0).dstFactor = __wbindgen_enum_GpuBlendFactor[arg1];
}
export function __wbg_set_end_of_pass_write_index_02ee5189026c1d3a(arg0, arg1) {
    getObject(arg0).endOfPassWriteIndex = arg1 >>> 0;
}
export function __wbg_set_entries_1472deaee7053fb7(arg0, arg1) {
    getObject(arg0).entries = getObject(arg1);
}
export function __wbg_set_entries_b2258b5ef29810b0(arg0, arg1) {
    getObject(arg0).entries = getObject(arg1);
}
export function __wbg_set_entry_point_11f912102ade99b1(arg0, arg1, arg2) {
    getObject(arg0).entryPoint = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_entry_point_f9224cdb29cbe5df(arg0, arg1, arg2) {
    getObject(arg0).entryPoint = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_external_texture_613e4434100d63ee(arg0, arg1) {
    getObject(arg0).externalTexture = getObject(arg1);
}
export function __wbg_set_fail_op_73a4e194f4bc914a(arg0, arg1) {
    getObject(arg0).failOp = __wbindgen_enum_GpuStencilOperation[arg1];
}
export function __wbg_set_format_1670e760e18ac001(arg0, arg1) {
    getObject(arg0).format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_2141a8a1fd36fb9c(arg0, arg1) {
    getObject(arg0).format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_25e4aacc74949e38(arg0, arg1) {
    getObject(arg0).format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_3f7008e9e568f0fc(arg0, arg1) {
    getObject(arg0).format = __wbindgen_enum_GpuVertexFormat[arg1];
}
export function __wbg_set_format_4a4fccdfc45bc409(arg0, arg1) {
    getObject(arg0).format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_7696f8290da8a36b(arg0, arg1) {
    getObject(arg0).format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_974a01725f579c5d(arg0, arg1) {
    getObject(arg0).format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_fragment_f7ce64feaf1cd7dc(arg0, arg1) {
    getObject(arg0).fragment = getObject(arg1);
}
export function __wbg_set_front_face_09e32557f8852301(arg0, arg1) {
    getObject(arg0).frontFace = __wbindgen_enum_GpuFrontFace[arg1];
}
export function __wbg_set_g_c31c959457596456(arg0, arg1) {
    getObject(arg0).g = arg1;
}
export function __wbg_set_has_dynamic_offset_fbc1bb343939ed0b(arg0, arg1) {
    getObject(arg0).hasDynamicOffset = arg1 !== 0;
}
export function __wbg_set_height_710b87344b3d6748(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
}
export function __wbg_set_height_b386c0f603610637(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
}
export function __wbg_set_height_f21f985387070100(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
}
export function __wbg_set_label_0ec13ba975f77124(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_3b658d9ce970552c(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_48883f5f49e4ec47(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_4bbbc289ddddebd7(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_4d609666f09cfdfb(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_4f4264b0041180e2(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_5b46e419b9e88c5e(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_95423cd2e1f4b5dd(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_c3fc0a66f4ecc82b(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_c857f45a8485236a(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_dc8df9969898889c(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_e3709fe3e82429b5(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_fb5d28b3ba7af11f(arg0, arg1, arg2) {
    getObject(arg0).label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_layout_7f76289be3294b4a(arg0, arg1) {
    getObject(arg0).layout = getObject(arg1);
}
export function __wbg_set_layout_c20d48b352b24c1b(arg0, arg1) {
    getObject(arg0).layout = getObject(arg1);
}
export function __wbg_set_load_op_c71d200e998908b0(arg0, arg1) {
    getObject(arg0).loadOp = __wbindgen_enum_GpuLoadOp[arg1];
}
export function __wbg_set_lod_max_clamp_aaac5daaecca96d4(arg0, arg1) {
    getObject(arg0).lodMaxClamp = arg1;
}
export function __wbg_set_lod_min_clamp_ed2162d4b198abba(arg0, arg1) {
    getObject(arg0).lodMinClamp = arg1;
}
export function __wbg_set_mag_filter_c8a8c1218cd38da6(arg0, arg1) {
    getObject(arg0).magFilter = __wbindgen_enum_GpuFilterMode[arg1];
}
export function __wbg_set_mapped_at_creation_2d003ce549611385(arg0, arg1) {
    getObject(arg0).mappedAtCreation = arg1 !== 0;
}
export function __wbg_set_mask_a933ba2e61c7610a(arg0, arg1) {
    getObject(arg0).mask = arg1 >>> 0;
}
export function __wbg_set_max_anisotropy_fb4bae64cb5acf57(arg0, arg1) {
    getObject(arg0).maxAnisotropy = arg1;
}
export function __wbg_set_min_binding_size_308360802ae7a9ba(arg0, arg1) {
    getObject(arg0).minBindingSize = arg1;
}
export function __wbg_set_min_filter_2dafbdeb188fd817(arg0, arg1) {
    getObject(arg0).minFilter = __wbindgen_enum_GpuFilterMode[arg1];
}
export function __wbg_set_mip_level_babe1ff64201f0ea(arg0, arg1) {
    getObject(arg0).mipLevel = arg1 >>> 0;
}
export function __wbg_set_mip_level_count_cd3197411f4f2432(arg0, arg1) {
    getObject(arg0).mipLevelCount = arg1 >>> 0;
}
export function __wbg_set_mip_level_count_fdc72450a94244ef(arg0, arg1) {
    getObject(arg0).mipLevelCount = arg1 >>> 0;
}
export function __wbg_set_mipmap_filter_79f552c459e63aa6(arg0, arg1) {
    getObject(arg0).mipmapFilter = __wbindgen_enum_GpuMipmapFilterMode[arg1];
}
export function __wbg_set_module_18d541838665d831(arg0, arg1) {
    getObject(arg0).module = getObject(arg1);
}
export function __wbg_set_module_20641353ebb28712(arg0, arg1) {
    getObject(arg0).module = getObject(arg1);
}
export function __wbg_set_multisample_e0f310ea9e40c2d9(arg0, arg1) {
    getObject(arg0).multisample = getObject(arg1);
}
export function __wbg_set_multisampled_cd50d8f6709cea1a(arg0, arg1) {
    getObject(arg0).multisampled = arg1 !== 0;
}
export function __wbg_set_offset_2e78915f5d65d704(arg0, arg1) {
    getObject(arg0).offset = arg1;
}
export function __wbg_set_offset_405017033a936d89(arg0, arg1) {
    getObject(arg0).offset = arg1;
}
export function __wbg_set_offset_e7ce8b8eaaf46b95(arg0, arg1) {
    getObject(arg0).offset = arg1;
}
export function __wbg_set_operation_b96fabca3716aaa3(arg0, arg1) {
    getObject(arg0).operation = __wbindgen_enum_GpuBlendOperation[arg1];
}
export function __wbg_set_origin_c5f017d3f09ad7ff(arg0, arg1) {
    getObject(arg0).origin = getObject(arg1);
}
export function __wbg_set_pass_op_765be90bb2f27220(arg0, arg1) {
    getObject(arg0).passOp = __wbindgen_enum_GpuStencilOperation[arg1];
}
export function __wbg_set_power_preference_39b347bf0d236ce6(arg0, arg1) {
    getObject(arg0).powerPreference = __wbindgen_enum_GpuPowerPreference[arg1];
}
export function __wbg_set_primitive_d6456d7efe6b4fe5(arg0, arg1) {
    getObject(arg0).primitive = getObject(arg1);
}
export function __wbg_set_query_set_20ecd7f9a16f3ec6(arg0, arg1) {
    getObject(arg0).querySet = getObject(arg1);
}
export function __wbg_set_r_07bd987697069496(arg0, arg1) {
    getObject(arg0).r = arg1;
}
export function __wbg_set_required_features_650c9e5dafbaa395(arg0, arg1) {
    getObject(arg0).requiredFeatures = getObject(arg1);
}
export function __wbg_set_resolve_target_c18cd4048765732a(arg0, arg1) {
    getObject(arg0).resolveTarget = getObject(arg1);
}
export function __wbg_set_resource_8cea0fe2c8745c3e(arg0, arg1) {
    getObject(arg0).resource = getObject(arg1);
}
export function __wbg_set_rows_per_image_2f7969031c71f0d8(arg0, arg1) {
    getObject(arg0).rowsPerImage = arg1 >>> 0;
}
export function __wbg_set_sample_count_07aedd28692aeae8(arg0, arg1) {
    getObject(arg0).sampleCount = arg1 >>> 0;
}
export function __wbg_set_sample_type_601a744a4bd6ea07(arg0, arg1) {
    getObject(arg0).sampleType = __wbindgen_enum_GpuTextureSampleType[arg1];
}
export function __wbg_set_sampler_1a2729c0aa194081(arg0, arg1) {
    getObject(arg0).sampler = getObject(arg1);
}
export function __wbg_set_shader_location_bdcfdc1009d351b1(arg0, arg1) {
    getObject(arg0).shaderLocation = arg1 >>> 0;
}
export function __wbg_set_size_7a392ee585f87da8(arg0, arg1) {
    getObject(arg0).size = arg1;
}
export function __wbg_set_size_c6bf409f70f4420f(arg0, arg1) {
    getObject(arg0).size = getObject(arg1);
}
export function __wbg_set_size_f902b266d636bf6e(arg0, arg1) {
    getObject(arg0).size = arg1;
}
export function __wbg_set_src_factor_50cef27aa8aece91(arg0, arg1) {
    getObject(arg0).srcFactor = __wbindgen_enum_GpuBlendFactor[arg1];
}
export function __wbg_set_stencil_back_e740415a5c0b637a(arg0, arg1) {
    getObject(arg0).stencilBack = getObject(arg1);
}
export function __wbg_set_stencil_clear_value_6be76b512040398d(arg0, arg1) {
    getObject(arg0).stencilClearValue = arg1 >>> 0;
}
export function __wbg_set_stencil_front_03185e1c3bafa411(arg0, arg1) {
    getObject(arg0).stencilFront = getObject(arg1);
}
export function __wbg_set_stencil_load_op_084f44352b978b3d(arg0, arg1) {
    getObject(arg0).stencilLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
}
export function __wbg_set_stencil_read_mask_e2736fc4af9399e4(arg0, arg1) {
    getObject(arg0).stencilReadMask = arg1 >>> 0;
}
export function __wbg_set_stencil_read_only_31f3d99299373c12(arg0, arg1) {
    getObject(arg0).stencilReadOnly = arg1 !== 0;
}
export function __wbg_set_stencil_store_op_428fb4955e4899d6(arg0, arg1) {
    getObject(arg0).stencilStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
}
export function __wbg_set_stencil_write_mask_b1d3e1655305a187(arg0, arg1) {
    getObject(arg0).stencilWriteMask = arg1 >>> 0;
}
export function __wbg_set_step_mode_98e49f7877daf1c5(arg0, arg1) {
    getObject(arg0).stepMode = __wbindgen_enum_GpuVertexStepMode[arg1];
}
export function __wbg_set_storage_texture_6ee0cbeb50698110(arg0, arg1) {
    getObject(arg0).storageTexture = getObject(arg1);
}
export function __wbg_set_store_op_e761080d541a10cc(arg0, arg1) {
    getObject(arg0).storeOp = __wbindgen_enum_GpuStoreOp[arg1];
}
export function __wbg_set_strip_index_format_16df9e33c7aa97e6(arg0, arg1) {
    getObject(arg0).stripIndexFormat = __wbindgen_enum_GpuIndexFormat[arg1];
}
export function __wbg_set_targets_9fd1ec0b8edc895c(arg0, arg1) {
    getObject(arg0).targets = getObject(arg1);
}
export function __wbg_set_texture_f03807916f70dcc6(arg0, arg1) {
    getObject(arg0).texture = getObject(arg1);
}
export function __wbg_set_texture_f8ae0bb4bb159354(arg0, arg1) {
    getObject(arg0).texture = getObject(arg1);
}
export function __wbg_set_timestamp_writes_3998dbfa21e48dbe(arg0, arg1) {
    getObject(arg0).timestampWrites = getObject(arg1);
}
export function __wbg_set_topology_036632318a24227d(arg0, arg1) {
    getObject(arg0).topology = __wbindgen_enum_GpuPrimitiveTopology[arg1];
}
export function __wbg_set_type_0cb4cdb5eff87f31(arg0, arg1) {
    getObject(arg0).type = __wbindgen_enum_GpuBufferBindingType[arg1];
}
export function __wbg_set_type_d05fa8415ad0761f(arg0, arg1) {
    getObject(arg0).type = __wbindgen_enum_GpuSamplerBindingType[arg1];
}
export function __wbg_set_unclipped_depth_17a5ab83d4e7cadc(arg0, arg1) {
    getObject(arg0).unclippedDepth = arg1 !== 0;
}
export function __wbg_set_usage_3d569e7b02227032(arg0, arg1) {
    getObject(arg0).usage = arg1 >>> 0;
}
export function __wbg_set_usage_ac222ece73f994b7(arg0, arg1) {
    getObject(arg0).usage = arg1 >>> 0;
}
export function __wbg_set_usage_ca00520767c8a475(arg0, arg1) {
    getObject(arg0).usage = arg1 >>> 0;
}
export function __wbg_set_usage_fe13088353b65bee(arg0, arg1) {
    getObject(arg0).usage = arg1 >>> 0;
}
export function __wbg_set_vertex_76b7ac4bdfbb06f4(arg0, arg1) {
    getObject(arg0).vertex = getObject(arg1);
}
export function __wbg_set_view_1ef41eeb26eaf718(arg0, arg1) {
    getObject(arg0).view = getObject(arg1);
}
export function __wbg_set_view_46b654a12649c6f6(arg0, arg1) {
    getObject(arg0).view = getObject(arg1);
}
export function __wbg_set_view_dimension_12c332494a2697dc(arg0, arg1) {
    getObject(arg0).viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
}
export function __wbg_set_view_dimension_31b9fd7126132e82(arg0, arg1) {
    getObject(arg0).viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
}
export function __wbg_set_view_formats_152cb995add2ee4e(arg0, arg1) {
    getObject(arg0).viewFormats = getObject(arg1);
}
export function __wbg_set_view_formats_cc77650da6c3b25b(arg0, arg1) {
    getObject(arg0).viewFormats = getObject(arg1);
}
export function __wbg_set_visibility_6d1fc94552f22ac3(arg0, arg1) {
    getObject(arg0).visibility = arg1 >>> 0;
}
export function __wbg_set_width_5ee1e2d4a0fd929b(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
}
export function __wbg_set_width_7f07715a20503914(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
}
export function __wbg_set_width_d60bc4f2f20c56a4(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
}
export function __wbg_set_write_mask_c92743022356850e(arg0, arg1) {
    getObject(arg0).writeMask = arg1 >>> 0;
}
export function __wbg_set_x_0771b0f86d56cdf9(arg0, arg1) {
    getObject(arg0).x = arg1 >>> 0;
}
export function __wbg_set_y_668d1578881576dd(arg0, arg1) {
    getObject(arg0).y = arg1 >>> 0;
}
export function __wbg_set_z_3e24a918a76c816d(arg0, arg1) {
    getObject(arg0).z = arg1 >>> 0;
}
export function __wbg_static_accessor_GLOBAL_12837167ad935116() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_static_accessor_SELF_a621d3dfbb60d0ce() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_static_accessor_WINDOW_f8727f0cf888e0bd() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_submit_a1850a1cb6baf64a(arg0, arg1) {
    getObject(arg0).submit(getObject(arg1));
}
export function __wbg_then_0d9fe2c7b1857d32(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}
export function __wbg_then_b9e7b3b5f1a9e1b5(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_unmap_ab94ab04cfb14bee(arg0) {
    getObject(arg0).unmap();
}
export function __wbg_warn_a40b971467b219c7(arg0, arg1, arg2, arg3) {
    console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
}
export function __wbg_width_5f66bde2e810fbde(arg0) {
    const ret = getObject(arg0).width;
    return ret;
}
export function __wbg_writeBuffer_b203cf79b98d6dd8() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).writeBuffer(getObject(arg1), arg2, getObject(arg3), arg4, arg5);
}, arguments); }
export function __wbg_writeTexture_0466bf7d7d35e04e() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).writeTexture(getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
}, arguments); }
export function __wbindgen_cast_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 93, function: Function { arguments: [Externref], shim_idx: 94, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_1830, __wasm_bindgen_func_elem_1846);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000002(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000003(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
    const ret = getArrayU8FromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000004(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_object_clone_ref(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
}
export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
}
function __wasm_bindgen_func_elem_1846(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_1846(arg0, arg1, addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_2522(arg0, arg1, arg2, arg3) {
    wasm.__wasm_bindgen_func_elem_2522(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}


const __wbindgen_enum_GpuAddressMode = ["clamp-to-edge", "repeat", "mirror-repeat"];


const __wbindgen_enum_GpuBlendFactor = ["zero", "one", "src", "one-minus-src", "src-alpha", "one-minus-src-alpha", "dst", "one-minus-dst", "dst-alpha", "one-minus-dst-alpha", "src-alpha-saturated", "constant", "one-minus-constant", "src1", "one-minus-src1", "src1-alpha", "one-minus-src1-alpha"];


const __wbindgen_enum_GpuBlendOperation = ["add", "subtract", "reverse-subtract", "min", "max"];


const __wbindgen_enum_GpuBufferBindingType = ["uniform", "storage", "read-only-storage"];


const __wbindgen_enum_GpuCanvasAlphaMode = ["opaque", "premultiplied"];


const __wbindgen_enum_GpuCompareFunction = ["never", "less", "equal", "less-equal", "greater", "not-equal", "greater-equal", "always"];


const __wbindgen_enum_GpuCullMode = ["none", "front", "back"];


const __wbindgen_enum_GpuFilterMode = ["nearest", "linear"];


const __wbindgen_enum_GpuFrontFace = ["ccw", "cw"];


const __wbindgen_enum_GpuIndexFormat = ["uint16", "uint32"];


const __wbindgen_enum_GpuLoadOp = ["load", "clear"];


const __wbindgen_enum_GpuMipmapFilterMode = ["nearest", "linear"];


const __wbindgen_enum_GpuPowerPreference = ["low-power", "high-performance"];


const __wbindgen_enum_GpuPrimitiveTopology = ["point-list", "line-list", "line-strip", "triangle-list", "triangle-strip"];


const __wbindgen_enum_GpuSamplerBindingType = ["filtering", "non-filtering", "comparison"];


const __wbindgen_enum_GpuStencilOperation = ["keep", "zero", "replace", "invert", "increment-clamp", "decrement-clamp", "increment-wrap", "decrement-wrap"];


const __wbindgen_enum_GpuStorageTextureAccess = ["write-only", "read-only", "read-write"];


const __wbindgen_enum_GpuStoreOp = ["store", "discard"];


const __wbindgen_enum_GpuTextureAspect = ["all", "stencil-only", "depth-only"];


const __wbindgen_enum_GpuTextureDimension = ["1d", "2d", "3d"];


const __wbindgen_enum_GpuTextureFormat = ["r8unorm", "r8snorm", "r8uint", "r8sint", "r16uint", "r16sint", "r16float", "rg8unorm", "rg8snorm", "rg8uint", "rg8sint", "r32uint", "r32sint", "r32float", "rg16uint", "rg16sint", "rg16float", "rgba8unorm", "rgba8unorm-srgb", "rgba8snorm", "rgba8uint", "rgba8sint", "bgra8unorm", "bgra8unorm-srgb", "rgb9e5ufloat", "rgb10a2uint", "rgb10a2unorm", "rg11b10ufloat", "rg32uint", "rg32sint", "rg32float", "rgba16uint", "rgba16sint", "rgba16float", "rgba32uint", "rgba32sint", "rgba32float", "stencil8", "depth16unorm", "depth24plus", "depth24plus-stencil8", "depth32float", "depth32float-stencil8", "bc1-rgba-unorm", "bc1-rgba-unorm-srgb", "bc2-rgba-unorm", "bc2-rgba-unorm-srgb", "bc3-rgba-unorm", "bc3-rgba-unorm-srgb", "bc4-r-unorm", "bc4-r-snorm", "bc5-rg-unorm", "bc5-rg-snorm", "bc6h-rgb-ufloat", "bc6h-rgb-float", "bc7-rgba-unorm", "bc7-rgba-unorm-srgb", "etc2-rgb8unorm", "etc2-rgb8unorm-srgb", "etc2-rgb8a1unorm", "etc2-rgb8a1unorm-srgb", "etc2-rgba8unorm", "etc2-rgba8unorm-srgb", "eac-r11unorm", "eac-r11snorm", "eac-rg11unorm", "eac-rg11snorm", "astc-4x4-unorm", "astc-4x4-unorm-srgb", "astc-5x4-unorm", "astc-5x4-unorm-srgb", "astc-5x5-unorm", "astc-5x5-unorm-srgb", "astc-6x5-unorm", "astc-6x5-unorm-srgb", "astc-6x6-unorm", "astc-6x6-unorm-srgb", "astc-8x5-unorm", "astc-8x5-unorm-srgb", "astc-8x6-unorm", "astc-8x6-unorm-srgb", "astc-8x8-unorm", "astc-8x8-unorm-srgb", "astc-10x5-unorm", "astc-10x5-unorm-srgb", "astc-10x6-unorm", "astc-10x6-unorm-srgb", "astc-10x8-unorm", "astc-10x8-unorm-srgb", "astc-10x10-unorm", "astc-10x10-unorm-srgb", "astc-12x10-unorm", "astc-12x10-unorm-srgb", "astc-12x12-unorm", "astc-12x12-unorm-srgb"];


const __wbindgen_enum_GpuTextureSampleType = ["float", "unfilterable-float", "depth", "sint", "uint"];


const __wbindgen_enum_GpuTextureViewDimension = ["1d", "2d", "2d-array", "cube", "cube-array", "3d"];


const __wbindgen_enum_GpuVertexFormat = ["uint8", "uint8x2", "uint8x4", "sint8", "sint8x2", "sint8x4", "unorm8", "unorm8x2", "unorm8x4", "snorm8", "snorm8x2", "snorm8x4", "uint16", "uint16x2", "uint16x4", "sint16", "sint16x2", "sint16x4", "unorm16", "unorm16x2", "unorm16x4", "snorm16", "snorm16x2", "snorm16x4", "float16", "float16x2", "float16x4", "float32", "float32x2", "float32x3", "float32x4", "uint32", "uint32x2", "uint32x3", "uint32x4", "sint32", "sint32x2", "sint32x3", "sint32x4", "unorm10-10-10-2", "unorm8x4-bgra"];


const __wbindgen_enum_GpuVertexStepMode = ["vertex", "instance"];
const BloomEngineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bloomengine_free(ptr >>> 0, 1));

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export3(addHeapObject(e));
    }
}

let heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
