<template>
  <v-app>
    <default-bar />

    <default-view />
  </v-app>
</template>

<script lang="ts" setup>
  import DefaultBar from './components/AppBar.vue'
  import DefaultView from './components/RouterView.vue'
</script>