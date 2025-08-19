<template>
  <div>
    <v-list>
      <v-virtual-scroll
          :height="600"
          :items="files"
          item-height="48px"
      >

        <template #default="{ item }">
          <v-list-item
              :title="item.name"
              :prepend-icon="item.isDir ? 'mdi-folder' : 'mdi-file'"
              density="compact"
              @click="updatePath(`${currentPath}/${item.name}`)"
          />
        </template>

      </v-virtual-scroll>
    </v-list>
  </div>
</template>

<script setup lang="ts">
import {onMounted, type Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";


interface FileEntry {
  name: string;
  isDir: boolean;
}

const files = ref<FileEntry[]>([])
const currentPath = ref()

async function loadDirectory(targetPath: string) {
  try {
    files.value = await invoke <FileEntry[]> ('read_dir', { path: targetPath })
  } catch (e) {
    console.log("Error leyendo directorio: ", e)
  }
}

function updatePath(targetPath: string) {
  //Prevents double slashes
  targetPath = targetPath.replace(/\/{2,}/g, '/')
  loadDirectory(targetPath)
  currentPath.value = targetPath
}

onMounted(() => {
  updatePath("/")
})
</script>