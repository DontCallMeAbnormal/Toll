
<script setup>
import * as XLSX from 'xlsx'
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api'
import commUtil from '@/comm/commUtil'
import ConfigButton from './part/ConfigButton.vue'
const tableData = ref('')
const jsonParseErr = ref('')
const selectValue = ref('')
// const options = ref([])
const options = ref([
    {label:"ac51", value : "[{\"title\":\"aaz116{账户ID}\",\"dataIndex\":\"aaz116\"},{\"title\":\"aae001{年度}\",\"dataIndex\":\"aae001\"},{\"title\":\"aae262{本年单位缴费部分划入个人账户本金}\",\"dataIndex\":\"aae262\"},{\"title\":\"aae263{本年单位缴费部分划入个人账户本金所产生的利息}\",\"dataIndex\":\"aae263\"},{\"title\":\"aae264{本年个人缴费部分划入个人账户本金}\",\"dataIndex\":\"aae264\"},{\"title\":\"aae265{本年个人缴费部分划入个人账户本金所产生的利息}\",\"dataIndex\":\"aae265\"},{\"title\":\"aae852{截至本年末个人账户个人缴费部分划转累计本息}\",\"dataIndex\":\"aae852\"}]"},
    {label:"ac08", value : "[{\"title\":\"aae003(费款所属期)\",\"dataIndex\":\"aae003\"},{\"title\":\"aae794\",\"dataIndex\":\"aae794\"},{\"title\":\"aae083(个人实缴划入账户)\",\"dataIndex\":\"aae083\"},{\"title\":\"aae081(单位实缴划入账户)\",\"dataIndex\":\"aae081\"},{\"title\":\"aab191(到账日期)\",\"dataIndex\":\"aab191\"},{\"title\":\"aae819(账户ID)\",\"dataIndex\":\"aae819\"}]"},
    {label:"ic91", value : "[{\"title\":\"参保地名称\",\"dataIndex\":\"aab300\"},{\"title\":\"年度\",\"dataIndex\":\"aae001\"},{\"title\":\"缴费起始时间\",\"dataIndex\":\"aae041\"},{\"title\":\"缴费终止时间\",\"dataIndex\":\"aae042\"},{\"title\":\"月缴费基数\",\"dataIndex\":\"aae180\"},{\"title\":\"缴费月数\",\"dataIndex\":\"aae202\"},{\"title\":\"个人缴费比例\",\"dataIndex\":\"aaa041\"},{\"title\":\"单位缴费划入个人账户比例\",\"dataIndex\":\"aaa043\"},{\"title\":\"单位缴费比例\",\"dataIndex\":\"aaa042\"},{\"title\":\"当年记账金额\",\"dataIndex\":\"aae381\"},{\"title\":\"当年记账金额中的个人缴费部分\",\"dataIndex\":\"aae264\"},{\"title\":\"当年记账利息\",\"dataIndex\":\"aae269\"},{\"title\":\"当年记账利息中的个人缴费部分\",\"dataIndex\":\"aae265\"},{\"title\":\"至本年末账户累计储存额\",\"dataIndex\":\"aae382\"},{\"title\":\"至本年末账户累计储存额中的个人缴费部分\",\"dataIndex\":\"aae273\"},{\"title\":\"历年缴费流水号\",\"dataIndex\":\"aaz751\"},{\"title\":\"备注\",\"dataIndex\":\"aae013\"}]"}
])
// 表格内容
const jsonListObj = computed(() => {
    jsonParseErr.value = ''
    if (!tableData.value) {
        return undefined
    }
    try {
        const obj = JSON.parse(tableData.value)

        // 通过表格数据直接生成表头
        // tableColBoj
        let tableColBoj = null
        if(Array.isArray(obj)){
            tableColBoj = obj[0]
        }
        const keys = Object.keys(tableColBoj)
        
        const keyList = keys.map(keyName=>({title:keyName,dataIndex:keyName}))
        selectValue.value = JSON.stringify(keyList)
        // 通过表格数据直接生成表头

        if (Array.isArray(obj)) {
            return obj
        }
        throw '输入的表格内容字符串不是json数组'
    } catch (e) {
        jsonParseErr.value = e.toString()
    }
    return undefined
})

//表头内容
const jsonTableHeadListObj = computed(() => {
    jsonParseErr.value = ''
    if (!selectValue.value) {
        return undefined
    }
    try {
        const obj = JSON.parse(selectValue.value)
        if (Array.isArray(obj)) {
            return obj
        }
        throw '输入的表头字符串不是json数组'
    } catch (e) {
        jsonParseErr.value = e.toString()
    }
    return undefined
})

function exportExcel() {
    const exportData = jsonListObj.value.map(i=>{
        let newObj = {}
        jsonTableHeadListObj.value.forEach(j=>{
            Object.keys(i).filter(key=>j.dataIndex==key).forEach(key=>{
                newObj[j.title] = i[key]
            })
        })
        return newObj;
    })
    // 获取表格数据
    const ws = XLSX.utils.json_to_sheet(exportData)
    // 获取表头数据
    const ws2 = XLSX.utils.json_to_sheet(jsonTableHeadListObj.value)
    // 合并表头和表格数据
    const wb = XLSX.utils.book_new()
    XLSX.utils.book_append_sheet(wb, ws, 'Sheet1')
    XLSX.utils.book_append_sheet(wb, ws2, '表头')
    // 导出
    XLSX.writeFile(wb, 'table.xlsx')
}

</script>
<template>
    <el-row :gutter="5">
        <el-col :xs="12" :sm="12">
            <el-card class="box-card" header="录入JSON数据以解析到表格中">
                <el-input v-model="tableData" :autosize="{ minRows: 2, maxRows: 4 }" type="textarea"
                    placeholder="输入json数组来解析" />
            </el-card>
        </el-col>
        <el-col :xs="12" :sm="12">
            <el-card class="box-card" header="录入表格的表头和对应的key值" >
                <el-row>
                    <el-col :sm="18" >
                        <el-select v-model="selectValue"  placeholder="Select" >
                            <el-option v-for="(item,index) in options" :key="index" :label="item.label" :value="item.value" />
                        </el-select>
                    </el-col>
                    <el-col :sm="2" :offset="1" >
                        <el-row>
                            <ConfigButton @listOption="(d) => { options = d}  ">配置</ConfigButton>
                        </el-row>
                    </el-col>
                </el-row>
                <el-row style="margin-top: 10px;">
                    <el-col :sm="24">
                        <el-input  v-model="selectValue" :autosize="{ minRows: 2, maxRows: 4 }" type="textarea"
                            placeholder="输入json数组来解析" />
                    </el-col>
                </el-row>
            </el-card>
        </el-col>
    </el-row>
    <el-row>
        <el-col>
            {{ jsonParseErr }}
            <el-button type="primary" @click="exportExcel">导出Excel</el-button>
        </el-col>
    </el-row>
    <el-row style="margin-top: 10px;">
        <el-col>
            <el-table :data="jsonListObj" border style="width: 100%">
                <el-table-column v-for="item in jsonTableHeadListObj" :sortable="true" :prop="item.dataIndex" :label="item.title" />
            </el-table>
        </el-col>
    </el-row>
</template>