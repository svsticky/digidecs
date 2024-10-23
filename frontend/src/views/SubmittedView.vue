<template>
  <v-container class="d-flex flex-column align-center">
    <v-card
      :width="width"
      class="pl-2 pr-2"
    >
      <v-card-title>{{ $t('submitted.title') }}</v-card-title>
      <v-card-text>
        <div>
          {{ $t('submitted.description') }}
        </div>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import {defineComponent} from "vue"

export default defineComponent({
  computed: {
    width() {
      if (this.$vuetify.display.mobile) {
        return "100%";
      } else {
        return "75%";
      }
    }
  },
})
</script>