<script setup >
import { onMounted,ref,defineEmits  } from "vue";
import {
  Check,
  Delete,
  Edit,
  Message,
  Search,
  Star,
  Plus,
  RemoveFilled,
} from '@element-plus/icons-vue'
import { ElMessage,ElMessageBox } from "element-plus";

import commUtil from '@/comm/commUtil'
// 定义事件
const emit = defineEmits(['listOption']);

const listData = ref([])
const dialogVisible = ref(false)
const editItemFormVisible = ref(false) 
const addTypeFormVisible = ref(false) 
const editFormData = ref({})
const addTypeFormData = ref({})

 onMounted(async () => {
  await resetListOption()
 })

function handleClose() {
        // this.$message('对话框已关闭');
}

function handleDelete(index, row) {
  ElMessageBox.confirm('确定要删除吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    const data = await commUtil.getTextResource('jsonListOptions.json')
    let listOption = JSON.parse(data)
    listOption[row.label] = undefined
    // 提交配置修改数据
    await handleSubmit(JSON.stringify(listOption))
    ElMessage({type: 'success',message: '删除成功!'})
  }).catch((error) => {
    ElMessage({type: 'error',message: JSON.stringify(error)})
  });
}


function showEditItemForm(index, row) {
  editItemFormVisible.value = true;
  editFormData.value.domains = JSON.parse(row.value)
  editFormData.value.label = row.label
}
// 显示类型添加表单
function showAddTypeForm() {
  addTypeFormVisible.value = true;
}

/**
 * 提交配置修改
 */
async function handleSubmit(content) {
    const result = await commUtil.setTextResource('jsonListOptions.json',content)
    await resetListOption() //更新数据
}

/**
 * 添加配置
 */
async function handleAddTypeData(addTypeFormData){
  let data = "{}"
  try {
    data = await commUtil.getTextResource('jsonListOptions.json')
    if (data.length == 0 || data == '{}' || data == '""' || data == '' || data == null) {
      data = "{}"
    }
  } catch (error) {
    console.log('error',error);
    data = "{}"
  }
  let listOption = JSON.parse(data)
  console.log('listOption 添加类型',listOption[addTypeFormData.keyname]);
  if (listOption[addTypeFormData.keyname] != undefined){
    ElMessage({type: 'error',message: '类型已存在，请勿重复添加!'})
    return
  }
  listOption[addTypeFormData.keyname] = [{dataIndex:addTypeFormData.keyname,title:addTypeFormData.keyname}]
  await handleSubmit(JSON.stringify(listOption))
  addTypeFormVisible.value = false
  ElMessage({type: 'success',message: '添加成功!'})
}

async function handleEditItemData(itemsData){
  itemsData = itemsData.map(item => {
    return {
      dataIndex: item.dataIndex,
      title: item.title
    }
  })
  console.log('handleEditItemData',itemsData);
  console.log('label',editFormData.value.label);
  const data = await commUtil.getTextResource('jsonListOptions.json')
  let listOption = JSON.parse(data)
  listOption[editFormData.value.label] = itemsData
  await handleSubmit(JSON.stringify(listOption))
  ElMessage({type: 'success',message: '修改成功!'})
  editItemFormVisible.value = false
}



// 查询配置文件信息更新页面
async function resetListOption(){
  const data = await commUtil.getTextResource('jsonListOptions.json')
  let listOption = JSON.parse(data)
  const listOptionMap = Object.keys(listOption).map(key => {
    return {
      label: key,
      value: JSON.stringify(listOption[key])
    }
  })
  emit('listOption', listOptionMap)
  console.log('listData',listOptionMap);
  listData.value = listOptionMap
}


function removeDomain(item) {
  const index = editFormData.value.domains.indexOf(item)
  if (index !== -1) {
    editFormData.value.domains.splice(index, 1)
  }
}

function addDomain(){
  editFormData.value.domains.push({
    dataIndex: '',
    value: '',
  })
}
</script>

<template>
    <div>
      <el-button @click="dialogVisible = true"><slot></slot></el-button>
  
      <el-dialog
        title="列表与操作"
        v-model="dialogVisible"
        width="50%"
        @close="handleClose"
      >
        <el-table
          :data="listData"
          style="width: 100%"
        >
          <el-table-column
            prop="label"
            label="类型"
          />
          <el-table-column
            label="操作"
          >
            <template v-slot="scope">
                <el-button type="danger" :icon="Delete" circle @click="handleDelete(scope.$index, scope.row)" />
                <el-button type="primary" :icon="Edit" circle @click="showEditItemForm(scope.$index, scope.row)" />
            </template>
          </el-table-column>
        </el-table>
  
        <div slot="footer" class="dialog-footer">
          <el-button @click="dialogVisible = false">关闭</el-button>
          <el-button type="primary" @click="showAddTypeForm">新增类型</el-button>
        </div>
      </el-dialog>



      <!-- 新增表单 -->
      <el-dialog
          title="新增类型"
          v-model="addTypeFormVisible"
          width="60%"
          @close="addTypeFormVisible = false"
        >
        <el-form :model="addTypeFormData" label-width="80px">
            <el-form-item label="类型名称">
              <el-input v-model="addTypeFormData.keyname"></el-input>
            </el-form-item>
            <!-- 可以添加更多输入框 -->
          </el-form>
          <div slot="footer" class="dialog-footer">
            <el-button @click="addTypeFormVisible = false">取消</el-button>
            <el-button type="primary" @click="handleAddTypeData(addTypeFormData)">确定</el-button>
          </div>
        </el-dialog>

      <!-- 新增表单 -->
      <el-dialog
          title="修改项"
          v-model="editItemFormVisible"
          width="700px"
          @close="editItemFormVisible = false"
        >
        <el-form :model="editFormData" label-width="80px">
          <el-form-item
            v-for="(domain,index) in editFormData.domains"
            :key="index"
            :label="index + ''"
            :prop="'domains.' + index + '.dataIndex'"
            :rules="{
              required: true,
              message: 'domain can not be null',
              trigger: 'blur',
            }"
          >
          <el-row :gutter="20">
            <el-col :span="11">
              <el-input v-model="domain.title" placeholder="中文含义" />
            </el-col>
            <el-col :span="11">
              <el-input v-model="domain.dataIndex" placeholder="对应字段名" />
            </el-col>
            <el-col :span="2">
              <el-button size="small" type="danger" class="mt-2" :icon="RemoveFilled" @click.prevent="removeDomain(domain)">
                Delete
              </el-button>
            </el-col>
          </el-row>
          </el-form-item>        
        </el-form>
          <div slot="footer" class="dialog-footer">
            <el-button type="primary" :icon="Plus" @click="addDomain">添加项目</el-button>
            <el-button type="success" @click="handleEditItemData(editFormData.domains)">确定</el-button>
            <el-button @click="editItemFormVisible = false">取消</el-button>
          </div>
        </el-dialog>
    </div>
  </template>