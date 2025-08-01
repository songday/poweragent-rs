<script setup>
import { inject, ref, reactive, onMounted } from 'vue';
import { useI18n } from 'vue-i18n'
import EpWarning from '~icons/ep/warning'
import { copyProperties, getDefaultBranch, httpReq } from '@/assets/tools.js'
const getNode = inject('getNode');
const { t, tm, rt } = useI18n();
const node = getNode();
node.on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
});
const formLabelWidth = '100px'
const nodeSetFormVisible = ref(false)
const paramSetFormVisible = ref(false)
const nodeName = ref();
let originAsyncReqSetting = false;
const nodeData = reactive({
    nodeName: t('httpReqNode.nodeName'),
    description: '',
    protocol: 'http://',
    method: 'GET',
    address: '',
    timeoutMilliseconds: 1500,
    postContentType: 'UrlEncoded',
    headers: [],
    queryParams: [],
    formData: [],
    requestBody: '',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/123.0',
    asyncReq: false,
    branches: [],
    valid: false,
    invalidMessages: [],
    newNode: true,
})
const param = reactive({
    name: '',
    value: '',
    valueSource: '',
})
const varDialogVisible = ref(false)
const dynamicTitle = ref('')
const activeName = ref('h')
const editIdx = ref(0)
const vars = reactive([])
const selectedVar = ref('')
const requestBodyRef = ref()

node.on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
    originAsyncReqSetting = node.getData().asyncReq;
});
const validate = () => {
    const m = nodeData.invalidMessages;
    m.splice(0, m.length);
    if (nodeData.httpApiName == '' || nodeData.httpApiId == '')
        m.push('Please choose a HTTP interface');
    if (getNode().getPortAt(0).id == '')
        m.push('Please connect "Next" to another node');
    nodeData.valid = m.length == 0;
}
const saveForm = () => {
    addBranches();
    validate();
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
    hideForm()
}
const hideForm = () => {
    nodeSetFormVisible.value = false;
}
const addBranches = () => {
    if (originAsyncReqSetting == nodeData.asyncReq)
        return;
    node.removePorts();
    nodeData.branches = [];
    if (nodeData.asyncReq) {
        node.addPort({
            group: 'absolute',
            args: { x: nodeName.value.offsetWidth - 15, y: nodeName.value.offsetHeight + 50 },
            attrs: {
                text: {
                    text: t('cronJobNode.nextStep'),
                    fontSize: 12,
                },
            },

        });
        nodeData.branches.push(getDefaultBranch())
        const port = node.getPortAt(0);
        const branch = nodeData.branches[0];
        branch.branchName = port.attrs.text.text;
        branch.branchId = port.id;
    } else {
        node.addPort({
            group: 'absolute',
            args: { x: nodeName.value.offsetWidth - 15, y: nodeName.value.offsetHeight + 40 },
            attrs: {
                text: {
                    text: t('common.successful'),
                    fontSize: 12,
                },
            },
        });
        node.addPort({
            group: 'absolute',
            args: { x: nodeName.value.offsetWidth - 15, y: nodeName.value.offsetHeight + 56 },
            attrs: {
                text: {
                    text: t('common.failed'),
                    fontSize: 12,
                },
            },
        });
        nodeData.branches.push(getDefaultBranch())
        nodeData.branches.push(getDefaultBranch())
        let port = node.getPortAt(0);
        let branch = nodeData.branches[0];
        branch.branchName = port.attrs.text.text;
        branch.branchId = port.id;
        port = node.getPortAt(1);
        branch = nodeData.branches[1];
        branch.branchName = port.attrs.text.text;
        branch.branchId = port.id;
    }
}
onMounted(async () => {
    if (nodeData.newNode) {
        nodeData.nodeName += node.getData().nodeCnt.toString();
        nodeData.newNode = false;
    }
    //     let t = await httpReq('GET', 'variable', { robotId: robotId }, null, null);
    //     if (t && t.status == 200 && t.data) {
    //         for (var x in t.data) {
    //             if (t.data.hasOwnProperty(x)) {
    //                 console.log(t.data[x])
    //                 vars.push(t.data[x]);
    //             }
    //         }
    //     }
})
const newParam = () => {
    param.name = '';
    param.value = '';
    param.valueSource = 'Val';
    editIdx.value = -1;
    const p = activeName.value;
    if (p == 'h')
        dynamicTitle.value = t('httpReqNode.dynamicTitle.h')
    else if (p == 'q')
        dynamicTitle.value = t('httpReqNode.dynamicTitle.q')
    else if (p == 'f')
        dynamicTitle.value = t('httpReqNode.dynamicTitle.f')
    paramSetFormVisible.value = true;
}
const addParam = () => {
    const p = cloneObj(param);
    const idx = editIdx.value;
    if (idx > -1) {
        if (activeName.value == 'h')
            nodeData.headers[idx] = p;
        else if (activeName.value == 'q')
            nodeData.queryParams[idx] = p;
        else if (activeName.value == 'f')
            nodeData.formData[idx] = p;
    } else {
        if (activeName.value == 'h')
            nodeData.headers.push(p);
        else if (activeName.value == 'q')
            nodeData.queryParams.push(p);
        else if (activeName.value == 'f')
            nodeData.formData.push(p);
    }
    paramSetFormVisible.value = false
}
const editParam = (idx) => {
    editIdx.value = idx;
    if (activeName.value == 'h')
        copyProperties(nodeData.headers[idx], param)
    else if (activeName.value == 'q')
        copyProperties(nodeData.queryParams[idx], param)
    else if (activeName.value == 'f')
        copyProperties(nodeData.formData[idx], param)
    paramSetFormVisible.value = true
}
const delParam = (idx) => {
    if (activeName.value == 'h')
        nodeData.headers[idx].splice(idx, 1)
    else if (activeName.value == 'q')
        nodeData.queryParams[idx].splice(idx, 1)
    else if (activeName.value == 'f')
        nodeData.formData[idx].splice(idx, 1)
}
const save = async () => {
    nodeData.protocol = nodeData.protocol.replace('://', '').toUpperCase();
    const t = await httpReq('POST', 'external/http/' + apiId, { robotId: robotId }, null, nodeData);
    // console.log(t);
    if (t && t.status == 200) {
        ElMessage({
            showClose: true,
            message: 'All data has been saved.',
            type: 'success',
        });
        goBack();
    } else {
        ElMessage({
            showClose: true,
            message: 'Oops, this is something wrong.',
            type: 'error',
        })
    }
}
const insertVar = () => {
    // console.log(requestBodyRef)
    // requestBodyRef.value.focus()
    // let cursorPosition = requestBodyRef.value.selectionStart
    // console.log(cursorPosition)
    // console.log(requestBodyRef.selectionStart)
    nodeData.requestBody += '`' + selectedVar.value + '`'
    // console.log(requestBodyRef.requestBody)
    varDialogVisible.value = false
}
const handleClick = (tab, event) => {
    // dynamicTitle.value = 'Add ' + tab.paneLabel + ' parameter';
    // console.log(dynamicTitle.value)
    console.log(tab, event)
}
const changeTab = (v) => {
    if (v != 'POST' && activeName.value == 'f')
        activeName.value = 'q'
}
</script>
<style scoped>
.nodeBox {
    border: 2px #0000000e solid;
    height: 120px;
    width: 100%;
    background-color: white;
    font-size: 12px;
}

