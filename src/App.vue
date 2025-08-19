<template>
  <v-app>
    <v-main>
      <v-container>
        <NavigationComponent/>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import NavigationComponent from "@/components/NavigationComponent.vue";
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
