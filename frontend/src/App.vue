<script setup lang="ts">
import {computed, ref} from "vue";
import {useRoute} from "vue-router";
import {useUserStore} from "@/stores/user";
import {logout} from "@/scripts/utils";
import ColorSwitcher from "@/views/components/ColorSwitcher.vue";

const route = useRoute();
const userStore = useUserStore();

const showLayout = computed(() => {
  return !route.meta.layout || route.meta.layout !== 'empty';
});

const userNav = ref([
  {
    label: userStore.user?.name,
    items: [
      {
        label: 'Profile',
        icon: 'pi pi-user-edit',
        route: "/myprofile"
      },
      {
        label: 'Log Out',
        icon: 'pi pi-sign-out',
        command: () => {
          logout();
        }
      }
    ]
  }
]);

const headerNav = [
  {name: 'Projects', href: '/projects'},
  {name: 'Team', href: '/team'},
  {name: 'Partners', href: '/partners'},
  {name: 'Mom', href: '/test'},
]

const footerNav = {
  main: [
    {name: 'About', href: '/about'},
    {name: 'Accessibility', href: '/accessibility'},
    {name: 'Partners', href: '/partners'},
  ],
  social: [
    {
      name: 'X (formerly Twitter)',
      href: '#',
      icon: "pi pi-twitter",
    },
    {
      name: 'GitHub',
      href: '#',
      icon: "pi pi-github",
    },
    {
      name: 'YouTube',
      href: 'https://www.youtube.com/@intelligencemodding4093',
      icon: "pi pi-youtube",
    },
  ],
}

</script>

<template>
  <Toast></Toast>
  <div v-if="showLayout">
    <div class="flex flex-col min-h-screen">
    <div class="sticky top-0 bg-slate-800 z-20">
      <nav class="mx-auto flex max-w-7xl items-center justify-between p-3 lg:px-8" aria-label="Global">
        <div class="flex lg:flex-1">
          <router-link to="/" class="-m-1 p-1 ">
            <span class="sr-only">Intelligence Modding</span>
            <img class="hover:shadow-lg hover:shadow-sky-700 bg-blend-multiply"
                 src="/BH_INH_Logo_2.png"
                 alt=""
                 width=64
                 height=64
            />
          </router-link>
        </div>
        <div class="flex lg:hidden">
          <button
              type="button"
              class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700"
          >
            <span class="sr-only">Open main menu</span>
          </button>
        </div>
        <div class="hidden lg:flex lg:gap-x-12">
          <router-link v-for="item in headerNav" :to=item.href
                       class="text-sm hover:text-sky-400 font-semibold leading-6 text-white">
            {{ item.name }}
          </router-link>
        </div>
        <div class="hidden lg:flex lg:flex-1 lg:justify-end">
          <div class="px-3">
            <ColorSwitcher/>
          </div>
          <router-link to="/login" class="text-sm font-semibold leading-6 text-white">
            Log in <span aria-hidden="true">&rarr;</span>
          </router-link>
        </div>
      </nav>
    </div>
    <div class="flex-grow">
      <router-view></router-view>
    </div>
    <footer class="bg-slate-800 border-t border-slate-600">
      <div class="mx-auto max-w-7xl overflow-hidden px-6 py-24 sm:py-8 lg:px-8">
        <nav class="mb-6 columns-2 sm:flex sm:justify-center sm:space-x-12" aria-label="Footer">
          <router-link v-for="item in footerNav.main" :to=item.href
                       class="text-sm leading-6 text-gray-600 hover:text-gray-900">
            {{ item.name }}
          </router-link>
        </nav>
        <div class="mt-6 flex justify-center space-x-10">
          <router-link v-for="item in footerNav.social" key={item.name} :to=item.href
                       class="text-gray-400 hover:text-gray-500">
            <span class="sr-only">{item.name}</span>
            <p :class="`h-6 w-6 ${item.icon}`" aria-hidden="true"/>
          </router-link>
        </div>
        <p class="mt-6 text-center text-xs leading-5 text-gray-500">
          &copy; 2024 Intelligence Modding, Inc. All rights reserved.
        </p>
      </div>
    </footer>
    </div>
  </div>
  <div v-else>
    <router-view></router-view>
  </div>
</template>

<style lang="scss">

</style>