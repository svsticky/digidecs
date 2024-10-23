// @ts-check

import eslint from '@eslint/js';
import pluginVue from 'eslint-plugin-vue';
import tseslint from 'typescript-eslint';

export default [
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  {
    plugins: {
      'typescript-eslint': tseslint.plugin,
    },
    languageOptions: {
      parserOptions: {
        parser: tseslint.parser,
        project: './tsconfig.json',
        extraFileExtensions: ['.vue'],
        sourceType: 'module',
      },
    },
  },
  {
    ignores: [
      'node_modules/**',
      'eslint.config.js',
      'babel.config.js',
      'vue.config.js',
      'vite.config.ts',
      'src/vite-end.d.ts',
      "dist/**"
    ],
  }
];
