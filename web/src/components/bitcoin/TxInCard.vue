<script setup lang="ts">
import Frame from "./Frame.vue";
import {Transaction, TxIn} from "../../bitcoin.ts";
import {safeParseInt} from "../../lib.ts";
import SelectableIcon from "./SelectableIcon.vue";
import {CreateOutline as CreateIcon, InformationOutline as InfoIcon} from '@vicons/ionicons5';
import {ref} from "vue";
import ScriptAsmModal from "./ScriptAsmModal.vue";
import TxiScriptSigInputModal from "./TxiScriptSigInputModal.vue";

let valueModel = defineModel<TxIn>('value');
let emit = defineEmits(['close']);
let props = defineProps<{
  index: number,
  tx: Transaction,
}>();

let showModal = ref({
  scriptSigInfo: false,
  sequence: false,
  inputScriptSig: false,
});

function enterSequence(value: number) {
  showModal.value.sequence = false;
  valueModel.value.sequence = value;
}
</script>

<template>
  <TxiScriptSigInputModal
      :tx="props.tx" :index="props.index"
      v-model:show="showModal.inputScriptSig"
      @result="x => valueModel.scriptSig = x"
  />

  <n-modal v-model:show="showModal.sequence"
  >
    <n-card
        style="width: 600px"
        title="Preset Sequences"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <n-space justify="center">
        <n-button-group vertical>
          <n-button @click="enterSequence(0xFFFFFFFF)">MAX (0xFFFFFFFF)</n-button>
          <n-button @click="enterSequence(0x00000000)">ZERO (0x00000000)</n-button>
          <n-button @click="enterSequence(0xFFFFFFFE)">ENABLE_LOCKTIME_NO_RBF (0xFFFFFFFE)</n-button>
          <n-button @click="enterSequence(0xFFFFFFFD)">ENABLE_RBF_NO_LOCKTIME (0xFFFFFFFD)</n-button>
          <n-button @click="enterSequence(0xFFFFFFFD)">ENABLE_LOCKTIME_AND_RBF (0xFFFFFFFD)</n-button>
        </n-button-group>
      </n-space>
    </n-card>
  </n-modal>

  <ScriptAsmModal :script-hex="valueModel.scriptSig" v-model:show="showModal.scriptSigInfo"/>

  <Frame :title="`TxIn #${props.index}`" title-adjust="left" title-size="normal" show-close-icon @close="emit('close')">
    <div class="cell">
      <span class="label">Outpoint</span>
      <div id="outpoint-line">
        <n-input size="small" class="input1" placeholder="TxId" v-model:value="valueModel.outpointTxId"/>
        <n-input size="small"
                 placeholder="Idx"
                 :allow-input="x => /^\d*$/.test(x)" class="input2"
                 :value="`${valueModel.outpointIndex}`"
                 @update:value="x => valueModel.outpointIndex = safeParseInt(x)"/>
      </div>
    </div>
    <div class="cell">
      <div class="label">
        Sequence
        <SelectableIcon @click="showModal.sequence = true">
          <CreateIcon/>
        </SelectableIcon>
      </div>
      <n-input size="small"
               placeholder="Sequence value"
               :value="`${valueModel.sequence}`"
               @update:value="x => valueModel.sequence = safeParseInt(x)"
      />
    </div>
    <div class="cell">
      <div class="label">
        ScriptSig
        <SelectableIcon @click="showModal.inputScriptSig = true" style="margin-left: .25em">
          <CreateIcon/>
        </SelectableIcon>
        <SelectableIcon @click="showModal.scriptSigInfo = true"
        >
          <InfoIcon/>
        </SelectableIcon>
      </div>
      <n-input size="small" type="textarea" placeholder="Script hex"
               v-model:value="valueModel.scriptSig"/>
    </div>
    <div class="cell">
      <span class="label">Witness</span>
      <n-input size="small" type="textarea"
               placeholder="Witness hex arrays separated by commas"
               v-model:value="valueModel.witness"/>
    </div>
  </Frame>
</template>

<style scoped>
#outpoint-line {
  display: flex;

  .input2 {
    max-width: 5em;
  }
}

.cell {
  margin: .25em 0;
}

.label {
  display: inline-flex;
  align-items: center;
  gap: .25em;
}
</style>
