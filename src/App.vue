<template>
  <div>
    <img :src="vueLogo" class="logo vue" alt="Vue logo" />
  </div>

  <div>

      <h1>Explorador de Archivos</h1>

      <ul>
        <li v-for="file in files">
          {{ file.isDir ? "📁" : "📄" }} {{ file.name}}
        </li>
      </ul>

  </div>
  <HelloWorld msg="Vite + Vue" />
</template>

<script setup lang="ts">
import vueLogo from "@/assets/vue.svg"
import HelloWorld from './components/HelloWorld.vue'
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

interface FileEntry {
  name: string;
  isDir: boolean;
}

const files = ref<FileEntry[]>([])

async function loadDirectory(path: string) {
  try {
    files.value = await invoke < FileEntry[] > ('read_dir', {path})
  } catch (e) {
    console.log("Error leyendo directorio: ", e)
  }
}

onMounted(() => {
  loadDirectory("/")
})
</script>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
