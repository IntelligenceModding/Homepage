<template>
  <img
      class="m-1 object-cover"
      :src="currentPfpSource"
      @error="handleImageError"
      alt=""
  />
</template>

<script setup lang="ts">
import {ref} from 'vue';
import {useUserStore} from '@/stores/user';

const props = defineProps(['user']);
let user = props.user;

const currentPfpSource = ref('');
const userStore = useUserStore();

if (!user) {
  user = userStore.user?.id;
}

currentPfpSource.value =
    'http://127.0.0.1:6969/api/v1/users/' +
    user +
    '/image';

function handleImageError() {
  currentPfpSource.value = 'https://avatar.iran.liara.run/public';
}

</script>