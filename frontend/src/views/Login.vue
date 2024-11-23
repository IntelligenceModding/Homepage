<script setup lang="ts">

import {ref} from "vue";
import {auth} from "@/scripts/data";
import {useRouter} from "vue-router";
import InputText from 'primevue/inputtext';

const router = useRouter()
const password = ref("")
const email = ref("")

let invalid = false;

async function login() {
  const valid = await auth(email.value, password.value);

  if (valid) {
    await router.push({name: "myprofile"})
  } else {
    invalid = true;
    password.value = "";
  }
}

</script>

<template>
  <div
      class="px-6 py-20 md:px-12 lg:px-20 flex h-screen w-screen items-center justify-center bg-[linear-gradient(-225deg,var(--p-primary-500),var(--p-primary-700)_48%,var(--p-primary-800))] dark:bg-[linear-gradient(-225deg,var(--p-primary-400),var(--p-primary-600)_48%,var(--p-primary-800))]"
  >
    <div class="p-12 shadow-lg text-center lg:w-[30rem] backdrop-blur-md rounded-xl bg-[rgba(255,255,255,0.1)]">
      <div class="text-4xl font-medium mb-12 text-primary-contrast">Intelligence Modding</div>
      <InputText
          type="email"
          class="!appearance-none placeholder:!text-primary-contrast/40 !border-0 !p-4 !w-full !outline-0 !text-xl !block !mb-6 !bg-white/10 !text-primary-contrast/70 !rounded-full"
          placeholder="Username"
          v-model="email"
      />
      <InputText
          type="password"
          class="placeholder:!text-primary-contrast/40 !border-0 !p-4 !w-full !outline-0 !text-xl !mb-6 !bg-white/10 !text-primary-contrast/70 !rounded-full"
          placeholder="Password"
          v-model="password"
      />
      <button
          type="button"
          class="max-w-40 w-full rounded-full appearance-none border-0 p-4 outline-0 text-xl mb-6 font-medium bg-white/30 hover:bg-white/40 active:bg-white/20 text-primary-contrast/80 cursor-pointer transition-colors duration-150"
          @click="login">

        Sign In
      </button>
      <div v-if="invalid" class="flex justify-center">
        <p class="text-red-500 font-bold">Could not find user</p>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>