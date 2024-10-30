<script setup lang="ts">
</script>

<template>
  <n-message-provider>
    <n-modal-provider>
      <n-dialog-provider>
        <RouterView/>
      </n-dialog-provider>
    </n-modal-provider>
  </n-message-provider>
</template>

<style scoped>

</style>
