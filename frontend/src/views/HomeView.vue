<template>
  <v-container class="d-flex flex-column align-center">
    <MaterialBanner
      :width="width"
      :text="error"
      icon="mdi-alert-outline"
      title="Error"
      type="error"
      @close="error = undefined"
    />

    <v-card
      :width="width"
      class="pl-2 pr-2"
    >
      <v-card-title> {{ $t("home.title") }} </v-card-title>
      <v-card-subtitle> {{ $t("home.subtitle") }}</v-card-subtitle>

      <v-card-text>
        <v-form v-model="form.valid">
          <v-text-field
            v-model="form.name"
            color="primary"
            :label="$t('home.form.name')"
            :hint="$t('home.form.hints.name')"
            :rules="rules.required"
          />

          <v-text-field
            v-model="form.iban"
            color="primary"
            :label="$t('home.form.iban')"
            :hint="$t('home.form.hints.iban')"
            :rules="rules.iban"
          />

          <v-text-field
            v-model="form.email"
            color="primary"
            :label="$t('home.form.email')"
            :hint="$t('home.form.hints.email')"
            :rules="rules.email"
          />

          <v-text-field
            v-model="form.value"
            color="primary"
            :label="$t('home.form.value')"
            :hint="$t('home.form.hints.value')"
            :rules="rules.value"
            prepend-icon="mdi-currency-eur"
          />

          <v-text-field
            v-model="form.what"
            color="primary"
            :label="$t('home.form.what')"
            :hint="$t('home.form.hints.what')"
            :rules="rules.required"
          />

          <v-text-field
            v-model="form.commission"
            color="primary"
            :label="$t('home.form.commission')"
            :hint="$t('home.form.hints.commission')"
            :rules="rules.required"
          />

          <v-file-input
            v-model="form.files"
            color="primary"
            :label="$t('home.form.files')"
            :rules="rules.files"
            accept="image/*,.pdf"
            multiple
            chips
            size
          />

          <p class="mt-0 mb-3 text-grey">
            {{ $t('home.form.filesExplanation') }}
          </p>

          <v-textarea
            v-model="form.notes"
            color="primary"
            :label="$t('home.form.notes')"
            auto-grow
          />

          <v-checkbox
            v-model="form.checked"
            color="primary"
            :label="$t('home.form.checked')"
          />
        </v-form>
      </v-card-text>

      <v-card-actions>
        <v-spacer />
        <v-btn
          :loading="loading"
          variant="tonal"
          elevation="1"
          color="primary"
          :disabled="loading || !form.valid"
          @click="submit"
        >
          {{ $t("home.submit") }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang="ts">

import {defineComponent} from "vue";
import {InputValidationRules} from "@/main";
import {Digidecs, DigidecsLocale} from "@/scripts/digidecs";
import MaterialBanner from "@/views/components/MaterialBanner.vue";

interface Data {
  error: string | undefined,
  loading: boolean,
  form: {
    valid: boolean,
    name: string,
    iban: string,
    email: string,
    value: string,
    what: string,
    commission: string,
    notes: string | null,
    files: File[],
    checked: boolean,
  },
  rules: {
    required: InputValidationRules,
    iban: InputValidationRules,
    email: InputValidationRules,
    value: InputValidationRules,
    files: ((v: File[]) => string | boolean)[],
  }
}

function checkIBAN(iban: string): boolean {  
  // Tested with:
  // GB33BUKB20201555555555 (correct)
  // GB94BARC10201530093459 (correct)
  // gB94BARC10201530093459 (correct, evaluates to its uppercase equivalent)
  // GB94BARC20201530093459 (incorrect checksum)
  iban = iban.toUpperCase();
  const ibanRegex = /^[A-Z]{2}[0-9]{2}[A-Z0-9]{1,30}$/;
  if (!ibanRegex.test(iban)) {
    return false;
  }

  // ISO/IEC 7064:2003
  // Convert IBAN to numeric representation (rearrange and replace letter with corresponding numeric value)
  const numericIban = (iban.slice(4) + iban.slice(0,4))
  .replace(/[A-Z]/g, char => (parseInt(char, 36)).toString());
  // Match only sequences of 1-7 digits, convert each to number and add, then take result mod 97.
  const remainder = numericIban
    .match(/\d{1,7}/g)?.reduce((acc, block) => Number(acc + block) % 97, 0); 
  return remainder===1;
}

export default defineComponent({
  components: {MaterialBanner},
  data(): Data {
    return {
      error: undefined,
      loading: false,
      form: this.emptyForm(),
      rules: {
        required: [
          v => !!v || this.$t("home.form.rules.required"),
        ],
        iban: [
          v => !!v || this.$t("home.form.rules.required"),
          v => checkIBAN(v) || this.$t("home.form.rules.ibanInvalid")
        ],
        email: [
          v => !!v || this.$t("home.form.rules.required"),
          v => /[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]/.test(v) || this.$t("home.form.rules.emailInvalid")
        ],
        value: [
          v => !!v || this.$t("home.form.rules.required"),
          v => /^[+-]?([0-9]+([.|,][0-9]*)?|[.|,][0-9]+)$/.test(v) || this.$t("home.form.rules.valueInvalid"),
          v => !isNaN(Number.parseFloat(v)) || this.$t("home.form.rules.valueInvalid"),
          v => Number.parseFloat(v) > 0.0 || this.$t("home.form.rules.valueInvalid")
        ],
        files: [
          v => !!v || this.$t("home.form.rules.required"),
          v => !v || v.map(z => z.size).reduce((buf, a) => buf + a, 0) < 1.5e7 || this.$t("home.form.rules.filesTooLarge")
        ]
      }
    }
  },
  computed: {
    width() {
      if (this.$vuetify.display.mobile) {
        return "100%";
      } else {
        return "75%";
      }
    }
  },
  methods: {
    async submit() {

      if(!this.form.checked || this.form.files.length == 0) {
        this.error = this.$t("home.invalidFieldsError");
        window.scroll({
          top: 0
        });

        return;
      }

      this.loading = true;
      console.log("Digidecs upload started");
      const r = await Digidecs.start(
        this.form.name,
        this.form.iban,
        this.form.email,
        Number.parseFloat(this.form.value.replaceAll(",",".")),
        this.form.what,
        this.form.commission,
        this.form.notes,
        this.form.files,
        this.getCurrentLocale(),
      );

      if(r.isErr()) {
        this.displayError();
        return;
      }

      const digidecs = r.unwrap();
      for(let i = 0; i < this.form.files.length; i++) {
        console.log(`Uploading file ${i + 1}/${this.form.files.length}`);

        const r1 = await digidecs.upload_attachment(this.form.files[i], i);
        if(r1.isErr()) {
          this.displayError();
          return;
        }
      }

      console.log("Digidecs upload complete");
      const r1 = await digidecs.complete();
      if(r1.isErr()) {
        this.displayError();
        return;
      }

      this.loading = false;
      this.form = this.emptyForm();

      this.$router.push('/complete');
    },
    getCurrentLocale(): DigidecsLocale {
      const currentLocale = this.$i18n.locale;
      switch(currentLocale) {
        case "nl": return DigidecsLocale.NL;
        case "en-US": return DigidecsLocale.EN;
        default: return DigidecsLocale.NL;
      }
    },
    displayError() {
      this.error = this.$t('error');
      this.loading = false;
    },

    emptyForm() {
      return {
        valid: true,
        name: "",
        iban: "",
        email: "",
        value: "",
        what: "",
        commission: "",
        notes: null,
        files: [],
        checked: false,
      };
    }
  }
})

</script>