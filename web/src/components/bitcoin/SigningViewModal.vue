<script setup lang="ts">
import SigningView from "./SigningView.vue";
import {Transaction} from "../../bitcoin.ts";
import {SigningResult} from "../../lib.ts";

let model = defineModel('show');

let props = defineProps<{
  index: number,
  tx: Transaction,
}>();

let emit = defineEmits<{
  result: [value: SigningResult],
}>();
</script>

<template>
  <n-modal v-model:show="model"
  >
    <n-card
        style="width: 600px"
        title="Signing"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <SigningView :tx="props.tx" :index="props.index"
                   @result="r => {emit('result', r); model = false}"
      />
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
