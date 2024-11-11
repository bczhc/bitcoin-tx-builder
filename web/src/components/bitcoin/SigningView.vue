<script setup lang="ts">
import {SIGHASH_OPTIONS, SigHashKey, SignatureParams, SigningType, Transaction} from "../../bitcoin.ts";
import {ref} from "vue";
import {CreateOutline as CreateIcon, InformationOutline as InfoIcon} from '@vicons/ionicons5';
import SelectableIcon from "./SelectableIcon.vue";
import {SigningResult} from "../../lib.ts";
import ScriptInfoModal from "./ScriptInfoModal.vue";
import TxoScriptPubKeyInputModal from "./TxoScriptPubKeyInputModal.vue";
import ScriptInput from "./ScriptInput.vue";
import SecretKeyFromWifModal from "./SecretKeyFromWifModal.vue";

let emit = defineEmits<{
  result: [value: SigningResult],
}>();

let props = defineProps<{
  tx: Transaction,
  index: number,
}>();

let txoScriptPubKey = ref('');

let signingTypeRadioOptions: { value: SigningType, label: string }[] = [
  {value: 'legacy', label: 'Legacy'},
  {value: 'p2wpkh', label: 'P2WPKH'},
  {value: 'p2wsh', label: 'P2WSH'},
];

let showModal = ref({
  secretKeyFromWif: false,
  txoScriptPubKeyFromAddress: false,
  txoScriptPubKeyInfo: false,
});

let model = defineModel<SignatureParams>('value');
</script>

<template>
  <SecretKeyFromWifModal
      v-model:show="showModal.secretKeyFromWif"
      @ec-result="x => model.secretKey = x"
  />

  <ScriptInfoModal :script-hex="txoScriptPubKey" v-model:show="showModal.txoScriptPubKeyInfo"/>
  <TxoScriptPubKeyInputModal v-model:show="showModal.txoScriptPubKeyFromAddress"
                             @result="x => txoScriptPubKey = x"/>


  <n-h5 style="margin: 0">Signing for TxIn #{{ props.index }}</n-h5>
  <n-radio-group v-model:value="model.signingType"
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
      <n-input type="textarea" v-model:value="model.txoScriptPubKey"
               placeholder="ScriptPubKey of the TxO this input uses"/>
    </div>
    <div>
      <span>SigHash Type</span>
      <n-select :options="SIGHASH_OPTIONS" v-model:value="model.sighashTypeName"/>
    </div>
    <div>
      <div style="display: inline-flex; align-items: center; gap: .5em">
        Secret Key
        <SelectableIcon @click="showModal.secretKeyFromWif = true">
          <CreateIcon/>
        </SelectableIcon>
      </div>
      <n-input type="textarea" v-model:value="model.secretKey" placeholder="32-byte Hex"/>
    </div>
    <div style="display: inline-flex; align-items: center; gap: .5em;" v-if="model.signingType != 'legacy'">
      <span>Amount</span>
      <n-input placeholder="Sats" autosize style="min-width: 10em"
               v-model:value="model.amount"
      />
    </div>
    <div v-if="model.signingType == 'p2wsh'">
      <ScriptInput title="Witness Script" show-info-button v-model:value="model.witnessScript"/>
    </div>
  </div>
</template>

<style scoped>
#form-list {
  display: flex;
  flex-direction: column;
  gap: .5em;
}
</style>
