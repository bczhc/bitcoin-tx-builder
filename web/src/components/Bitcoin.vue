<script setup lang="ts">
import {computed, ref} from "vue";
import {stringifyFallible, useWasm} from "../lib.ts";

const wasm = useWasm();

let scriptHexInput = ref('');
let scriptAsmOutput = computed(() => {
  return stringifyFallible(() => wasm.Bitcoin.parse_script_hex(scriptHexInput.value))
});
let digestType = ref('hash160');
let base58CheckInput = ref('');
let base58CheckOutput = computed(() => {
  return stringifyFallible(() => wasm.Bitcoin.base58_check(base58CheckInput.value));
});
let digestInput = ref('');
let digestOutput = computed(() => stringifyFallible(() => {
  let data = wasm.Bitcoin.parse_hex_str(digestInput.value);
  return wasm.Bitcoin.digest(data, digestType.value);
}))

let digestTypeOptions = [
  {label: 'RIPEMD160', value: 'ripemd160'},
  {label: 'SHA1', value: 'sha1'},
  {label: 'SHA256', value: 'sha256'},
  {label: 'SHA256d', value: 'sha256d'},
  {label: 'HASH160', value: 'hash160'},
]
</script>

<template>
  <div id="root">
    script hex
    <n-input v-model:value="scriptHexInput"/>
    assembly output:
    <div class="auto-wrap">{{ scriptAsmOutput }}</div>
    <n-divider/>

    hex
    <n-input v-model:value="digestInput"/>
    <n-select :options="digestTypeOptions" placeholder="Digest Algorithm" v-model:value="digestType"/>
    digest:
    <div class="auto-wrap">{{ digestOutput }}</div>
    <n-divider/>

    hex
    <n-input v-model:value="base58CheckInput"/>
    base58Check:
    <div class="auto-wrap">{{ base58CheckOutput }}</div>
    <n-divider/>
  </div>
</template>

<style scoped>
#root {
  padding: .5em;
}

.auto-wrap {
  word-break: break-word;
  white-space: normal;
}
</style>
