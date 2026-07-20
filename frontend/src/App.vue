<script setup lang="ts">
import {computed, ref, onMounted} from "vue";
import {useRoute} from "vue-router";

const route = useRoute();

const showLayout = computed(() => {
  return !route.meta.layout || route.meta.layout !== 'empty';
});

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
// We use 150 columns * 19px = 2850px wide to guarantee it covers huge monitors.
const columns = 190;
const rows = 4;
const pixelSize = 19;

// Density of pixels for each row (Row 0 is heavy, Row 3 is very sparse)
const probabilities = [0.85, 0.55, 0.25, 0.05];

// Store the pixels in a ref
const ditherPixels = ref<DitherPixel[]>([]);

// Generate the random pattern only on the client side to prevent hydration mismatches
onMounted(() => {
  const pixels: DitherPixel[] = [];
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < columns; c++) {
      if (Math.random() < probabilities[r]) {
        pixels.push({
          x: c * pixelSize,
          y: r * pixelSize,
          key: `${c}-${r}`
        });
      }
    }
  }
  ditherPixels.value = pixels;
});
</script>

<template>
  <Toast></Toast>
  <div v-if="showLayout">
    <div class="flex flex-col min-h-screen">
      <header
        class="relative flex flex-col w-full bg-[url('/background.webp')] bg-cover bg-center aspect-[1440/775] min-h-[400px] max-h-[85vh] z-20">
        <nav class="mx-auto flex w-full max-w-7xl items-center justify-between p-3 lg:px-8"
             aria-label="Global">
          <div class="flex items-center gap-2 lg:flex-1">
            <router-link to="/" class="-m-1 p-1">
              <span class="sr-only">Intelligence Modding</span>
              <img class="hover:shadow-lg hover:shadow-sky-700 bg-blend-multiply"
                   src="/BH_INH_Logo_2.png"
                   alt="Logo"
                   width="64"
                   height="64"/>
            </router-link>
            <p class="text-white text-xl font-bold tracking-tight drop-shadow-lg">
              Intelligence
            </p>
          </div>

          <div class="flex lg:hidden">
            <button type="button"
                    class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-white">
              <span class="sr-only">Open main menu</span>
            </button>
          </div>

          <div class="hidden lg:flex lg:gap-x-12">
            <router-link v-for="item in headerNav" :key="item.name" :to="item.href"
                         class="text-sm hover:text-sky-400 font-semibold leading-6 text-white drop-shadow-md">
              {{ item.name }}
            </router-link>
          </div>
        </nav>

        <div class="flex-grow flex items-center justify-center px-4 pb-12">
          <h1  class="text-white text-xl md:text-3xl font-bold tracking-tight text-center drop-shadow-lg">
            You spin me right round
          </h1>
        </div>

        <div
          class="absolute bottom-0 inset-x-0 h-[3px] bg-linear-to-b from-0% to-surface-800"></div>

      </header>
      <div class="flex-grow bg-surface-900">
        <router-view></router-view>
      </div>
      <footer class="bg-surface-800 flex flex-col mt-auto w-full">

        <div class="w-full overflow-hidden leading-none flex justify-center h-[76px]">
          <svg
            class="text-surface-900 min-w-[2850px] h-[76px]"
            width="2850"
            height="76"
            viewBox="0 0 2850 76"
            xmlns="http://www.w3.org/2000/svg"
          >
            <rect
              v-for="pixel in ditherPixels"
              :key="pixel.key"
              :x="pixel.x"
              :y="pixel.y"
              width="19"
              height="19"
              fill="currentColor"
            />
          </svg>
        </div>

        <div class="px-6 py-12 lg:px-8 w-full max-w-4xl mx-auto flex flex-col md:flex-row justify-between items-center gap-6 text-surface-400">

          <div class="flex flex-col">
            <img src="/BH_INH_Logo_2.png" alt="Logo" width="80" height="80" class="opacity-80 pb-2" />
            <span class="text-lg font-semibold text-surface-100 tracking-wider pb-3">Intelligence Modding</span>
            <span class="text-sm font-semibold text-surface-100 tracking-wider">Mhhh tasty mods</span>
          </div>

          <nav class="flex flex-col gap-y-3 text-sm font-medium">
            <router-link
              v-for="item in footerNav.main"
              :key="item.name"
              :to="item.href"
              class="hover:text-surface-100 transition-colors"
            >
              {{ item.name }}
            </router-link>
          </nav>

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
