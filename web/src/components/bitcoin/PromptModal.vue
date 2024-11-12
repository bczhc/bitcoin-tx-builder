<script setup lang="ts">

import {computed} from "vue";
import {useMessage} from 'naive-ui';

const message = useMessage();

let showModel = defineModel<boolean>('show');
let inputModel = defineModel<string>('input');

let props = defineProps<{
  title: string,
  placeholder?: string,
  inputSize?: string,
  check?: (input: string) => void,
}>();

let emit = defineEmits(['done']);

let inputSize = computed(() => props.inputSize || 'large');

function onDone() {
  try {
    let checkFn = props.check || ((x: string) => x);
    checkFn(inputModel.value);
    emit('done');
  } catch (e: any) {
    message.error(e.toString());
  }
}
</script>

<template>
  <n-modal v-model:show="showModel"
  >
    <n-card
        style="width: 600px"
        :title="props.title"
        :bordered="false"
        role="dialog"
        aria-modal="true"
    >
      <n-input type="textarea" v-model:value="inputModel" :placeholder="props.placeholder"
               :size="inputSize"
      />
      <n-space justify="center" style="margin-top: .5em">
        <n-button type="primary" @click="onDone">Done</n-button>
      </n-space>
    </n-card>
  </n-modal>
</template>

<style scoped>

</style>
