<script setup lang="ts">

import UsersTable from "@/views/components/UsersTable.vue";
</script>

<template>
  <h3 class="font-semibold text-3xl">Users</h3>
  <p class="my-2 text-gray-200">Users registered in this intelligence instance</p>
  <Suspense>
    <UsersTable/>
  </Suspense>
</template>

<style scoped>

</style>