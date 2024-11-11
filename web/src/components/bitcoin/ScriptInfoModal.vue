<script setup lang="ts">
import ScriptInfoModalView from "./ScriptInfoModalView.vue";

let model = defineModel('show');
let props = defineProps<{
  scriptHex: string,
}>();
</script>

<template>
  <n-modal v-model:show="model"
  >
    <n-card
        style="width: 600px"
        title="Script Info"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <ScriptInfoModalView :script-hex="props.scriptHex"/>
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
