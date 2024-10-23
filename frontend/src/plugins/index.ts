import vuetify from './vuetify'
import router from '../router'

import type { App } from 'vue'
import i18n from "@/plugins/i18n";

export function registerPlugins (app: App) {
  app
    .use(vuetify)
    .use(i18n)
    .use(router)
}