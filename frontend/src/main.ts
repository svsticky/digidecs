import { createApp } from 'vue'
import { registerPlugins } from '@/plugins'
import App from './App.vue'

export const server = "http://localhost:8080";

// Vuetify missing type
export type InputValidationRule = (v: string) => string | boolean;
export type InputValidationRules = InputValidationRule[];

export type DataTableHeader = { title: string, value: string }
/**
 * The item you get when you click a row
 * E.g: @click:row="(_: any, v: RowItem<Foo>) => console.log(v.item))"
 * Will print the T.
 */
export type RowItem<T> = { item: T }

const app = createApp(App);

registerPlugins(app)

app.mount('#app')
