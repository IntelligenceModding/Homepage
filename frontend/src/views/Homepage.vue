<script setup lang="ts">
import {reactive, ref} from "vue";
import Checkbox from "primevue/checkbox";

//TODO: No logic implemented, needs to be fetched from the backend later.

// Define filter categories here
const filters: FilterCategory[] = [
  {
    key: "minecraftVersion",
    label: "Minecraft Version",
    options: ["1.21", "1.20", "1.19", "1.18"],
  },
  {
    key: "modLoader",
    label: "Mod Loader",
    options: ["Fabric", "Forge", "NeoForge", "Quilt"],
  },
];

const projectTypes: string[] = [
  "All Projects",
  "Mods",
  "Modpacks",
  "Resource Packs"
];

// Selected state per category, keyed by category.key. e.g. selected.projectType === ["Mod", "Shader"]
const selectedFilters = reactive<Record<string, string[]>>(
  Object.fromEntries(filters.map((c) => [c.key, []]))
);

// Selected project types
const selectedTypes = ref<string[]>([]);

function toggleType(type: string) {
  const i = selectedTypes.value.indexOf(type);
  if (i === -1) {
    selectedTypes.value.push(type);
  } else {
    selectedTypes.value.splice(i, 1);
  }
}
</script>

<template>
  <div class="mx-48 my-8 grid grid-cols-12 gap-x-16 text-lg">
    <div class="col-span-3 self-start rounded-4xl bg-surface-800 p-6">
      <p class="font-bold">Filter</p>
      <!-- Filters -->
      <div v-for="filter in filters" :key="filter.key" class="mt-4">
        <p class="pb-1">{{ filter.label }}</p>
        <div
          v-for="option in filter.options"
          :key="option"
          class="flex items-center gap-2 font-light text-md"
        >
          <Checkbox
            size="small"
            v-model="selectedFilters[filter.key]"
            :inputId="`${filter.key}-${option}`"
            :value="option"
          />
          <label :for="`${filter.key}-${option}`">{{ option }}</label>
        </div>
      </div>
    </div>
    <div class="col-span-9 ml-16">
      <!-- Project Types -->
      <div class="grid grid-cols-4 gap-4 text-2xl mb-16">
        <button
          v-for="type in projectTypes"
          :key="type"
          type="button"
          @click="toggleType(type)"
          class="flex items-center justify-center gap-2 rounded-4xl p-4 text-center transition-colors bg-surface-800"
          :class="selectedTypes.includes(type) ? 'shadow-lg shadow-violet-800 ring-3 ring-violet-800' : 'shadow-lg shadow-surface-900 ring-3 ring-surface-900'"
        >
          {{ type }}
          <span v-if="selectedTypes.includes(type)" aria-hidden="true">&times;</span>
        </button>
      </div>
      <!-- Projects -->
      <div class="grid grid-cols-1 gap-4 ml-12">
        <div v-for="i in [1,1,1,1,1,1,1,1,1,1,1]" class="flex items-center gap-6 rounded-4xl bg-surface-800 py-8 px-8 my-4">
          <img src="/BH_INH_Logo_2.png" alt="Project 1" class="w-24 h-24 -ml-20 shrink-0 rounded-4xl object-cover" />
          <div>
            <h2 class="text-2xl mb-2 font-semibold">Intelligence - New Horizon</h2>
            <p>The perfect modpack to explore 1.16.5 with new quests, magic, nature and exploration The perfect modpack to explore 1.16.5 with new quests, magic, nature and exploration</p>
          </div>
        </div>
      </div>
    </div>
  </div>

</template>

<style scoped>

</style>
