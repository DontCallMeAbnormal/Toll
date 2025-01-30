<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import { ref,reactive,watch, onUnmounted,onMounted } from "vue";
import { ElMessage } from "element-plus";
import { Close } from '@element-plus/icons-vue';
import { listen } from '@tauri-apps/api/event';

const processList = ref([])
const btnLoading = ref(false)
const keyMapState = ref(false)

/**
 * 表单对象
 */
const formState = reactive({
  window_borderless: false,
  window_x: 0,
  window_y: 130,
  window_width: 400,
  window_height: 888,
  push_target: '/sdcard/Download/WeiXin',
  power_off_on_close: false,
  turn_screen_off: false,
  stay_awake: false,
  always_on_top: false,
  fullscreen: false,
  tcpip: '',
  otg: false,
  hid_mouse: false,
  hid_keyboard: false
});

const formItems = [
  { label: '窗口位置x', field: 'window_x', type: 'input', number: true, span: 6 },
  { label: '窗口位置y', field: 'window_y', type: 'input', number: true, span: 6 },
  { label: '窗口宽度', field: 'window_width', type: 'input', number: true, span: 6 },
  { label: '窗口高度', field: 'window_height', type: 'input', number: true, span: 6 },
  { label: '文件推送至设备目录', field: 'push_target', type: 'input', span: 24 },
  { label: '禁用窗口边框', field: 'window_borderless', type: 'switch', span: 8 },  { label: '退出设备时息屏', field: 'power_off_on_close', type: 'switch', span: 8 },
  { label: '在屏幕关闭的状态进行镜像', field: 'turn_screen_off', type: 'switch', span: 8 },
  { label: '使设备不休眠', field: 'stay_awake', type: 'switch', span: 8 },
  { label: '保持窗口在最前', field: 'always_on_top', type: 'switch', span: 8 },
  { label: '全屏', field: 'fullscreen', type: 'switch', span: 8 },
  { label: 'OTG模式', field: 'otg', type: 'switch', span: 8 },
  { label: 'hid鼠标', field: 'hid_mouse', type: 'switch', span: 8 },
  { label: 'hid键盘', field: 'hid_keyboard', type: 'switch', span: 8 },
  { label: '无线调试地址', field: 'tcpip', type: 'input', span: 16 },
];

async function playScrcpy(){
    btnLoading.value = true
    try {
        const scrcpyParam = {
            ...formState,
        };
        if (scrcpyParam.otg){
            scrcpyParam.window_width = 100;
            scrcpyParam.window_height = 100;
        }
        await invoke('play_scrcpy',{scrcpyParam});
        ElMessage({  message: '已创建投屏',  type: 'success',})
    } catch (error) {
        console.log(error);
        const formattedString = error.toString()
            .replaceAll('&','&amp;')
            .replaceAll('<','&lt;')
            .replaceAll('>','&gt;')
            .replaceAll('"','&quot;')
            .replaceAll('\'','&#039;')
            .replaceAll('\n','</br>')
            .replaceAll('\r','')
            .replaceAll(' ','&nbsp;')
        ElMessage({  message: formattedString,  type: 'warning', dangerouslyUseHTMLString: true})
    }
    await showAllProcess();
    btnLoading.value = false
}

async function showAllProcess(){
    const resultProcessList = await invoke('get_all_life_process');
    processList.value = resultProcessList
}

async function closeProcess(processId){
    try {
        await invoke('close_process_by_id',{processId});
        ElMessage({  message: '已关闭进程',  type: 'success',})
    } catch (error) {
        ElMessage({  message: error,  type: 'warning',})
    }
    await showAllProcess();
}

setInterval(async () => {
    await showAllProcess();
}, 1000);


async function uploadFileToAdb(file){
    const getFileBase64 = (param_file) => {return new Promise((resolve, reject) => {
          const reader = new FileReader();
          reader.onload = (e) => {
            const base64String = e.target.result.split(',')[1]; // 去掉"data:application/octet-stream;base64,"
            resolve(base64String);
          };
          reader.onerror = reject;
          reader.readAsDataURL(param_file);
    })};
    const fileBase64 = await getFileBase64(file);
    console.log('file',file)
    try {
        const result = await invoke('upload_file_to_adb',{fileBase64,pushTarget: formState.push_target,fileName: file.name});
        ElMessage({  message: result,  type: 'success',})
    } catch (error) {
        ElMessage({  message: error,  type: 'warning',})
    }
    console.log('result',result);
    return false
}


const unlisten = listen('press-alt', (event) => {
    console.log(`Got press-alt`);
    keyMapState.value = !keyMapState.value
    keyMapSwitch(keyMapState.value)
});
async function keyMapSwitch(val) {
    console.log(val);
    if (val) {
        const result = await invoke('enable_keyborad_input',{});
    } else {
        const result = await invoke('disable_keybord_input',{});
    }
}

onMounted(() => {
    keyMapState.value = false
    keyMapSwitch(keyMapState.value)
})
// 注销监听
 onUnmounted(() => {
    unlisten.then(e=>e())
 })
</script>

<template>
    <el-upload
        action="/"
        :before-upload="uploadFileToAdb"
        ref="upload"
        :auto-upload="true"
        :show-file-list="false"
        :drag="true">
        <template #trigger>
            <el-button size="small" type="primary">推送文件到设备</el-button>
            <p>目标路径: {{formState.push_target}}</p>
        </template>
    </el-upload>
    <el-row :gutter="5">
        <el-col :span="item.span ?? 8" v-for="(item, index) in formItems" :key="index">
            <el-form-item :label="item.label">
                <template v-if="item.type === 'switch'">
                    <el-switch v-model="formState[item.field]" />
                </template>
                <template v-else-if="item.type === 'input' && item.number">
                    <el-input type="number" v-model.number="formState[item.field]" />
                </template>
                <template v-else-if="item.type === 'input'">
                    <el-input v-model="formState[item.field]" />
                </template>
                <!-- 其他类型组件可以根据需要添加 -->
            </el-form-item>
        </el-col>
    </el-row>
    <el-button @click="playScrcpy" :loading="btnLoading" >投屏</el-button>
    键盘模拟点击(F4) <el-switch v-model="keyMapState" @change="keyMapSwitch" />
    <el-row :gutter="5">
        <el-col :span="8" v-for="(item,index) in processList" :key="index">
            <el-card>
                <div class="card-header">
                    <span>投屏进程 {{item.process_id}}</span>
                </div>
                <template #footer>
                    <div style="text-align: right;">
                        <el-button type="danger" size="small" :icon="Close" round @click="closeProcess(item.process_id)">强制终止</el-button>
                    </div>
                </template>
            </el-card>
        </el-col>
    </el-row>
</template>