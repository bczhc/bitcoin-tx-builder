<script setup lang="ts">
import {ref} from "vue";
import {useWasm} from "../../lib.ts";
import {GLOBAL_NETWORK} from "../../bitcoin.ts";
import {useMessage} from 'naive-ui';

let emit = defineEmits(['result', 'done']);

let address = ref('');
const wasm = useWasm();
const message = useMessage();

function doneClick() {
  try {
    let r = wasm.TxBuilder.address_to_script_pub_key(address.value, GLOBAL_NETWORK);
    emit('result', r);
    emit('done');
  } catch (e: any) {
    message.error(e.toString());
  }
}
</script>

<template>
  <n-input placeholder="Address" v-model:value="address"/>
  <n-space justify="end" style="margin-top: .5em">
    <n-button type="primary" @click="doneClick">Done</n-button>
  </n-space>
</template>

<style scoped>

</style>
