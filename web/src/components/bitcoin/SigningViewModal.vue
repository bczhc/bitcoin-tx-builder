<script setup lang="ts">
import SigningView from "./SigningView.vue";
import {SignatureParams, signForSignature, Transaction, validateTxSerialization} from "../../bitcoin.ts";
import {SigningResult} from "../../lib.ts";
import {ref} from "vue";
import {useMessage} from "naive-ui";

const message = useMessage();

let model = defineModel('show');

let props = defineProps<{
  index: number,
  tx: Transaction,
}>();

let emit = defineEmits<{
  result: [value: SigningResult],
}>();

let signingParams = ref<SignatureParams>({
  signingType: 'legacy',
  secretKey: '',
  sighashTypeName: 'All',
  witnessScript: '',
  amount: '',
  txoScriptPubKey: '',
});

function signClick() {
  try {
    validateTxSerialization(props.tx);
    let result = signForSignature(signingParams.value, props.tx, props.index);
    emit('result', result);
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
        title="Signing"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <SigningView :tx="props.tx" :index="props.index"
                   @result="r => {emit('result', r); model = false}"
                   v-model:value="signingParams"
      />
      <n-space justify="center">
        <n-button style="margin-top: .5em" type="primary" @click="signClick">Sign</n-button>
      </n-space>
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
