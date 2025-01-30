import { invoke } from '@tauri-apps/api'
export default {

    /**
     * 获取窗口宽高
     */
    getWindowInfo: () => {
        const windowInfo = {
            width: window.innerWidth,
            hight: window.innerHeight
        }
        return windowInfo
    },

    /**
     * 获取文本数据
     */
    getTextResource: async (resourceName)=>{
        return invoke('get_text_config',{name: resourceName})
    },
    setTextResource: async (resourceName,content)=>{
        return invoke('set_text_config',{name: resourceName, content})
    }
}