<script setup lang="ts">
import {ref} from "vue";
import {useWasm} from "../../lib.ts";
import {useMessage} from 'naive-ui';

let message = useMessage();

let model = defineModel<boolean>('show');

let hex = ref('');
let emit = defineEmits<{
  result: [value: string],
}>();

function onClick() {
  try {
    let txJson = useWasm().TxBuilder.tx_hex_to_json(hex.value);
    emit('result', txJson);
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
        title="Import Transaction"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <n-input type="textarea" placeholder="Raw Transaction Hex" v-model:value="hex"/>
      <n-button type="primary" @click="onClick" style="margin-top: 1em">Done</n-button>
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
