<script setup lang="ts">
import {Ref, ref} from "vue";
import SelectableIcon from "./SelectableIcon.vue";
import {InformationOutline as InfoIcon} from '@vicons/ionicons5';
import ScriptInfoModal from "./ScriptInfoModal.vue";
import {useWasm} from "../../lib.ts";
import {useMessage} from 'naive-ui';
import {GLOBAL_NETWORK} from "../../bitcoin.ts";

let message = useMessage();
let model = defineModel('show');
let emit = defineEmits(['result']);
let wasm = useWasm();

let address = ref('');
let redeem = ref('');

let showModal = ref({
  scriptInfo: false,
});

type TabValue = 'Address' | 'Create P2SH' | 'Create P2WSH';
let tabValue: Ref<TabValue> = ref('Address');

function doneClick() {
  try {
    let r: string;
    switch (tabValue.value) {
      case "Address":
        r = wasm.TxBuilder.address_to_script_pub_key(address.value, GLOBAL_NETWORK);
        emit('result', r);
        break;
      case "Create P2SH":
        r = wasm.TxBuilder.generate_p2sh_pub_key(redeem.value);
        emit('result', r);
        break;
      case "Create P2WSH":
        r = wasm.TxBuilder.create_p2wsh_script_pubkey(redeem.value);
        emit('result', r);
        break;
    }
    model.value = false;
  } catch (e: any) {
    message.error(e.toString());
  }
}
</script>

<template>
  <ScriptInfoModal :script-hex="redeem" v-model:show="showModal.scriptInfo"/>

  <n-modal v-model:show="model"
  >
    <n-card
        style="width: 600px"
        title="Script Helper"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <n-tabs type="line" animated v-model:value="tabValue">
        <n-tab-pane name="Address">
          <n-input v-model:value="address" placeholder="Address"/>
        </n-tab-pane>
        <n-tab-pane
            name="Create P2SH"
            style="display: flex; align-items: center"
        >
          <n-input v-model:value="redeem" placeholder="Redeem Script Hex" type="textarea"/>
          <SelectableIcon @click="showModal.scriptInfo = true">
            <InfoIcon/>
          </SelectableIcon>
        </n-tab-pane>
        <n-tab-pane
            name="Create P2WSH"
            style="display: flex; align-items: center"
        >
          <n-input v-model:value="redeem" placeholder="Witness Script Hex" type="textarea"/>
          <SelectableIcon @click="showModal.scriptInfo = true">
            <InfoIcon/>
          </SelectableIcon>
        </n-tab-pane>
      </n-tabs>
      <n-space justify="end" style="margin-top: .5em">
        <n-button type="primary" @click="doneClick">Done</n-button>
      </n-space>
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
