<script setup lang="ts">
import {Ref, ref} from "vue";
import {Transaction} from "../../bitcoin.ts";
import P2shRedeemView from "./P2shRedeemView.vue";
import SigningViewModal from "./SigningViewModal.vue";
import {SigningResult, useWasm} from "../../lib.ts";
import {useMessage} from 'naive-ui';

const message = useMessage();

let model = defineModel('show');
type TabValue = 'Sign P2PKH' | 'Redeem P2SH';
let tabValue: Ref<TabValue> = ref('Sign P2PKH')

let props = defineProps<{
  tx: Transaction,
  index: number,
}>();

let emit = defineEmits(['result']);

let showModal = ref({
  getSignature: false,
});

let signingParams = ref({
  signature: '',
  publicKey: '',
});

function signingDoneClicked() {
  try {
    let scriptSig = useWasm().TxBuilder.create_p2wpkh_script_sig(
        signingParams.value.signature,
        signingParams.value.publicKey,
    );
    emit('result', scriptSig);
    model.value = false;
  } catch (e: any) {
    message.error(e.toString());
  }
}
</script>

<template>
  <SigningViewModal :index="props.index" :tx="props.tx"
                    v-model:show="showModal.getSignature"
                    @result="(r: SigningResult) => {
    signingParams.signature = r.signature;
    signingParams.publicKey = r.publicKey;
  }"/>

  <n-modal v-model:show="model"
  >
    <n-card
        style="width: 600px"
        title="Input Script"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <n-tabs id="n-tab" type="line" v-model:value="tabValue" animated>
        <n-tab-pane name="Sign P2PKH" class="pane" id="sign-p2pkh-pane">
          <div>
            <span>
              Signature
              <n-button text tag="a" type="primary" @click="showModal.getSignature = true">Sign</n-button>
            </span>
            <n-input type="textarea" placeholder="Signature Hex" v-model:value="signingParams.signature"/>
          </div>
          <div>
            <span>Public Key</span>
            <n-input type="textarea" placeholder="Public Key Hex" v-model:value="signingParams.publicKey"/>
          </div>
          <div>
            <n-space justify="center">
              <n-button @click="signingDoneClicked">Done</n-button>
            </n-space>
          </div>
        </n-tab-pane>
        <n-tab-pane name="Redeem P2SH">
          <P2shRedeemView @result="x => {emit('result', x); model = false}"/>
        </n-tab-pane>
      </n-tabs>
    </n-card>
  </n-modal>
</template>

<style scoped>
#sign-p2pkh-pane {
  display: flex;
  flex-direction: column;
  gap: .5em;
}
</style>
