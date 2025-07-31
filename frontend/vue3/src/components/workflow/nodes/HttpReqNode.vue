<script setup>
import { inject, ref, reactive, onMounted } from 'vue';
const getNode = inject('getNode');
const node = getNode();
const nodeName = ref();
const nodeData = reactive({
    protocal: '',
    method: '',
    url: '',
    valid: false,
    invalidMessages: [],
    newNode: true,
})
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
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="50%"
            :append-to-body="true" :destroy-on-close="true">
            <el-input v-model="data" style="max-width: 600px" placeholder="Please input url">
                <template #prepend>
                    <el-select v-model="data.protocal" placeholder="Protocol" style="width: 115px">
                        <el-option label="HTTP" value="1" />
                        <el-option label="HTTPS" value="2" />
                    </el-select>
                    <el-select v-model="data.method" placeholder="Method" style="width: 115px">
                        <el-option label="GET" value="1" />
                        <el-option label="POST" value="2" />
                        <el-option label="PUT" value="3" />
                        <el-option label="DELETE" value="4" />
                    </el-select>
                </template>
            </el-input>

            <el-form :label-position="labelPosition" label-width="100px" :model="nodeData">
                <el-form-item :label="t('common.nodeName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item label="Ending text" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.endingText" type="textarea" />
                </el-form-item>
            </el-form>
            <div class="demo-drawer__footer">
                <el-button type="primary" :loading="loading" @click="saveForm()">{{ t('.common.save') }}</el-button>
                <el-button @click="hideForm()">{{ t('common.cancel') }}</el-button>
            </div>
        </el-drawer>
        <!-- </teleport> -->
    </div>
</template>