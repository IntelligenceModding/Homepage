<script setup lang="ts">
import {reactive} from "vue";
import Checkbox from "primevue/checkbox";

interface FilterCategory {
  key: string;
  label: string;
  options: string[];
}

// Define filter categories here — add a new object (or a new option) and the UI updates automatically.
const categories: FilterCategory[] = [
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

// Selected state per category, keyed by category.key. e.g. selected.projectType === ["Mod", "Shader"]
const selected = reactive<Record<string, string[]>>(
  Object.fromEntries(categories.map((c) => [c.key, []]))
);
</script>

<template>
  <div class="mx-48 my-8 grid grid-cols-12 gap-x-16 text-lg">
    <div class="col-span-3 rounded-4xl bg-surface-800 p-6">
      <p class="font-bold">Filter</p>
      <div v-for="category in categories" :key="category.key" class="mt-4">
        <p class="pb-1">{{ category.label }}</p>
        <div
          v-for="option in category.options"
          :key="option"
          class="flex items-center gap-2 font-light text-md"
        >
          <Checkbox
            size="small"
            v-model="selected[category.key]"
            :inputId="`${category.key}-${option}`"
            :value="option"
          />
          <label :for="`${category.key}-${option}`">{{ option }}</label>
        </div>
      </div>
    </div>
    <div class="col-span-9">
      <div class="grid grid-cols-4 gap-4">
        <!-- Project type filters -->
        <div class="bg-surface-800 p-4 rounded-4xl text-center">
          All Projects
        </div>
        <div class="bg-surface-800 p-4 rounded-4xl text-center">
          Mods
        </div>
        <div class="bg-surface-800 p-4 rounded-4xl text-center">
          Modpacks
        </div>
        <div class="bg-surface-800 p-4 rounded-4xl text-center">
          Resource Packs
        </div>
      </div>
      <div>
        <!-- Projects in little wide bubbles with a picture on the left -->
      </div>
    </div>
  </div>

</template>

<style scoped>

</style>
