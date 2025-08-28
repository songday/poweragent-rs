<script setup>
import { inject, ref, reactive, onMounted } from 'vue';
import EpWarning from '~icons/ep/warning'
import { copyProperties } from '@/assets/tools'
import { useI18n } from 'vue-i18n'
const getNode = inject('getNode');
const { t, tm, rt } = useI18n();
const node = getNode();
node.on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
});
const formLabelWidth = '100px'
const info = ref('')
const nodeName = ref();
const nodeSetFormVisible = ref(false)

onMounted(() => {
    if (nodeData.newNode) {
        nodeData.nodeName += node.getData().nodeCnt.toString();
        nodeData.timezoneOffsetMin = 0 - (new Date()).getTimezoneOffset();
        const heightOffset = nodeName.value.offsetHeight + 60;
        const x = nodeName.value.offsetWidth - 15;
        node.addPort({
            group: 'absolute',
            args: { x: x, y: heightOffset },
            attrs: {
                text: {
                    text: t('cronJobNode.nextStep'),
                    fontSize: 12,
                },
            },
        });
        nodeData.newNode = false;
    }
    copyProperties(node.getData(), nodeData);
    setInfo();
    validate();
})

const nodeData = reactive({
    nodeName: t('cronJobNode.nodeName'),
    sec: '*',
    min: '*',
    hour: '*',
    dayOfMonth: '*',
    month: '*',
    dayOfWeek: '*',
    timezoneOffsetMin: 0,
    triggerTimestampVarName: 'TriggerTime',
    invalidMessages: [],
    newNode: true,
})

const setInfo = () => {
    let a = [];
    a.push(t('cronJobNode.settings.sec') + ': ' + nodeData.sec);
    a.push(t('cronJobNode.settings.min') + ': ' + nodeData.min);
    a.push(t('cronJobNode.settings.hour') + ': ' + nodeData.hour);
    a.push(t('cronJobNode.settings.dayOfMonth') + ': ' + nodeData.dayOfMonth);
    a.push(t('cronJobNode.settings.month') + ': ' + nodeData.month);
    a.push(t('cronJobNode.settings.dayOfWeek') + ': ' + nodeData.dayOfWeek);
    a.push(t('cronJobNode.settings.timezoneOffsetMinutes') + ': UTC' + (nodeData.timezoneOffsetMin >= 0 ? '+' : '') + nodeData.timezoneOffsetMin);
    info.value = a.join(', ')
}

const validate = () => {
    nodeData.invalidMessages = [];
    if (nodeData.nodeName == '') {
        nodeData.invalidMessages.push(t('cronJobNode.errors.nodeName'));
    }
    if (nodeData.sec == '' && nodeData.min == '' && nodeData.hour == '' && nodeData.dayOfMonth == '' && nodeData.month == '' && nodeData.dayOfWeek == '') {
        nodeData.invalidMessages.push(t('cronJobNode.errors.cron'));
    }
    nodeData.valid = nodeData.invalidMessages.length == 0;
}

const saveForm = () => {
    hideForm();
    setInfo();
    validate();
}

const hideForm = () => { nodeSetFormVisible.value = false }
</script>
<style scoped>
.nodeBox {
    border: 2px #0000000e solid;
    height: 110px;
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
        <div>{{ info }}</div>
        <!-- <teleport to="body"> -->
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="50%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form label-width="100px" :model="nodeData">
                <el-form-item :label="t('common.nodeName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item :label="t('cronJobNode.settings.sec')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.sec" />
                </el-form-item>
                <el-form-item :label="t('cronJobNode.settings.min')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.min" />
                </el-form-item>
                <el-form-item :label="t('cronJobNode.settings.hour')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.hour" />
                </el-form-item>
                <el-form-item :label="t('cronJobNode.settings.dayOfMonth')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.dayOfMonth" />
                </el-form-item>
                <el-form-item :label="t('cronJobNode.settings.month')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.month" />
                </el-form-item>
                <el-form-item :label="t('cronJobNode.settings.dayOfWeek')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.dayOfWeek" />
                </el-form-item>
                <el-form-item :label="t('cronJobNode.settings.output')" :label-width="formLabelWidth">
                </el-form-item>
                <el-form-item :label="t('cronJobNode.settings.outputTriggerTimeVarName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.triggerTimestampVarName" />
                </el-form-item>
            </el-form>
            <div class="demo-drawer__footer">
                <el-button type="primary" @click="saveForm()">{{ t('common.save') }}</el-button>
                <el-button @click="hideForm()">{{ t('common.cancel') }}</el-button>
            </div>
        </el-drawer>
        <!-- </teleport> -->
    </div>
</template>