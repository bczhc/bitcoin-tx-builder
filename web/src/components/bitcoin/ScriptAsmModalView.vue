<script setup lang="ts">
import {stringifyFallible, useWasm} from "../../lib.ts";

const wasm = useWasm();

let props = defineProps<{
  scriptHex: string
}>();
</script>

<template>
  <div style="word-wrap: anywhere; font-family: monospace">
    {{
      stringifyFallible(() => {
        return wasm.TxBuilder.script_to_asm(props.scriptHex);
      })
    }}
  </div>
</template>

<style scoped>

</style>
