<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import { ref,reactive } from "vue";
import { ElMessage } from "element-plus";
import { Select,CloseBold } from '@element-plus/icons-vue'

const routeInfo = ref([])
// 对话框控制
const dialogVible = ref(false)
const dialogateway = ref('')
const dialogData = reactive({})

async function getNetworkInfo(){
    const resultMsg = await invoke('get_all_adapter_info');
    const map_promis = resultMsg.filter(i => i.name.indexOf('VMware') === -1).map(async i=>{
        const route_info = await invoke('get_route_info',{gateway: i.gateway,ipAddr: i.ip_address});
        return {...i, route_state: !!route_info && route_info.length > 0}
    })
    const msg = await Promise.all(map_promis)
    routeInfo.value = msg
}

// 开启网络
async function enable_route(record){
    record.route_state = !record.route_state
    const routeInfo = {
        net_taget: '0.0.0.0',
        subnet_mask: '0.0.0.0',
        gateway: record.gateway,
        net_interface: record.ip_address
    }
    if(!!record.route_state){
        ElMessage({
            message: `路由已经是启动状态`,
            type: 'warning'
        })
        return
    }
    const resultMsg = await invoke('add_route',{routeInfo});
    if(!record.gateway){
        dialogData.value = routeInfo
        dialogateway.value = record.ip_address
        dialogVible.value = true
        return
    }
    getNetworkInfo()
}

async function flushDNS(){
    try {
        const resultMsg = await invoke('flush_dns');
        ElMessage({
            message: `刷新DNS成功`,
            type: 'success'
        })
    } catch (error) {
        ElMessage({
            message: error,
            type: 'warning'
        })
    }
}

//禁用网络
async function disenable_route(record){
    record.route_state = !record.route_state
    const routeInfo = {
        net_taget: '0.0.0.0',
        subnet_mask: '0.0.0.0',
        gateway: record.gateway,
        net_interface: record.ip_address
    }
    if(!record.gateway || !record.route_state){
        ElMessage({
            message: `网关：${record.gateway} , 路由状态: ${record.route_state} 不满足禁用条件`,
            type: 'warning'
        })
        return
    }
    const resultMsg = await invoke('delete_route',{routeInfo});
    getNetworkInfo()
}

// 弹框的网络启用
async function dialog_enable_route(record,gateway){
    const routeInfo = {
        ...record.value,
        gateway
    }
    const resultMsg = await invoke('add_route',{routeInfo});
    getNetworkInfo()

    dialogData.value = {} // 清空dialogData
    dialogVible.value = false // 清空dialogData
}

</script>

<template>
    <el-button @click="getNetworkInfo" >获取网络配置信息</el-button>
    <el-button @click="flushDNS" >刷新DNS</el-button>
    <el-row :gutter="5">
        <el-col :span="24">
            <el-card class="box-card" header="网络信息">
                <el-table :data="routeInfo" stripe style="width: 100%">
                    <el-table-column prop="name" label="网卡名称"></el-table-column>
                    <el-table-column prop="ip_address" label="IP"></el-table-column>
                    <el-table-column prop="subnet_mask" label="掩码"></el-table-column>
                    <el-table-column prop="gateway" label="网关"></el-table-column>
                    <el-table-column label="路由启用状态" width="180" >
                        <template #default="scope">
                            <el-switch v-model="scope.row.route_state" 
                                @change="(val) => { val ? enable_route(scope.row) : disenable_route(scope.row) }"
                                inline-prompt
                                :active-action-icon="Select"
                                :inactive-icon="CloseBold"
                                 />
                        </template>
                    </el-table-column>
                </el-table>
            </el-card>
        </el-col>
    </el-row>
    <el-dialog v-model="dialogVible" title="输入网关" width="400">
        <el-row :gutter="5" >
            <el-col >
                <el-input v-model="dialogateway" placeholder="请输入" clearable ></el-input>
            </el-col>
        </el-row>
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary"  @click="() => dialog_enable_route(dialogData,dialogateway)" >确定</el-button>
                <el-button type="info" @click="() => dialogVible = false" >取消</el-button>
            </div>
        </template>
    </el-dialog>
</template>