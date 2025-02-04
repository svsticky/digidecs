import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import { checkIBAN } from '../views/HomeView.vue';
import {IBAN} from "../scripts/iban";


describe('checkIBAN', () => {
  it('should return true for a valid IBAN', () => {
    const validIBAN = 'DE89370400440532013000';
    expect(IBAN.checkIBAN(validIBAN)).toBe(true);
  });

  it('should return false for an invalid IBAN', () => {
    const invalidIBAN = 'DE89370400440532013001';
    expect(IBAN.checkIBAN(invalidIBAN)).toBe(false);
  });

  it('should return false for an IBAN with incorrect length', () => {
    const shortIBAN = 'DE8937040044053201300';
    expect(IBAN.checkIBAN(shortIBAN)).toBe(false);
  });

  it('should return false for an IBAN with invalid characters', () => {
    const invalidCharIBAN = 'DE89$70400440532013000';
    expect(IBAN.checkIBAN(invalidCharIBAN)).toBe(false);
  });

  it('should return false for an empty IBAN', () => {
    const emptyIBAN = '';
    expect(IBAN.checkIBAN(emptyIBAN)).toBe(false);
  });
});