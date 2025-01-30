<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Container from "./components/Container.vue";
import commUtil from '@/comm/commUtil'
import { ref,onUnmounted, onMounted } from "vue"
import { appWindow } from '@tauri-apps/api/window'


const windowHeight = ref('')
const windowWidth = ref('')
// 窗口适应
function initWindowSize(){
  const winobj = commUtil.getWindowInfo()
  windowHeight.value = winobj.hight - 45
  windowWidth.value = winobj.width - 10
}
const debounce = (fn, delay) => {
  let timer;
  return function () {
    if (timer) {
      clearTimeout(timer);
    }
    timer = setTimeout(fn, delay);
  }
};
const cancalDebounce = debounce(initWindowSize, 200);
window.addEventListener('resize', cancalDebounce);




onMounted(()=>{
  initWindowSize()
})

onUnmounted(()=>{
  window.removeEventListener('resize', cancalDebounce);
})

// 自定义窗口
function windowMinimize(){ appWindow.minimize() }
function windowMaximize(){ appWindow.toggleMaximize() }
function windowClose(){ appWindow.hide() }
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-button" @click="windowMinimize">
      <img
        src="/window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div class="titlebar-button" @click="windowMaximize">
      <img
        src="/window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div class="titlebar-button titlebar-button-close" @click="windowClose">
      <img src="/close.svg" alt="close" />
    </div>
  </div>
  <div class="app" :style="`height: ${windowHeight}px;width: ${windowWidth}px;margin-top:33px`">
    <Container />
  </div>
</template>

<style scoped>
.titlebar {
  height: 30px;
  background: var(--el-bg-color);
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  filter: invert(1);
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: var(--el-color-info-light-3);
}
.titlebar-button-close:hover{
  background: var(--el-color-error);
  filter: initial;
}
</style>
