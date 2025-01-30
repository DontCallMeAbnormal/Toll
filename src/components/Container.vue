<script setup>
import MenuConmpent from './Menu.vue';
import NotFound from './NotFoundPage.vue';
import router from '../pubilic/router/router';
import { ref,computed,onMounted } from "vue"

// 页面路由控制
const currentPath = ref('')
const currentView = computed(() => {
  const path = currentPath.value.slice(1)
  return router.find(i=>i.path === path)?.conmpent || NotFound
})
function menuOnselect(index,indexPath,routeResult){
  currentPath.value = index
}
function onInitAside(e){ // 初始页面
  currentPath.value = e.defaultRouter
}

// 控制菜单栏
const showAside = ref(false)
const menuConmpentRef = ref()
const asideWidth = computed(()=>{
  const initWidth = menuConmpentRef?.value?.$el?.nextElementSibling?.offsetWidth ?? 162
  return showAside.value ? `${initWidth}px`:'0px'
})

</script>


<template>
    <el-container class="common-layout">
      <el-aside :width="asideWidth" class="common-layout-aside"  style="background-color:var(--bg-color)" >
        <div  :style="`width:${asideWidth}`" class="menuclass">
          <MenuConmpent ref="menuConmpentRef"  @select="menuOnselect" @onSegmentedSelect="menuOnselect" @onInit="onInitAside" :route="router" />
        </div>
      </el-aside>
      <el-container>
        <el-header>
          <el-button @click="()=>showAside = !showAside" ><el-icon><Expand /></el-icon></el-button>
        </el-header>
        <!-- <el-header class="common-layout-header">
          <el-button @click="()=>showAside = !showAside" >
            <el-icon><Expand /></el-icon>
          </el-button>
        </el-header> -->
        <el-main class="common-layout-main">
          <component :is="currentView" />
        </el-main>
      </el-container>
    </el-container>
</template>

<style scoped>
.menuclass{
  transition: width .3s ease-in-out;
  background-color: var(--bg-color);
  border-radius: 5px;
  display: inline-flex;
  box-sizing: border-box;
  overflow: hidden;
}
</style>