import { createPinia } from 'pinia';
import { createApp } from 'vue';
import './index.scss';
import App from './App.vue';

const app = createApp(App);
app.use(createPinia()).mount('#app');