(function () {
    'use strict';

    var _documentCurrentScript = typeof document !== 'undefined' ? document.currentScript : null;
    /* @ts-self-types="./gba_web_frontend.d.ts" */

    function run_app() {
        const ret = wasm.run_app();
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    function __wbg_get_imports() {
        const import0 = {
            __proto__: null,
            __wbg_Error_408e67f47ca7b58b: function(arg0, arg1) {
                const ret = Error(getStringFromWasm0(arg0, arg1));
                return ret;
            },
            __wbg_Number_3890faa6d3ff057d: function(arg0) {
                const ret = Number(arg0);
                return ret;
            },
            __wbg___wbindgen_boolean_get_c9c83ebd41b34df3: function(arg0) {
                const v = arg0;
                const ret = typeof(v) === 'boolean' ? v : undefined;
                return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
            },
            __wbg___wbindgen_debug_string_a57024b9c6e4a48b: function(arg0, arg1) {
                const ret = debugString(arg1);
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg___wbindgen_in_ac983077f137f2e6: function(arg0, arg1) {
                const ret = arg0 in arg1;
                return ret;
            },
            __wbg___wbindgen_is_function_5e4570eb24ffa122: function(arg0) {
                const ret = typeof(arg0) === 'function';
                return ret;
            },
            __wbg___wbindgen_is_null_7d13f41e1a2d5140: function(arg0) {
                const ret = arg0 === null;
                return ret;
            },
            __wbg___wbindgen_is_object_a2790eb24c211ea0: function(arg0) {
                const val = arg0;
                const ret = typeof(val) === 'object' && val !== null;
                return ret;
            },
            __wbg___wbindgen_is_string_e6f02f0ea5f20a32: function(arg0) {
                const ret = typeof(arg0) === 'string';
                return ret;
            },
            __wbg___wbindgen_is_undefined_6cff064c44e0d823: function(arg0) {
                const ret = arg0 === undefined;
                return ret;
            },
            __wbg___wbindgen_jsval_eq_0a18949a61670320: function(arg0, arg1) {
                const ret = arg0 === arg1;
                return ret;
            },
            __wbg___wbindgen_jsval_loose_eq_acf2776254a8d832: function(arg0, arg1) {
                const ret = arg0 == arg1;
                return ret;
            },
            __wbg___wbindgen_number_get_136b9679cab35cfb: function(arg0, arg1) {
                const obj = arg1;
                const ret = typeof(obj) === 'number' ? obj : undefined;
                getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
            },
            __wbg___wbindgen_string_get_d154f1e671052120: function(arg0, arg1) {
                const obj = arg1;
                const ret = typeof(obj) === 'string' ? obj : undefined;
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg___wbindgen_throw_bb96b2010945f0bc: function(arg0, arg1) {
                throw new Error(getStringFromWasm0(arg0, arg1));
            },
            __wbg__wbg_cb_unref_be22cc64ae6946a0: function(arg0) {
                arg0._wbg_cb_unref();
            },
            __wbg_abort_cb24946e29699837: function(arg0) {
                arg0.abort();
            },
            __wbg_addEventListener_3b8edc02c33d9f77: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
            }, arguments); },
            __wbg_addEventListener_d6fb728fba6ad35c: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
            }, arguments); },
            __wbg_altKey_755975127b4ad2c8: function(arg0) {
                const ret = arg0.altKey;
                return ret;
            },
            __wbg_beginPath_4b87fe7ed5408cac: function(arg0) {
                arg0.beginPath();
            },
            __wbg_body_d6eca0586d628e3c: function(arg0) {
                const ret = arg0.body;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_bubbles_004494fc9c11b448: function(arg0) {
                const ret = arg0.bubbles;
                return ret;
            },
            __wbg_cache_key_581e6d43e117266a: function(arg0) {
                const ret = arg0.__yew_subtree_cache_key;
                return isLikeNone(ret) ? Number.MAX_SAFE_INTEGER : (ret) >>> 0;
            },
            __wbg_call_35dba3c747ad7521: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.call(arg1, arg2);
                return ret;
            }, arguments); },
            __wbg_cancelBubble_3c22a4a11bfa15f9: function(arg0) {
                const ret = arg0.cancelBubble;
                return ret;
            },
            __wbg_childNodes_7410f7798ab1eb2b: function(arg0) {
                const ret = arg0.childNodes;
                return ret;
            },
            __wbg_clearRect_81c3c80fbe793b63: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.clearRect(arg1, arg2, arg3, arg4);
            },
            __wbg_cloneNode_1c667adc0c119cfa: function() { return handleError(function (arg0) {
                const ret = arg0.cloneNode();
                return ret;
            }, arguments); },
            __wbg_code_1bac1fd03147d97e: function(arg0, arg1) {
                const ret = arg1.code;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_commit_ea28440d4f5a8dd8: function() { return handleError(function (arg0) {
                arg0.commit();
            }, arguments); },
            __wbg_composedPath_fc8c6c0a810bd2dc: function(arg0) {
                const ret = arg0.composedPath();
                return ret;
            },
            __wbg_connect_d2a36cf1f5a1ec54: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.connect(arg1);
                return ret;
            }, arguments); },
            __wbg_copyToChannel_a9a409d91e35f309: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                arg0.copyToChannel(getArrayF32FromWasm0(arg1, arg2), arg3);
            }, arguments); },
            __wbg_createBufferSource_3679674c3bfc1e4e: function() { return handleError(function (arg0) {
                const ret = arg0.createBufferSource();
                return ret;
            }, arguments); },
            __wbg_createBuffer_9b192707f1e81570: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = arg0.createBuffer(arg1 >>> 0, arg2 >>> 0, arg3);
                return ret;
            }, arguments); },
            __wbg_createElementNS_f18ede2d74f15ea1: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                const ret = arg0.createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
                return ret;
            }, arguments); },
            __wbg_createElement_7f42344eee7bb810: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
                return ret;
            }, arguments); },
            __wbg_createObjectStore_2ddc8f74181944c1: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = arg0.createObjectStore(getStringFromWasm0(arg1, arg2), arg3);
                return ret;
            }, arguments); },
            __wbg_createTextNode_f5ee2b1cd3e249bb: function(arg0, arg1, arg2) {
                const ret = arg0.createTextNode(getStringFromWasm0(arg1, arg2));
                return ret;
            },
            __wbg_crypto_38df2bab126b63dc: function(arg0) {
                const ret = arg0.crypto;
                return ret;
            },
            __wbg_ctrlKey_9490b716a4845258: function(arg0) {
                const ret = arg0.ctrlKey;
                return ret;
            },
            __wbg_currentTime_5594ee0e8ef1889a: function(arg0) {
                const ret = arg0.currentTime;
                return ret;
            },
            __wbg_debug_3853dbaf0bca30f9: function(arg0) {
                console.debug(arg0);
            },
            __wbg_destination_f6ba56e7f07829d0: function(arg0) {
                const ret = arg0.destination;
                return ret;
            },
            __wbg_document_ac38448dbfd31a57: function(arg0) {
                const ret = arg0.document;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_drawImage_c9877592da6891fb: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.drawImage(arg1, arg2, arg3, arg4, arg5);
            }, arguments); },
            __wbg_entries_7774d489e1da5f4f: function(arg0) {
                const ret = Object.entries(arg0);
                return ret;
            },
            __wbg_error_24e6ac605d438e54: function() { return handleError(function (arg0) {
                const ret = arg0.error;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_error_757e9472f8410341: function(arg0, arg1) {
                let deferred0_0;
                let deferred0_1;
                try {
                    deferred0_0 = arg0;
                    deferred0_1 = arg1;
                    console.error(getStringFromWasm0(arg0, arg1));
                } finally {
                    wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
                }
            },
            __wbg_error_b1df75078f69c897: function(arg0, arg1) {
                var v0 = getArrayJsValueFromWasm0(arg0, arg1);
                wasm.__wbindgen_free(arg0, arg1 * 4, 4);
                console.error(...v0);
            },
            __wbg_error_d051ede168a8fa99: function(arg0) {
                const ret = arg0.error;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_error_dd408a7b3cb542dd: function(arg0) {
                console.error(arg0);
            },
            __wbg_files_56a897754f75826b: function(arg0) {
                const ret = arg0.files;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_fillRect_3077c0e38eb34cd1: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.fillRect(arg1, arg2, arg3, arg4);
            },
            __wbg_from_74f3d90e0ff11240: function(arg0) {
                const ret = Array.from(arg0);
                return ret;
            },
            __wbg_getContext_71c33f14b63da593: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getElementById_1637d6969b003cda: function(arg0, arg1, arg2) {
                const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_getRandomValues_c44a50d8cfdaebeb: function() { return handleError(function (arg0, arg1) {
                arg0.getRandomValues(arg1);
            }, arguments); },
            __wbg_get_4babbbf9303c1945: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.get(arg1);
                return ret;
            }, arguments); },
            __wbg_get_836a517ee3483cda: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_get_971a0c45d172643f: function() { return handleError(function (arg0, arg1) {
                const ret = Reflect.get(arg0, arg1);
                return ret;
            }, arguments); },
            __wbg_get_c0c8f8d7da0c03dd: function(arg0, arg1) {
                const ret = arg0[arg1 >>> 0];
                return ret;
            },
            __wbg_get_with_ref_key_6412cf3094599694: function(arg0, arg1) {
                const ret = arg0[arg1];
                return ret;
            },
            __wbg_hash_55f5fe24d9863630: function(arg0, arg1) {
                const ret = arg1.hash;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_hash_597d991faa794205: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.hash;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_height_e56f6fb197710e09: function(arg0) {
                const ret = arg0.height;
                return ret;
            },
            __wbg_history_8941dec1bcca1cc5: function() { return handleError(function (arg0) {
                const ret = arg0.history;
                return ret;
            }, arguments); },
            __wbg_host_f512e97ce1222138: function(arg0) {
                const ret = arg0.host;
                return ret;
            },
            __wbg_href_ab966bccc773240e: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.href;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_href_be4730f759121453: function(arg0, arg1) {
                const ret = arg1.href;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_info_726982aff9befe16: function(arg0) {
                console.info(arg0);
            },
            __wbg_insertBefore_f3733e91a030079e: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.insertBefore(arg1, arg2);
                return ret;
            }, arguments); },
            __wbg_instanceof_ArrayBuffer_993d02d2d254cad1: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof ArrayBuffer;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_CanvasRenderingContext2d_d23139c3ef7651a3: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof CanvasRenderingContext2D;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_Element_a3960bb00f4964bc: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof Element;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_HtmlCanvasElement_327e7f7530c72bbd: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof HTMLCanvasElement;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_HtmlInputElement_6077656bcaf1eb33: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof HTMLInputElement;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_IdbDatabase_e9dd9f20c51d8d42: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof IDBDatabase;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_IdbFactory_8b61495ce09d6c93: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof IDBFactory;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_IdbOpenDbRequest_b913751ffb9239bb: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof IDBOpenDBRequest;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_IdbRequest_471b050024626dac: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof IDBRequest;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_ShadowRoot_55844b1b54688323: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof ShadowRoot;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_Uint8Array_f935dbb0aa7cdeed: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof Uint8Array;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_Window_5625ff9937037a38: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof Window;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_isSafeInteger_f3d6cd19ccfe4512: function(arg0) {
                const ret = Number.isSafeInteger(arg0);
                return ret;
            },
            __wbg_is_86be747e88e872fb: function(arg0, arg1) {
                const ret = Object.is(arg0, arg1);
                return ret;
            },
            __wbg_key_d1b2fd5ee42567c0: function(arg0, arg1) {
                const ret = arg1.key;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_lastChild_c18578f426fb6405: function(arg0) {
                const ret = arg0.lastChild;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_length_2dd58ff350b5afcd: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_length_36bd29c6848c2144: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_length_ecfa2c63d3d0d82c: function(arg0) {
                const ret = arg0.length;
                return ret;
            },
            __wbg_lineTo_9495a068a4f48283: function(arg0, arg1, arg2) {
                arg0.lineTo(arg1, arg2);
            },
            __wbg_listener_id_fd5d191d59511336: function(arg0) {
                const ret = arg0.__yew_listener_id;
                return isLikeNone(ret) ? Number.MAX_SAFE_INTEGER : (ret) >>> 0;
            },
            __wbg_location_5d269cf0aa99107a: function(arg0) {
                const ret = arg0.location;
                return ret;
            },
            __wbg_log_e6372b4fbfc9f81e: function(arg0) {
                console.log(arg0);
            },
            __wbg_message_88eda073e68b1d26: function(arg0, arg1) {
                const ret = arg1.message;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_metaKey_f282cd52fbd7cb27: function(arg0) {
                const ret = arg0.metaKey;
                return ret;
            },
            __wbg_moveTo_a5882cdf1a7d39d9: function(arg0, arg1, arg2) {
                arg0.moveTo(arg1, arg2);
            },
            __wbg_msCrypto_bd5a034af96bcba6: function(arg0) {
                const ret = arg0.msCrypto;
                return ret;
            },
            __wbg_name_41b795553ec88cd8: function(arg0, arg1) {
                const ret = arg1.name;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_name_facbed56940f0fec: function(arg0, arg1) {
                const ret = arg1.name;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_namespaceURI_1d0ee22eac981c98: function(arg0, arg1) {
                const ret = arg1.namespaceURI;
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_new_032f5cf47e7b0cae: function() { return handleError(function () {
                const ret = new lAudioContext();
                return ret;
            }, arguments); },
            __wbg_new_116be93542d39019: function() {
                const ret = new Array();
                return ret;
            },
            __wbg_new_1f27644530c822b2: function() { return handleError(function () {
                const ret = new FileReader();
                return ret;
            }, arguments); },
            __wbg_new_227d7c05414eb861: function() {
                const ret = new Error();
                return ret;
            },
            __wbg_new_77cc4f4f472aeb81: function(arg0) {
                const ret = new Uint8Array(arg0);
                return ret;
            },
            __wbg_new_c4a5c3368506feb1: function() { return handleError(function (arg0, arg1) {
                const ret = new URL(getStringFromWasm0(arg0, arg1));
                return ret;
            }, arguments); },
            __wbg_new_ebe3e0f6837f0879: function() {
                const ret = new Object();
                return ret;
            },
            __wbg_new_from_slice_3eea173078478cfe: function(arg0, arg1) {
                const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
                return ret;
            },
            __wbg_new_with_base_6b1b9574fc18e5ab: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = new URL(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
                return ret;
            }, arguments); },
            __wbg_new_with_length_3ffc1c56427c525c: function(arg0) {
                const ret = new Uint8Array(arg0 >>> 0);
                return ret;
            },
            __wbg_new_with_u8_clamped_array_and_sh_d9a3bf9abac17f51: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
                return ret;
            }, arguments); },
            __wbg_nextSibling_1270411ea2610f57: function(arg0) {
                const ret = arg0.nextSibling;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_node_84ea875411254db1: function(arg0) {
                const ret = arg0.node;
                return ret;
            },
            __wbg_now_2283802bfbda617e: function(arg0) {
                const ret = arg0.now();
                return ret;
            },
            __wbg_now_8b265300afd5f2b9: function() {
                const ret = Date.now();
                return ret;
            },
            __wbg_objectStore_222b7add2b5c2770: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.objectStore(getStringFromWasm0(arg1, arg2));
                return ret;
            }, arguments); },
            __wbg_open_66c8b00ee562451f: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.open(getStringFromWasm0(arg1, arg2));
                return ret;
            }, arguments); },
            __wbg_open_c5ecda93515ce190: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                const ret = arg0.open(getStringFromWasm0(arg1, arg2), arg3 >>> 0);
                return ret;
            }, arguments); },
            __wbg_outerHTML_6b872f67d4531f96: function(arg0, arg1) {
                const ret = arg1.outerHTML;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_parentElement_ef76606593484767: function(arg0) {
                const ret = arg0.parentElement;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_parentNode_8634e029370ec1bb: function(arg0) {
                const ret = arg0.parentNode;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_pathname_8669139de28a77bd: function(arg0, arg1) {
                const ret = arg1.pathname;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_pathname_bc564f9e4fcdd029: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.pathname;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_performance_821a3767f0dce300: function(arg0) {
                const ret = arg0.performance;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_preventDefault_19878c58b8010668: function(arg0) {
                arg0.preventDefault();
            },
            __wbg_process_44c7a14e11e9f69e: function(arg0) {
                const ret = arg0.process;
                return ret;
            },
            __wbg_prototypesetcall_de8e0d9553586985: function(arg0, arg1, arg2) {
                Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
            },
            __wbg_pushState_eb9e1dcf29d2f366: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.pushState(arg1, getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
            }, arguments); },
            __wbg_push_adb0107829f02d75: function(arg0, arg1) {
                const ret = arg0.push(arg1);
                return ret;
            },
            __wbg_putImageData_17fd10517d5503a3: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                arg0.putImageData(arg1, arg2, arg3);
            }, arguments); },
            __wbg_put_49ed48c98d0c0d3c: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.put(arg1);
                return ret;
            }, arguments); },
            __wbg_put_5e0ae8c80bb952a7: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.put(arg1, arg2);
                return ret;
            }, arguments); },
            __wbg_queueMicrotask_ac694eae12e92dfb: function(arg0) {
                queueMicrotask(arg0);
            },
            __wbg_queueMicrotask_be5fe34a8f4cad4d: function(arg0) {
                const ret = arg0.queueMicrotask;
                return ret;
            },
            __wbg_randomFillSync_6c25eac9869eb53c: function() { return handleError(function (arg0, arg1) {
                arg0.randomFillSync(arg1);
            }, arguments); },
            __wbg_readAsArrayBuffer_1e0bf6cd0613d7fd: function() { return handleError(function (arg0, arg1) {
                arg0.readAsArrayBuffer(arg1);
            }, arguments); },
            __wbg_readyState_b5bf96748c63c0bb: function(arg0) {
                const ret = arg0.readyState;
                return ret;
            },
            __wbg_removeAttribute_bb10532a6f012605: function() { return handleError(function (arg0, arg1, arg2) {
                arg0.removeAttribute(getStringFromWasm0(arg1, arg2));
            }, arguments); },
            __wbg_removeChild_58f3071cb194ee29: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.removeChild(arg1);
                return ret;
            }, arguments); },
            __wbg_removeEventListener_aa653c6b402cc27e: function() { return handleError(function (arg0, arg1, arg2, arg3) {
                arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3);
            }, arguments); },
            __wbg_removeEventListener_f0778286eef3aecc: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4 !== 0);
            }, arguments); },
            __wbg_replaceState_e7ac4029d2fd43ae: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.replaceState(arg1, getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
            }, arguments); },
            __wbg_requestAnimationFrame_bcb3ce6247e27dd4: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.requestAnimationFrame(arg1);
                return ret;
            }, arguments); },
            __wbg_require_b4edbdcf3e2a1ef0: function() { return handleError(function () {
                const ret = module.require;
                return ret;
            }, arguments); },
            __wbg_resolve_020f95d838c6ef25: function(arg0) {
                const ret = Promise.resolve(arg0);
                return ret;
            },
            __wbg_result_0501bea148306f01: function() { return handleError(function (arg0) {
                const ret = arg0.result;
                return ret;
            }, arguments); },
            __wbg_result_89c2bfc79be07ad2: function() { return handleError(function (arg0) {
                const ret = arg0.result;
                return ret;
            }, arguments); },
            __wbg_resume_d3c27715f0790def: function() { return handleError(function (arg0) {
                const ret = arg0.resume();
                return ret;
            }, arguments); },
            __wbg_sampleRate_7751976089d109e1: function(arg0) {
                const ret = arg0.sampleRate;
                return ret;
            },
            __wbg_search_3ab40a92dceeaacb: function(arg0, arg1) {
                const ret = arg1.search;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_search_e85b52d847b83659: function() { return handleError(function (arg0, arg1) {
                const ret = arg1.search;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            }, arguments); },
            __wbg_setAttribute_507f8367905a9c03: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
                arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            }, arguments); },
            __wbg_setTimeout_a89fa3173dd1b518: function(arg0, arg1) {
                const ret = setTimeout(arg0, arg1);
                return ret;
            },
            __wbg_set_6be42768c690e380: function(arg0, arg1, arg2) {
                arg0[arg1] = arg2;
            },
            __wbg_set_8155bb79a948541b: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = Reflect.set(arg0, arg1, arg2);
                return ret;
            }, arguments); },
            __wbg_set_buffer_ef94b43a403b11b5: function(arg0, arg1) {
                arg0.buffer = arg1;
            },
            __wbg_set_cache_key_65a529cd1f95fc20: function(arg0, arg1) {
                arg0.__yew_subtree_cache_key = arg1 >>> 0;
            },
            __wbg_set_capture_0fda5cbdb4353cff: function(arg0, arg1) {
                arg0.capture = arg1 !== 0;
            },
            __wbg_set_checked_c1cee3b06ce68575: function(arg0, arg1) {
                arg0.checked = arg1 !== 0;
            },
            __wbg_set_defaultValue_266f54b037f79a7d: function() { return handleError(function (arg0, arg1, arg2) {
                arg0.defaultValue = getStringFromWasm0(arg1, arg2);
            }, arguments); },
            __wbg_set_fillStyle_52e75a25be60a3ff: function(arg0, arg1, arg2) {
                arg0.fillStyle = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_hash_1e058fd06c5a7a42: function(arg0, arg1, arg2) {
                arg0.hash = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_height_d72f2b76484a44de: function(arg0, arg1) {
                arg0.height = arg1 >>> 0;
            },
            __wbg_set_innerHTML_7d84b81d6f2a9fdf: function(arg0, arg1, arg2) {
                arg0.innerHTML = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_listener_id_bdd6845f6cb449dd: function(arg0, arg1) {
                arg0.__yew_listener_id = arg1 >>> 0;
            },
            __wbg_set_nodeValue_9b1ff418691c2d97: function(arg0, arg1, arg2) {
                arg0.nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_once_7f65050c57557ff9: function(arg0, arg1) {
                arg0.once = arg1 !== 0;
            },
            __wbg_set_onerror_41278ace6abe3973: function(arg0, arg1) {
                arg0.onerror = arg1;
            },
            __wbg_set_onsuccess_86d76d6974cd57e4: function(arg0, arg1) {
                arg0.onsuccess = arg1;
            },
            __wbg_set_onupgradeneeded_79b60102909f4a5e: function(arg0, arg1) {
                arg0.onupgradeneeded = arg1;
            },
            __wbg_set_passive_acb4a6d8f5b98357: function(arg0, arg1) {
                arg0.passive = arg1 !== 0;
            },
            __wbg_set_search_65c58ba6f17e037f: function(arg0, arg1, arg2) {
                arg0.search = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_strokeStyle_cce50c69cecc2df7: function(arg0, arg1, arg2) {
                arg0.strokeStyle = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_subtree_id_7234f128830a05c9: function(arg0, arg1) {
                arg0.__yew_subtree_id = arg1 >>> 0;
            },
            __wbg_set_value_22d56bead9380ee8: function(arg0, arg1, arg2) {
                arg0.value = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_value_676e9d6f43f3c9e4: function(arg0, arg1, arg2) {
                arg0.value = getStringFromWasm0(arg1, arg2);
            },
            __wbg_set_width_36ef6630b22fc519: function(arg0, arg1) {
                arg0.width = arg1 >>> 0;
            },
            __wbg_shiftKey_d24455602deb3490: function(arg0) {
                const ret = arg0.shiftKey;
                return ret;
            },
            __wbg_slice_6438d3c2b2847d98: function(arg0, arg1) {
                const ret = arg1.slice();
                const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
                const ret = arg1.stack;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_start_f2a1f4ed432f9992: function() { return handleError(function (arg0, arg1) {
                arg0.start(arg1);
            }, arguments); },
            __wbg_state_ebb066fae96b67dc: function() { return handleError(function (arg0) {
                const ret = arg0.state;
                return ret;
            }, arguments); },
            __wbg_static_accessor_GLOBAL_THIS_466428f93b4eaa76: function() {
                const ret = typeof globalThis === 'undefined' ? null : globalThis;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_GLOBAL_c7aea38d4de089bc: function() {
                const ret = typeof global === 'undefined' ? null : global;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_SELF_42d4fae05e59267a: function() {
                const ret = typeof self === 'undefined' ? null : self;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_WINDOW_e0db14a0eba6a812: function() {
                const ret = typeof window === 'undefined' ? null : window;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_stroke_5f311844f0db0d9a: function(arg0) {
                arg0.stroke();
            },
            __wbg_subarray_a4cc58201c7359fd: function(arg0, arg1, arg2) {
                const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
                return ret;
            },
            __wbg_subtree_id_32413ad1d938625a: function(arg0) {
                const ret = arg0.__yew_subtree_id;
                return isLikeNone(ret) ? Number.MAX_SAFE_INTEGER : (ret) >>> 0;
            },
            __wbg_target_13424fe1cdc436ac: function(arg0) {
                const ret = arg0.target;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_textContent_a8ab419abd77b63c: function(arg0, arg1) {
                const ret = arg1.textContent;
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_then_7026b513a94278a8: function(arg0, arg1) {
                const ret = arg0.then(arg1);
                return ret;
            },
            __wbg_transaction_4c999693e6e601bf: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.transaction(arg1, __wbindgen_enum_IdbTransactionMode[arg2]);
                return ret;
            }, arguments); },
            __wbg_value_35f0fb42e7c3d468: function(arg0, arg1) {
                const ret = arg1.value;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_value_75dd6140b2a4f88b: function(arg0, arg1) {
                const ret = arg1.value;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_value_ea3f13bcabcbe7ca: function(arg0, arg1) {
                const ret = arg1.value;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_versions_276b2795b1c6a219: function(arg0) {
                const ret = arg0.versions;
                return ret;
            },
            __wbg_warn_917d7f727ab78481: function(arg0) {
                console.warn(arg0);
            },
            __wbg_width_1952934caca67137: function(arg0) {
                const ret = arg0.width;
                return ret;
            },
            __wbindgen_cast_0000000000000001: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 939, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___wasm_bindgen_f7b093aeba8fc579___JsValue__core_3db1b4038a372208___result__Result_____wasm_bindgen_f7b093aeba8fc579___JsError___true_);
                return ret;
            },
            __wbindgen_cast_0000000000000002: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 659, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true_);
                return ret;
            },
            __wbindgen_cast_0000000000000003: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("IDBVersionChangeEvent")], shim_idx: 384, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_KeyboardEvent__KeyboardEvent______true_);
                return ret;
            },
            __wbindgen_cast_0000000000000004: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("KeyboardEvent")], shim_idx: 384, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_KeyboardEvent__KeyboardEvent______true__3);
                return ret;
            },
            __wbindgen_cast_0000000000000005: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 799, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
                const ret = makeClosure(arg0, arg1, wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true_);
                return ret;
            },
            __wbindgen_cast_0000000000000006: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 895, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true__1_);
                return ret;
            },
            __wbindgen_cast_0000000000000007: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 909, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true__2_);
                return ret;
            },
            __wbindgen_cast_0000000000000008: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 893, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke_______true_);
                return ret;
            },
            __wbindgen_cast_0000000000000009: function(arg0) {
                // Cast intrinsic for `F64 -> Externref`.
                const ret = arg0;
                return ret;
            },
            __wbindgen_cast_000000000000000a: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
                const ret = getArrayU8FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_000000000000000b: function(arg0, arg1) {
                // Cast intrinsic for `Ref(String) -> Externref`.
                const ret = getStringFromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_init_externref_table: function() {
                const table = wasm.__wbindgen_externrefs;
                const offset = table.grow(4);
                table.set(0, undefined);
                table.set(offset + 0, undefined);
                table.set(offset + 1, null);
                table.set(offset + 2, true);
                table.set(offset + 3, false);
            },
        };
        return {
            __proto__: null,
            "./gba_web_frontend_bg.js": import0,
        };
    }

    const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
    function wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke_______true_(arg0, arg1) {
        wasm.wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke_______true_(arg0, arg1);
    }

    function wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true_(arg0, arg1, arg2) {
        wasm.wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true_(arg0, arg1, arg2);
    }

    function wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_KeyboardEvent__KeyboardEvent______true_(arg0, arg1, arg2) {
        wasm.wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_KeyboardEvent__KeyboardEvent______true_(arg0, arg1, arg2);
    }

    function wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_KeyboardEvent__KeyboardEvent______true__3(arg0, arg1, arg2) {
        wasm.wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___web_sys_f8f8074046a1c453___features__gen_KeyboardEvent__KeyboardEvent______true__3(arg0, arg1, arg2);
    }

    function wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true_(arg0, arg1, arg2) {
        wasm.wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true_(arg0, arg1, arg2);
    }

    function wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true__1_(arg0, arg1, arg2) {
        wasm.wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true__1_(arg0, arg1, arg2);
    }

    function wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true__2_(arg0, arg1, arg2) {
        wasm.wasm_bindgen_f7b093aeba8fc579___convert__closures________invoke___web_sys_f8f8074046a1c453___features__gen_Event__Event______true__2_(arg0, arg1, arg2);
    }

    function wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___wasm_bindgen_f7b093aeba8fc579___JsValue__core_3db1b4038a372208___result__Result_____wasm_bindgen_f7b093aeba8fc579___JsError___true_(arg0, arg1, arg2) {
        const ret = wasm.wasm_bindgen_f7b093aeba8fc579___convert__closures_____invoke___wasm_bindgen_f7b093aeba8fc579___JsValue__core_3db1b4038a372208___result__Result_____wasm_bindgen_f7b093aeba8fc579___JsError___true_(arg0, arg1, arg2);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }


    const __wbindgen_enum_IdbTransactionMode = ["readonly", "readwrite", "versionchange", "readwriteflush", "cleanup"];

    function addToExternrefTable0(obj) {
        const idx = wasm.__externref_table_alloc();
        wasm.__wbindgen_externrefs.set(idx, obj);
        return idx;
    }

    const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

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

    function getArrayF32FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
    }

    function getArrayJsValueFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        const mem = getDataViewMemory0();
        const result = [];
        for (let i = ptr; i < ptr + 4 * len; i += 4) {
            result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
        }
        wasm.__externref_drop_slice(ptr, len);
        return result;
    }

    function getArrayU8FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
    }

    function getClampedArrayU8FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
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
        return decodeText(ptr >>> 0, len);
    }

    let cachedUint8ArrayMemory0 = null;
    function getUint8ArrayMemory0() {
        if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
            cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8ArrayMemory0;
    }

    let cachedUint8ClampedArrayMemory0 = null;
    function getUint8ClampedArrayMemory0() {
        if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
            cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
        }
        return cachedUint8ClampedArrayMemory0;
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            const idx = addToExternrefTable0(e);
            wasm.__wbindgen_exn_store(idx);
        }
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    function makeClosure(arg0, arg1, f) {
        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (...args) => {

            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            try {
                return f(state.a, state.b, ...args);
            } finally {
                real._wbg_cb_unref();
            }
        };
        real._wbg_cb_unref = () => {
            if (--state.cnt === 0) {
                wasm.__wbindgen_destroy_closure(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state);
            }
        };
        CLOSURE_DTORS.register(real, state, state);
        return real;
    }

    function makeMutClosure(arg0, arg1, f) {
        const state = { a: arg0, b: arg1, cnt: 1 };
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
                wasm.__wbindgen_destroy_closure(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state);
            }
        };
        CLOSURE_DTORS.register(real, state, state);
        return real;
    }

    function passArrayJsValueToWasm0(array, malloc) {
        const ptr = malloc(array.length * 4, 4) >>> 0;
        for (let i = 0; i < array.length; i++) {
            const add = addToExternrefTable0(array[i]);
            getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
        }
        WASM_VECTOR_LEN = array.length;
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

    function takeFromExternrefTable0(idx) {
        const value = wasm.__wbindgen_externrefs.get(idx);
        wasm.__externref_table_dealloc(idx);
        return value;
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
    function __wbg_finalize_init(instance, module) {
        wasm = instance.exports;
        cachedDataViewMemory0 = null;
        cachedFloat32ArrayMemory0 = null;
        cachedUint8ArrayMemory0 = null;
        cachedUint8ClampedArrayMemory0 = null;
        wasm.__wbindgen_start();
        return wasm;
    }

    async function __wbg_load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {
            if (!module.ok) {
                throw new Error(`failed to fetch Wasm: ${module.status} ${module.statusText} fetching '${module.url}'`);
            }

            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);
                } catch (e) {
                    const validResponse = expectedResponseType(module.type);

                    if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else { throw e; }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);
        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };
            } else {
                return instance;
            }
        }

        function expectedResponseType(type) {
            switch (type) {
                case 'basic': case 'cors': case 'default': return true;
            }
            return false;
        }
    }

    async function __wbg_init(module_or_path) {
        if (wasm !== undefined) return wasm;


        if (module_or_path !== undefined) {
            if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
                ({module_or_path} = module_or_path);
            } else {
                console.warn('using deprecated parameters for the initialization function; pass a single object instead');
            }
        }

        if (module_or_path === undefined) {
            module_or_path = new URL('gba_web_frontend_bg.wasm', (_documentCurrentScript && _documentCurrentScript.tagName.toUpperCase() === 'SCRIPT' && _documentCurrentScript.src || new URL('bundle.js', document.baseURI).href));
        }
        const imports = __wbg_get_imports();

        if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
            module_or_path = fetch(module_or_path);
        }

        const { instance, module } = await __wbg_load(await module_or_path, imports);

        return __wbg_finalize_init(instance);
    }

    async function main() {
       await __wbg_init({ module_or_path: 'pkg/gba_web_frontend_bg.wasm' });
       run_app();
    }
    main();

})();
