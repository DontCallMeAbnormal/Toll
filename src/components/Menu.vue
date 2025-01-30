<script setup>
import {
  Menu as IconMenu,
} from '@element-plus/icons-vue'
import { ref,onMounted,computed } from 'vue'

const emit = defineEmits(['onInit','onSegmentedSelect'])
const props = defineProps(['route'])
const segmentedModel = ref('')

onMounted(()=>{
  emit('onInit',{defaultRouter:`#${props.route[0].path}`})
  segmentedModel.value = props.route[0].path
  console.log(props);
})

function onSegmentedSelect(value){
  emit('onSegmentedSelect',`#${value}`)
}

const segmented_list = computed(()=>{
  return props.route.map(i=>({...i,value:i.path}));
})


</script>

<template>
  <!-- <el-segmented 
      background-color="var(--bg-color)"
      text-color="var(--menu-text-color)"
      active-text-color="var(--menu-active-text-color)"
      style="border: 0px;"
      v-model="segmentedModel"
      size="small"
      @change="()=>{onSegmentedSelect(segmentedModel)}"
      :options="segmented_list">
      <template #default="{ item }">
        <div class="flex flex-col items-center gap-2 p-2">
          <el-icon size="20">
            <component :is="item.icon || IconMenu" />
          </el-icon>
          <div style="font-size:0.8em;">{{ item.name }}</div>
        </div>
      </template>
  </el-segmented> -->
    <el-menu
        background-color="var(--bg-color)"
        text-color="var(--menu-text-color)"
        active-text-color="var(--menu-active-text-color)"
        style="border: 0px;"
        router="true"
        :default-active="`#${props.route[0].path}`"
      >
      <el-menu-item :index="`#${item.path}`" v-for="(item, index) in props.route" :key="index">
        <el-icon>
          <component :is="item.icon || IconMenu" />
        </el-icon>
        <span>{{item.name}}</span>
      </el-menu-item>
    </el-menu>
</template>