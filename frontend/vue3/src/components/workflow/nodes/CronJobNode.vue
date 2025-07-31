<script setup>
import { inject, ref, reactive, onMounted } from 'vue';
const getNode = inject('getNode');
const node = getNode();
const nodeName = ref();

onMounted(() => {
    if (nodeData.newNode) {
        nodeData.nodeName += node.getData().nodeCnt.toString();
        const heightOffset = nodeName.value.offsetHeight + 50;
        const x = nodeName.value.offsetWidth - 15;
        node.addPort({
            group: 'absolute',
            args: { x: x, y: heightOffset },
            attrs: {
                text: {
                    text: 'Next',
                    fontSize: 12,
                },
            },
        });
        nodeData.newNode = false;
    }
    // copyProperties(node.getData(), nodeData);
})

const nodeData = reactive({
    nodeName: 'Cron job node',
    sec: '',
    min: '',
    hour: '',
    dayOfMonth: '',
    month: '',
    dayOfWeek: '',
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
    background-color: #EFB7BA;
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
            Sec Min Hour DayOfMonth Month DayOfWeek
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