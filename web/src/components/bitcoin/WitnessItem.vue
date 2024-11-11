<script setup lang="ts">

import {ref} from "vue";

let props = defineProps<{
  index: number,
}>();

let emit = defineEmits(['remove']);

let model = defineModel('value');

type DropdownKey = 'remove';

let dropdownOptions: { label: string, value: DropdownKey }[] = [
  {label: 'Remove', key: 'remove'}
];

function onDropdownSelected(key: DropdownKey) {
  switch (key) {
    case "remove":
      emit('remove');
      break;
  }
}
</script>

<template>
  <div id="root">
    <div id="line">
      <n-dropdown :options="dropdownOptions" trigger="hover" @select="onDropdownSelected">
        <span class="label">[{{ index }}]</span>
      </n-dropdown>
      <n-input placeholder="Item hex" type="textarea" v-model:value="model"/>
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
</style>
