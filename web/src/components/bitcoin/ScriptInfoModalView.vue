<script setup lang="ts">
import {useWasm} from "../../lib.ts";
import {computed, ComputedRef} from "vue";
import {ScriptInfo} from "../../bitcoin.ts";

const wasm = useWasm();

let props = defineProps<{
  scriptHex: string
}>();

let scriptInfo: ComputedRef<ScriptInfo | null> = computed(() => {
  try {
    let info = wasm.TxBuilder.script_info(props.scriptHex);
    return JSON.parse(info) as ScriptInfo;
  } catch (_: any) {
    return null;
  }
});

let parseError: ComputedRef<string> = computed(() => {
  try {
    wasm.TxBuilder.script_info(props.scriptHex);
    return '';
  } catch (e: any) {
    return e.toString();
  }
})
</script>

<template>
  <div id="root-error" v-if="scriptInfo === null">
    <span class="label" style="color: red">Error</span>
    <div class="content">
      {{ parseError }}
    </div>
  </div>
  <div id="root" v-if="scriptInfo">
    <div>
      <span class="label">Assembly</span>
      <div class="content">
        {{ scriptInfo!!.asm }}
      </div>
    </div>
    <div>
      <span class="label" v-if="scriptInfo!!.type">Type</span>
      <div class="content">
        {{ scriptInfo!!.type!! }}
      </div>
    </div>
    <div v-if="scriptInfo!!.opReturnData">
      <span class="label">OP_RETURN data</span>
      <div class="content">
        {{ scriptInfo!!.opReturnData!! }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.label {
  font-weight: bold;
}

#root, #root-error {
  word-wrap: anywhere;
  font-family: monospace;
  display: flex;
  flex-direction: column;
  gap: .5em;
}
</style>
