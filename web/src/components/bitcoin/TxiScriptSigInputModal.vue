<script setup lang="ts">
import {Ref, ref} from "vue";
import SigningView from "./SigningView.vue";
import {Transaction} from "../../bitcoin.ts";
import P2shRedeemView from "./P2shRedeemView.vue";

let model = defineModel('show');
type TabValue = 'P2SH' | 'Legacy Signing';
let tabValue: Ref<TabValue> = ref('P2SH')

let props = defineProps<{
  tx: Transaction,
  index: number,
}>();

let emit = defineEmits(['result']);
</script>

<template>
  <n-modal v-model:show="model"
  >
    <n-card
        style="width: 600px"
        title="Input Script"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <n-tabs id="n-tab" type="line" v-model:value="tabValue" animated>
        <n-tab-pane name="Legacy Signing" class="pane">
          <SigningView :tx="props.tx" :index="props.index"
                       @result="x => {emit('result', x); model = false}"/>
        </n-tab-pane>
        <n-tab-pane name="P2SH">
          <P2shRedeemView @result="x => {emit('result', x); model = false}"/>
        </n-tab-pane>
      </n-tabs>
    </n-card>
  </n-modal>
</template>

<style scoped>
</style>
