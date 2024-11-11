<script setup lang="ts">
import {Ref, ref} from "vue";
import SigningView from "./SigningView.vue";
import {SignatureParams, signForSignature, Transaction, validateTxSerialization} from "../../bitcoin.ts";
import {useMessage} from 'naive-ui';
import {useWasm} from "../../lib.ts";

const message = useMessage();

let model = defineModel('show');

type TabValue = 'Signature' | 'Public Key';
let tabValue = ref<TabValue>('Signature');

let props = defineProps<{
  tx: Transaction,
  index: number,
}>();

let signingParams: Ref<SignatureParams> = ref({
  witnessScript: '',
  amount: '',
  secretKey: '',
  txoScriptPubKey: '',
  signingType: 'legacy',
  sighashTypeName: 'All',
});
let wif = ref('');

// the witness stack item
let emit = defineEmits(['result']);

function doneClicked() {
  try {
    validateTxSerialization(props.tx);
    switch (tabValue.value) {
      case 'Signature':
        let result = signForSignature(signingParams.value, props.tx, props.index);
        emit('result', result.signature);
        break;
      case 'Public Key':
        let publicKey = useWasm().TxBuilder.wif_to_public_key(wif.value);
        emit('result', publicKey);
        break;
    }
    model.value =false;
  } catch (e: any) {
    console.log(e);
    message.error(e.toString());
  }
}
</script>

<template>
  <n-modal v-model:show="model"
  >
    <n-card
        style="width: 600px"
        title="Insert Item"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <n-tabs type="line" animated v-model:value="tabValue">
        <n-tab-pane name="Signature">
          <SigningView :tx="props.tx" :index="props.index"
                       v-model:value="signingParams"
          />
        </n-tab-pane>
        <n-tab-pane name="Public Key">
          <n-input placeholder="WIF" v-model:value="wif"/>
        </n-tab-pane>
      </n-tabs>
      <div id="bottom" style="margin-top: .5em">
        <n-space justify="center">
          <n-button type="primary" @click="doneClicked">Done</n-button>
        </n-space>
      </div>
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
