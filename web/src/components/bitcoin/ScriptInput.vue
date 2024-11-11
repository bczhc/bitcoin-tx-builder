<script setup lang="ts">
import ScriptInfoModal from "./ScriptInfoModal.vue";
import {ref} from "vue";
import {InformationOutline as InfoIcon} from '@vicons/ionicons5';
import SelectableIcon from "./SelectableIcon.vue";

let valueModel = defineModel<string>('value');

let props = defineProps<{
  title: string,
  showInfoButton?: boolean,
}>();

let showModal = ref({
  scriptAsm: false,
});
</script>

<template>
  <ScriptInfoModal :script-hex="valueModel" v-model:show="showModal.scriptAsm"/>

  <div class="cell">
    <div class="label">
      {{ props.title }}
      <SelectableIcon @click="showModal.scriptAsm = true" v-if="props.showInfoButton">
        <InfoIcon/>
      </SelectableIcon>
    </div>
    <n-input type="textarea" v-model:value="valueModel" placeholder="Script Hex"/>
  </div>
</template>

<style scoped>
.label {
  display: flex;
  align-items: center;
  gap: .25em;
}
</style>
