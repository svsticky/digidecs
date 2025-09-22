import {Locale} from "@/plugins/locales/locale";

const NL: Locale = {
  site_title: "DigiDecs",
  error: "Er is iets verkeerd gegaan, probeer het later opnieuw",
  home: {
    title: "DigiDecs",
    subtitle: "Digitaal declareren bij Sticky",
    invalidFieldsError: "Een of meerdere velden zijn onjuist ingevuld",
    form: {
      name: "Naam",
      iban: "IBAN",
      email: "Email",
      value: "Bedrag",
      what: "Wat",
      commission: "Waarvoor / Welke commissie",
      notes: "Opmerkingen",
      files: "Bonnetjes / Facturen",
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
        iban: "GB94BARC10201530093459",
        value: "20,20",
        what: "Graafmachine",
        commission: "Bestuur, lul!"
      }
    },
    submit: "Verstuur"
  },
  submitted: {
    title: "Succes!",
    description: "Jouw digidecs is verstuurd! Mocht je na 7 dagen je geld nog niet terug hebben gekregen, neem dan contact op met de penningmeester"
  }
}

export default NL;