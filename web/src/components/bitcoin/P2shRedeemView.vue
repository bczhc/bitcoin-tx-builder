<script setup lang="ts">
import ScriptInput from "./ScriptInput.vue";
import {ref} from "vue";
import {useMessage} from 'naive-ui';
import {useWasm} from "../../lib.ts";

let scriptSigForRedeem = ref('');
let redeem = ref('');

let emit = defineEmits(['result']);
let message = useMessage();

function doneClick() {
  try {
    let s = useWasm().TxBuilder.script_sig_for_p2sh(
        scriptSigForRedeem.value,
        redeem.value
    );
    emit('result', s);
  } catch (e: any) {
    message.error(e.toString());
  }
}
</script>

<template>
  <div id="list">
    <ScriptInput title="ScriptSig for redeem-script"
                 v-model:value="scriptSigForRedeem"
                 show-info-button
    />
    <ScriptInput title="Redeem Script"
                 v-model:value="redeem"
                 show-info-button
    />
    <n-space justify="center">
      <n-button type="primary" @click="doneClick">Done</n-button>
    </n-space>
  </div>
</template>

<style scoped>
#list {
  display: flex;
  flex-direction: column;
  gap: .5em;
}
</style>
