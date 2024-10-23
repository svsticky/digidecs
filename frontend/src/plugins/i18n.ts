/* eslint-disable @typescript-eslint/no-explicit-any */

import {createI18n} from "vue-i18n";
import EN from "@/plugins/locales/en";
import NL from "@/plugins/locales/nl";

export default createI18n({
  fallbackLocale: 'nl',
  locale: 'nl',
  messages: {
    "en-US": EN as any,
    nl: NL as any
  }
})

/* eslint-enable @typescript-eslint/no-explicit-any */