import { createApp } from "vue";
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import JsonViewer from 'vue3-json-viewer'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import './styles.css'
//添加样式
import "vue3-json-viewer/dist/index.css";
import App from "./App.vue";


const app = createApp(App)
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}
app.use(ElementPlus)
app.use(JsonViewer)
app.mount("#app");