.nodeTitle {
    background-color: rgb(255, 196, 0);
    color: white;
    font-weight: 500;
    font-size: 14px;
    padding: 5px;
}
</style>
<template>
    <div class="nodeBox">
        <div ref="nodeName" class="nodeTitle">
            {{ nodeData.nodeName }}
            <span v-show="nodeData.invalidMessages.length > 0">
                <el-tooltip class="box-item" effect="dark" :content="nodeData.invalidMessages.join('<br/>')"
                    placement="bottom" raw-content>
                    <el-icon color="red" size="16">
                        <EpWarning />
                    </el-icon>
                </el-tooltip>
            </span>
        </div>
        <!-- <teleport to="body"> -->
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="70%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :model="nodeData" label-width="90px">
                <el-form-item :label="t('common.nodeName')">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item :label="t('httpReqNode.desc')">
                    <el-input v-model="nodeData.description" maxlength="256" show-word-limit type="textarea" />
                </el-form-item>
                <el-form-item label="Method">
                    <el-select v-model="nodeData.method" placeholder="" @change="changeTab">
                        <el-option label="GET" value="GET" />
                        <el-option label="POST" value="POST" />
                    </el-select>
                </el-form-item>
                <el-form-item label="Protocol">
                    <el-select v-model="nodeData.protocol" placeholder="">
                        <el-option label="HTTP" value="http://" />
                        <el-option label="HTTPS" value="https://" />
                    </el-select>
                </el-form-item>
                <el-form-item label="Address">
                    <el-input v-model="nodeData.address">
                        <!-- <template #prepend>POST Http://</template> -->
                        <template #prepend>{{ nodeData.method }} {{ nodeData.protocol }}</template>
                    </el-input>
                </el-form-item>
                <el-form-item label="Parameters">
                    <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
                        <el-tab-pane label="Header" name="h">
                            <el-table :data="nodeData.headers" stripe style="width: 100%">
                                <el-table-column prop="name" label="Parameter name" width="300" />
                                <el-table-column prop="value" label="Parameter value" width="200" />
                                <el-table-column fixed="right" :label="tm('mainflow.table')[2]" width="270">
                                    <template #default="scope">
                                        <el-button link type="primary" size="small" @click="editParam(scope.$index)">
                                            Edit
                                        </el-button> |
                                        <el-button link type="primary" size="small"
                                            @click="delParam(scope.$index, scope.row)">
                                            Delete
                                        </el-button>
                                    </template>
                                </el-table-column>
                            </el-table>
                            <el-button type="warning" @click="newParam">+Add header</el-button>
                        </el-tab-pane>
                        <el-tab-pane label="Query parameters" name="q">
                            <el-table :data="nodeData.queryParams" stripe style="width: 100%">
                                <el-table-column prop="name" label="Parameter name" width="300" />
                                <el-table-column prop="value" label="Parameter value" width="200" />
                                <el-table-column fixed="right" :label="tm('mainflow.table')[2]" width="270">
                                    <template #default="scope">
                                        <el-button link type="primary" size="small" @click="editParam(scope.$index)">
                                            Edit
                                        </el-button> |
                                        <el-button link type="primary" size="small"
                                            @click="delParam(scope.$index, scope.row)">
                                            Delete
                                        </el-button>
                                    </template>
                                </el-table-column>
                            </el-table>
                            <el-button type="warning" @click="newParam">+Add query parameter</el-button>
                        </el-tab-pane>
                        <el-tab-pane label="Request body" name="f" v-if="nodeData.method == 'POST'">
                            Request body type:
                            <el-radio-group v-model="nodeData.postContentType" class="ml-4">
                                <el-radio value="UrlEncoded" size="large">application/x-www-form-urlencoded</el-radio>
                                <el-radio value="JSON" size="large">JSON</el-radio>
                                <!-- <el-radio value="1" size="large">multipart/form-data</el-radio> -->
                            </el-radio-group>
                            <el-table v-if="nodeData.postContentType == 'UrlEncoded'" :data="nodeData.formData" stripe
                                style="width: 100%">
                                <el-table-column prop="name" label="Parameter name" width="300" />
                                <el-table-column prop="value" label="Parameter value" width="200" />
                                <el-table-column fixed="right" :label="tm('mainflow.table')[2]" width="270">
                                    <template #default="scope">
                                        <el-button link type="primary" size="small" @click="editParam(scope.$index)">
                                            Edit
                                        </el-button> |
                                        <el-button link type="primary" size="small"
                                            @click="delParam(scope.$index, scope.row)">
                                            Delete
                                        </el-button>
                                    </template>
                                </el-table-column>
                            </el-table>
                            <el-button type="warning" v-if="nodeData.postContentType == 'UrlEncoded'"
                                @click="newParam">+Add form
                                data</el-button>
                            <!-- <div style="margin: 20px 0" /> -->
                            <el-input ref="requestBodyRef" v-if="nodeData.postContentType == 'JSON'"
                                v-model="nodeData.requestBody" maxlength="10240" placeholder="JSON" show-word-limit
                                type="textarea" />
                            <el-button type="warning" v-if="nodeData.postContentType == 'JSON'"
                                @click="varDialogVisible = true">+Insert
                                a
                                variable</el-button>
                        </el-tab-pane>
                    </el-tabs>
                </el-form-item>
                <el-form-item label="User agent">
                    <el-input v-model="nodeData.userAgent" />
                </el-form-item>
                <el-form-item label="Sync/Async" :label-width="formLabelWidth">
                    <!-- <el-switch v-model="httpApiData.asyncReq" class="mb-2" active-text="Asynchronous" inactive-text="Synchronous" /> -->
                    <!-- <input type="checkbox" id="_asyncReq_" v-model="nodeData.asyncReq"
                        :checked="nodeData.asyncReq" /><label for="_asyncReq_">Asynchronous</label> -->
                    <el-checkbox v-model="nodeData.asyncReq" label="Asynchronous" />
                </el-form-item>
                <el-form-item label="Timeout" :label-width="formLabelWidth">
                    <el-input-number v-model="nodeData.timeoutMilliseconds" :min="200" :max="600000" />
                    {{ $t('common.millis') }}
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="saveForm">{{ $t('common.save') }}</el-button>
                    <el-button @click="hideForm">{{ $t('common.cancel') }}</el-button>
                </el-form-item>
            </el-form>
        </el-drawer>
        <el-dialog v-model="paramSetFormVisible" width="60%" :append-to-body="true" :destroy-on-close="true">
            <template #header="{ close, titleId, titleClass }">
                <div class="my-header">
                    <h4 :id="titleId" :class="titleClass">{{ dynamicTitle }}</h4>
                </div>
            </template>
            <el-form :model="param">
                <el-form-item :label="t('common.name')" :label-width="formLabelWidth">
                    <el-input v-model="param.name" autocomplete="off" />
                </el-form-item>
                <el-form-item label="Value" :label-width="formLabelWidth">
                    <el-space size="default" spacer="-">
                        <el-select v-model="param.valueSource" placeholder="" style="width:150px">
                            <el-option label="Const value" value="Val" />
                            <el-option label="From a variable" value="Var" />
                        </el-select>
                        <el-input v-if="param.valueSource == 'Val'" v-model="param.value" autocomplete="off"
                            style="width:400px" />
                        <el-select v-if="param.valueSource == 'Var'" v-model="selectedVar"
                            placeholder="Select a varaible" style="width:400px">
                            <el-option v-for="item in vars" :key="item.varName" :label="item.varName"
                                :value="item.varName" />
                        </el-select>
                    </el-space>
                </el-form-item>
            </el-form>
            <template #footer>
                <el-button type="primary" @click="addParam">{{ $t('common.save') }}</el-button>
                <el-button @click="paramSetFormVisible = false">{{ $t('common.cancel') }}</el-button>
            </template>
        </el-dialog>
        <!-- </teleport> -->
    </div>
</template>