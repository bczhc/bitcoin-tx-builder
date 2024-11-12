<script setup lang="ts">

import {ref} from "vue";
import {EllipsisHorizontal as EllipsisIcon} from "@vicons/ionicons5";
import SelectableIcon from "./SelectableIcon.vue";
import ScriptInfoModal from "./ScriptInfoModal.vue";
import WitnessItemInsertModal from "./WitnessItemInsertModal.vue";
import {Transaction} from "../../bitcoin.ts";

let props = defineProps<{
  txIndex: number,
  itemIndex: number,
  tx: Transaction,
}>();

let emit = defineEmits(['remove']);

let model = defineModel<string>('value');

type DropdownKey = 'remove' | 'script info' | 'insert';

let dropdownOptions: { label: string, key: DropdownKey }[] = [
  {label: 'Insert', key: 'insert'},
  {label: 'Script Info', key: 'script info'},
  {label: 'Remove', key: 'remove'}
];

let showModal = ref({
  scriptInfo: false,
  insert: false,
});

function onDropdownSelected(key: DropdownKey) {
  switch (key) {
    case "remove":
      emit('remove');
      break;
    case 'script info':
      showModal.value.scriptInfo = true;
      break;
    case "insert":
      showModal.value.insert = true;
      break;
  }
}
</script>

<template>
  <ScriptInfoModal :script-hex="model" v-model:show="showModal.scriptInfo"/>
  <WitnessItemInsertModal
      v-model:show="showModal.insert"
      :tx-index="props.txIndex"
      :tx="props.tx"
      @result="r => model = r"
  />

  <div id="root">
    <div id="line">
      <n-input placeholder="Item hex" type="textarea" v-model:value="model"/>
      <div style="display: inline-flex; flex-direction: column">
        <span class="label">[{{ props.itemIndex }}]</span>
        <n-dropdown :options="dropdownOptions"
                    @select="onDropdownSelected"
                    trigger="hover"
        >
          <SelectableIcon>
            <EllipsisIcon class="icon"/>
          </SelectableIcon>
        </n-dropdown>
      </div>
    </div>
  </div>
</template>

<style scoped>
#line {
  display: flex;
  align-items: center;
  gap: .5em;
}

.label {
  cursor: pointer;
}

.icon {
  opacity: .5;
}
</style>
