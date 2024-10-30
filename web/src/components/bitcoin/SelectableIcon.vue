<script setup lang="ts">

</script>

<template>
  <n-icon size="1.3em" class="selectable-icon">
    <slot/>
  </n-icon>
</template>

<style scoped>
.selectable-icon {
  transition: ease-in-out .2s;
  cursor: pointer;
}

.selectable-icon:hover {
  background-color: lightgray;
  transition: ease-in-out .2s;
}
</style>
