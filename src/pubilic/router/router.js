import JsonListView from '../part/jsonList/JsonListView.vue'
import JsonView from '../part/jsonFormat/JsonView.vue'
import RouteView from '../part/routeChange/RouteView.vue'
import ScrcpyView from '../part/scrcpyLink/ScrcpyView.vue'
import Sprite from '../part/sprite/Sprite.vue'
import { Cellphone,Cpu,Brush,Expand,MagicStick } from '@element-plus/icons-vue'
export default [
    {
        name: 'JSON列表展示',
        conmpent: JsonListView,
        path: '/JsonListView',
        icon: Expand,
    },
    {
        name: 'JSON格式化',
        conmpent: JsonView,
        path: '/JsonView',
        icon: Brush,
    },
    {
        name: '路由切换',
        conmpent: RouteView,
        path: '/RouteView',
        icon: Cpu,
    },
    {
        name: '手机投屏',
        conmpent: ScrcpyView,
        path: '/ScrcpyView',
        icon: Cellphone,
    },
    {
        name: '精灵图',
        conmpent: Sprite,
        path: '/Sprite',
        icon: MagicStick,
    }
]