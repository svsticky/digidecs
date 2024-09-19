import {Locale} from "@/plugins/locales/locale";

const NL: Locale = {
  site_title: "DigiDecs",
  error: "Er is iets verkeerd gegaan, probeer het later opnieuw",
  home: {
    title: "DigiDecs",
    subtitle: "Digitaal declareren bij Sticky",
    invalidFieldsError: "Een of meerdere velden zijn onjuist ingevuld",
    success: "Success! Je declaratie is ingestuurd. Mocht je na 5 werkdagen je geldf nog niet hebben ontvangen, neem dan contact op met de Penningmeester: penningmeester@svsticky.nl.",
    form: {
      name: "Naam",
      iban: "IBAN",
      email: "Email",
      value: "Bedrag",
      what: "Wat",
      commission: "Waarvoor / Welke commissie",
      notes: "Opmerkingen",
      files: "Bonnetjes",
      filesExplanation: "Alleen .pdf, .jpg, en .png bestanden, je kan meerdere bestanden selecteren. Zorg dat de datum, het (btw) bedrag en de verschillende producten of diensten goed leesbaar zijn.",
      checked: "Ik heb alles gecheckt en naar waarheid ingevuld",
      rules: {
        required: "Vereist",
        ibanInvalid: "Ongeldig IBAN",
        emailInvalid: "Ongeldig email adres",
        valueInvalid: "Ongeldig bedrag",
        filesTooLarge: "De maximale totale bestandsgrote is 15MB."
      },
      hints: {
        name: "Penningmeester",
        email: "{'eindbaas@svsticky.nl'}",
        iban: "NL13TEST0123456789",
        value: "19,19",
        what: "Graafmachine",
        commission: "Bestuur, lul!"
      }
    },
    submit: "Verstuur"
  }
}

export default NL;