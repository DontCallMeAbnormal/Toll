<script setup>
import { ref, computed } from "vue";
const jsonData = ref('')

const jsonStr = computed(() => {
    if (!jsonData.value) {
        return { "请录入json字符串": "" }
    }
    try {
        return JSON.parse(jsonData.value)
    } catch (e) {
        return { "解析错误": `${e.toString()}` }
    }
})
</script>
<link rel="stylesheet" href="jquery.jsonview.css" />
<template>
    <el-row :gutter="5">
        <el-col :span="24">
            <el-card class="box-card" header="录入JSON数据以格式化">
                <el-input v-model="jsonData" :autosize="{ minRows: 2, maxRows: 4 }" type="textarea"
                    placeholder="输入JSON数据" />
            </el-card>
        </el-col>
    </el-row>
    <el-row :gutter="5">
        <el-col :span="24">
            <el-card class="box-card" header="预览">
                <json-viewer :value="jsonStr" copyable boxed sort />
            </el-card>
        </el-col>
    </el-row>
</template>