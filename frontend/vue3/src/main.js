import './assets/main.css'
// import 'virtual:uno.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import i18n from './assets/lang/i18n'

const app = createApp(App)

app.use(router)
app.use(i18n)
app.mount('#app')
