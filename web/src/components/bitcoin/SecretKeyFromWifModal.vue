<script setup lang="ts">
import {ref} from "vue";
import {useWasm} from "../../lib.ts";
import {useMessage} from 'naive-ui';

let model = defineModel('show');

let wif = ref('');

let emit = defineEmits(['ecResult', 'pukResult']);
let wasm = useWasm();
let message = useMessage();

function doneClick() {
  try {
    let r = wasm.TxBuilder.wif_to_ec_hex(wif.value);
    emit('ecResult', r);
    r = wasm.TxBuilder.wif_to_public_key(wif.value);
    emit('pukResult', r);
    model.value = false;
  } catch (e: any) {
    message.error(e.toString());
  }
}
</script>

<template>
  <n-modal v-model:show="model"
  >
    <n-card
        style="width: 600px"
        title="From WIF"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <div id="form">
        <n-input placeholder="Wallet Import Format" v-model:value="wif"/>
        <n-space justify="center">
          <n-button @click="doneClick">Done</n-button>
        </n-space>
      </div>
    </n-card>
  </n-modal>
</template>

<style scoped>
#form {
  display: flex;
  flex-direction: column;
  gap: .5em;
}
</style>
