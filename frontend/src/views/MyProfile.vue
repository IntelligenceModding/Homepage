<script setup lang="ts">

import {ref} from "vue";
import type {HintedString} from "@primevue/core";
import {logout} from "@/scripts/utils";
import ProfilePicture from "@/views/components/ProfilePicture.vue";
import {useUserStore} from "@/stores/user";
import type {User} from "@/scripts/definitions";
import {patchUser} from "@/scripts/data";
import {useToast} from "primevue/usetoast";

const userStore = useUserStore();
const toast = useToast();

const name = ref(userStore.user?.name);
const email = ref(userStore.user?.email);
const firstname = ref(userStore.user?.firstname);
const lastname = ref(userStore.user?.lastname);

const submitted = ref(false);
const loading = ref(false);

const currentMenu = ref<HintedString<'settings' | 'appearance' | 'tokens' | 'password' | 'delete'>>("settings");

async function saveSettings() {
  submitted.value = true;
  if (!name.value || !email.value) {
    return;
  }
  loading.value = true;

  let newUser: User = {
    id: userStore.user?.id,
    firstname: firstname.value,
    lastname: lastname.value,
    name: name.value,
    email: email.value
  }

  userStore.setUser(await patchUser(newUser, userStore.token), userStore.token);
  toast.add({severity: 'success', summary: 'Successful', detail: 'Edited', life: 3000});
  loading.value = false;
  submitted.value = false;
}

const items = ref([
  {
    label: 'Profile',
    items: [
      {
        label: 'Settings',
        icon: 'pi pi-cog',
        command: () => {
          currentMenu.value = "settings";
        }
      },
      {
        label: 'Appearance',
        icon: 'pi pi-palette',
        command: () => {
          currentMenu.value = "appearance";
        }
      },
    ],
  },
  {
    label: 'Security',
    items: [
      {
        label: 'Tokens',
        icon: 'pi pi-key',
        command: () => {
          currentMenu.value = "tokens";
        }
      },
      {
        label: 'Change Password',
        icon: 'pi pi-lock',
        command: () => {
          currentMenu.value = "password";
        }
      },
      {
        label: 'Delete Account',
        icon: 'pi pi-trash',
        command: () => {
          currentMenu.value = "delete";
        }
      }
    ]
  },
  {
    items: [
      {
        label: 'Logout',
        icon: 'pi pi-sign-out',
        command: async () => {
          await logout();
        }
      }
    ]
  }
]);

</script>

<template>
  <h3 class="font-semibold text-3xl mb-2">My Profile</h3>
  <div class="rounded p-2 w-[60%] flex-col justify-center items-center">
    <div class="flex flex-row">
      <div class="border-r-2 border-surface-700 mr-2">
        <Menu :model="items" class="w-48 !border-0 mr-2"/>
      </div>
      <div v-if="currentMenu == 'settings'" class="m-4 w-full">
        <h2 class="font-semibold text-2xl mb-2">Settings</h2>
        <div class="border-b-2 border-surface-600 my-2 mx-1"/>
        <div>
          <div class="grid grid-cols-12 gap-4">
            <div class="col-span-4 row-span-2 flex items-center">
              <div class="border-dashed border border-surface-700 rounded-full bg-surface-900 mr-8">
                <ProfilePicture class="w-28 h-28 rounded-full"/>
              </div>
              <div class="flex flex-col m-2">
                <div class="border border-primary-600 rounded-md hover:border-primary-400">
                  <input id="upload" type="file" class="hidden" accept="image/*"/>
                  <label for="upload" class="cursor-pointer">
                    <div class="font-semibold text-primary-400 mr-2"><p class="pi pi-upload text-primary-400 m-3"></p>
                      Change Avatar
                    </div>
                  </label>
                </div>
                <small class="mt-1">JPG, GIF or PNG. 4MB max.</small>
              </div>
            </div>
            <div class="col-span-4">
              <FloatLabel variant="in">
                <InputText id="firstname" class="w-full bg-green-300" v-model.trim="firstname"/>
                <label for="firstname">Firstname</label>
              </FloatLabel>
            </div>
            <div class="col-span-4">
              <FloatLabel variant="in">
                <InputText id="lastname" class="w-full" v-model.trim="lastname"/>
                <label for="lastname">Lastname</label>
              </FloatLabel>
            </div>
            <div class="col-span-4">
              <FloatLabel variant="in">
                <InputText id="username" class="w-full" :invalid="submitted && !name" v-model.trim="name"/>
                <label for="username">Username</label>
              </FloatLabel>
            </div>
            <div class="col-span-4">
              <FloatLabel variant="in">
                <InputText id="email" class="w-full" :invalid="submitted && !email" v-model.trim="email" type="email"/>
                <label for="email">E-Mail</label>
              </FloatLabel>
            </div>
          </div>
        </div>
        <div class="border-b-2 border-surface-600 my-2 mx-1"/>
        <Button v-if="!loading" class="text-xl font-semibold mt-6" severity="success" icon="pi pi-save"
                label="Save Changes" size="large" @click="saveSettings"/>
        <Button v-if="loading" class="text-xl font-semibold mt-6" severity="success" icon="pi pi-spin pi-spinner"
                label="Save Changes" size="large"/>
      </div>
      <div v-if="currentMenu == 'appearance'" class="p-4">
        <h2 class="font-semibold text-2xl mb-2">Appearance</h2>
      </div>
      <div v-if="currentMenu == 'tokens'" class="p-4">
        <h2 class="font-semibold text-2xl mb-2">Tokens</h2>
      </div>
      <div v-if="currentMenu == 'password'" class="p-4">
        <h2 class="font-semibold text-2xl mb-2">Change Password</h2>
      </div>
      <div v-if="currentMenu == 'delete'" class="p-4">
        <h2 class="font-semibold text-2xl mb-2">Delete Account</h2>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>