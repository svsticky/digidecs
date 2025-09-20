export abstract class IBAN{

  static checkIBAN(iban: string): boolean {  
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


}