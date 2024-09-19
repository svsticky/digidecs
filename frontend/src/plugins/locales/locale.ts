export interface Locale {
  site_title: string,
  error: string,
  home: {
    title: string,
    subtitle: string,
    invalidFieldsError: string,
    success: string,
    form: {
      name: string,
      iban: string,
      email: string,
      value: string,
      what: string,
      commission: string,
      notes: string,
      files: string,
      filesExplanation: string,
      checked: string,
      rules: {
        required: string,
        ibanInvalid: string,
        valueInvalid: string,
        emailInvalid: string,
        filesTooLarge: string,
      },
      hints: {
        name: string,
        iban: string,
        email: string,
        value: string,
        what: string,
        commission: string,
      }
    },
    submit: string,
  }
}