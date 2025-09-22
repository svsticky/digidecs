import {Locale} from "@/plugins/locales/locale";

const EN: Locale = {
  site_title: "DigiDecs",
  error: "Something went wrong, please try again later",
  home: {
    title: "DigiDecs",
    subtitle: "Declare digitally at Sticky",
    invalidFieldsError: "One or multiple fields are incorreect",
    form: {
      name: "Name",
      iban: "IBAN",
      email: "Email",
      value: "Amount",
      what: "What",
      commission: "For what / which commission",
      notes: "Notes",
      files: "Receipts / Invoices",
      filesExplanation: "Only .pdf, .jpg or .png files. You can submit multiple documents. Ensure the date, the (VAT) amount and the different products or services are clearly readable.",
      checked: "I have checked everything and filled this form thruthfully",
      rules: {
        required: "Required",
        ibanInvalid: "Invalid IBAN",
        emailInvalid: "Invalid email address",
        valueInvalid: "Invalid amount",
        filesTooLarge: "The maximum file size is 15MB"
      },
      hints: {
        name: "Treasurer",
        email: "{'eindbaas@svsticky.nl'}",
        iban: "GB94BARC10201530093459",
        value: "20,20",
        what: "Digging Machine",
        commission: "The board, obviously!"
      }
    },
    submit: "Submit"
  },
  submitted: {
    title: 'Success!',
    description: "Your digidecs has been sent! If you haven't received your money back after 7 days, contact the treasurer"
  }
}

export default EN;