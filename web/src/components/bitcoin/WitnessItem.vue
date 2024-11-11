<script setup lang="ts">

import {ref} from "vue";
import {EllipsisHorizontal as EllipsisIcon} from "@vicons/ionicons5";
import SelectableIcon from "./SelectableIcon.vue";
import ScriptAsmModal from "./ScriptAsmModal.vue";

let props = defineProps<{
  index: number,
}>();

let emit = defineEmits(['remove']);

let model = defineModel('value');

type DropdownKey = 'remove' | 'script asm';

let dropdownOptions: { label: string, key: DropdownKey }[] = [
  {label: 'Script ASM', key: 'script asm'},
  {label: 'Remove', key: 'remove'}
];

let showModal = ref({
  scriptAsm: false,
});

function onDropdownSelected(key: DropdownKey) {
  switch (key) {
    case "remove":
      emit('remove');
      break;
    case 'script asm':
      showModal.value.scriptAsm = true;
      break;
  }
}
</script>

<template>
  <ScriptAsmModal :script-hex="model" v-model:show="showModal.scriptAsm"/>

  <div id="root">
    <div id="line">
      <n-input placeholder="Item hex" type="textarea" v-model:value="model"/>
      <div style="display: inline-flex; flex-direction: column">
        <span class="label">[{{ index }}]</span>
        <n-dropdown :options="dropdownOptions" trigger="hover"
                    @select="onDropdownSelected">
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
