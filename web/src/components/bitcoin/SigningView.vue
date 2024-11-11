<script setup lang="ts">
import {Transaction} from "../../bitcoin.ts";
import {ref} from "vue";
import {CreateOutline as CreateIcon, InformationOutline as InfoIcon} from '@vicons/ionicons5';
import SelectableIcon from "./SelectableIcon.vue";
import SecretKeyFromWifModal from "./SecretKeyFromWifModal.vue";
import {safeParseInt, safeToBigInt, SigningResult, useWasm} from "../../lib.ts";
import {useMessage} from 'naive-ui';
import ScriptAsmModal from "./ScriptAsmModal.vue";
import TxoScriptPubKeyInputModal from "./TxoScriptPubKeyInputModal.vue";
import ScriptInput from "./ScriptInput.vue";

let message = useMessage();

let emit = defineEmits<{
  result: [value: SigningResult],
}>();

let props = defineProps<{
  tx: Transaction,
  index: number,
}>();

let txoScriptPubKey = ref('');

type SigHashKey = 'All' | 'None' | 'Single'
    | 'AllPlusAnyoneCanPay' | 'NonePlusAnyoneCanPay' | 'SinglePlusAnyoneCanPay';
let sigHashOptions: { label: string, value: SigHashKey, code: number }[] = [
  {label: 'All - 0x01', value: 'All', code: 0x01},
  {label: 'None - 0x02', value: 'None', code: 0x02},
  {label: 'Single - 0x03', value: 'Single', code: 0x03},
  {label: 'All+AnyoneCanPay - 0x81', value: 'AllPlusAnyoneCanPay', code: 0x81},
  {label: 'None+AnyoneCanPay - 0x82', value: 'NonePlusAnyoneCanPay', code: 0x82},
  {label: 'Single+AnyoneCanPay - 0x83', value: 'SinglePlusAnyoneCanPay', code: 0x83},
];

function sigHashCode(value: SigHashKey) {
  return sigHashOptions.find(x => x.value === value)!!.code;
}

type SigningType = 'legacy' | 'p2wpkh' | 'p2wsh';

let signingTypeRadioOptions: { value: SigningType, label: string }[] = [
  {value: 'legacy', label: 'Legacy'},
  {value: 'p2wpkh', label: 'P2WPKH'},
  {value: 'p2wsh', label: 'P2WSH'},
];

let signingTypeSelected = ref<SigningType>('legacy');

let sigHash = ref<SigHashKey>('All');
let secretKey = ref('');
let publicKey = ref('');

let witnessScript = ref('');
let commitmentAmount = ref('');

let showModal = ref({
  secretKeyFromWif: false,
  txoScriptPubKeyFromAddress: false,
  txoScriptPubKeyInfo: false,
});

function signClick() {
  try {
    let wasm = useWasm();
    let amount = safeToBigInt(commitmentAmount.value);
    let signature = wasm.TxBuilder.sign_tx(
        JSON.stringify(props.tx),
        props.index,
        txoScriptPubKey.value,
        sigHashCode(sigHash.value),
        secretKey.value,
        witnessScript.value,
        amount as bigint,
        signingTypeSelected.value,
    );
    let publicKey = wasm.TxBuilder.secret_to_public_key_compressed(secretKey.value);
    let result: SigningResult = {
      signature,
      publicKey,
    };
    emit('result', result);
  } catch (e: any) {
    console.log(e);
    message.error(e.toString());
  }
}
</script>

<template>
  <SecretKeyFromWifModal
      v-model:show="showModal.secretKeyFromWif"
      @ec-result="x => secretKey = x"
      @puk-result="x => publicKey = x"
  />
  <ScriptAsmModal :script-hex="txoScriptPubKey" v-model:show="showModal.txoScriptPubKeyInfo"/>
  <TxoScriptPubKeyInputModal v-model:show="showModal.txoScriptPubKeyFromAddress"
                             @result="x => txoScriptPubKey = x"/>


  <n-h5 style="margin: 0">Signing for TxIn #{{ props.index }}</n-h5>
  <n-radio-group v-model:value="signingTypeSelected"
                 style="margin: .5em 0">
    <n-radio v-for="x in signingTypeRadioOptions"
             :label="x.label"
             :value="x.value"
    />
  </n-radio-group>
  <div id="form-list">
    <div>
      <div style="display: inline-flex; align-items: center">
        <span style="margin-right: .5em">Transaction Output ScriptPubKey</span>
        <SelectableIcon @click="showModal.txoScriptPubKeyFromAddress = true">
          <CreateIcon/>
        </SelectableIcon>
        <SelectableIcon @click="showModal.txoScriptPubKeyInfo = true">
          <InfoIcon/>
        </SelectableIcon>
      </div>
      <n-input type="textarea" v-model:value="txoScriptPubKey"
               placeholder="ScriptPubKey of the TxO this input uses"/>
    </div>
    <div>
      <span>SigHash Type</span>
      <n-select :options="sigHashOptions" v-model:value="sigHash"/>
    </div>
    <div>
      <div style="display: inline-flex; align-items: center; gap: .5em">
        Secret Key
        <SelectableIcon @click="showModal.secretKeyFromWif = true">
          <CreateIcon/>
        </SelectableIcon>
      </div>
      <n-input type="textarea" v-model:value="secretKey" placeholder="32-byte Hex"/>
    </div>
    <div style="display: inline-flex; align-items: center; gap: .5em;" v-if="signingTypeSelected != 'legacy'">
      <span>Amount</span>
      <n-input placeholder="Sats" autosize style="min-width: 10em"
               v-model:value="commitmentAmount"
      />
    </div>
    <div v-if="signingTypeSelected == 'p2wsh'">
      <ScriptInput title="Witness Script" show-info-button v-model:value="witnessScript"/>
    </div>
    <n-space justify="center">
      <n-button type="primary" @click="signClick">Sign</n-button>
    </n-space>
  </div>
</template>

<style scoped>
#form-list {
  display: flex;
  flex-direction: column;
  gap: .5em;
}
</style>
