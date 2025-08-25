<script setup>
import { inject, reactive, ref, onMounted, nextTick } from 'vue';
import { copyProperties, httpReq, getDefaultBranch } from '@/assets/tools.js'
import { useI18n } from 'vue-i18n'
import EpWarning from '~icons/ep/warning'
const { t, tm, rt } = useI18n();
const nodeData = reactive({
    nodeName: t('llmNode.nodeName'),
    modelCategory: 'LlmApis',
    llmApiProvider: 'OpenAI',
    modelId: '',
    ollamaApiUrl: 'http://localhost:11434/api/chat',
    apiKey: '',
    systemPrompt: '',
    userPrompt: '',
    contextLength: 5,
    timeoutMillis: 10000,
    valid: false,
    invalidMessages: [],
    branches: [],
    asyncReq: false,
    resultVarName: 'llmResponse',
    newNode: true,
});
const getNode = inject('getNode');
// const { ollamaModels } = inject('ollamaModels');
const nodeSetFormVisible = ref(false)
const nodeName = ref()
const formLabelWidth = '130px'

getNode().on("change:data", ({ current }) => {
    // console.log('toggled');
    // const { name, text, nodeSetFormVisible } = current;
    nodeSetFormVisible.value = true;
});

const allModels = {
    HuggingFace: [{
        modelId: '',
        modelName: '',
    }],
    Ollama: {
        apiUrl: '',
        models: [{
            modelId: 'qwen3-1.5b',
            modelName: 'QWen3 1.5b',
        }],
    },
    LlmApis: {
        OpenAI: {
            apiKey: '',
            apiUrl: '',
            models: [{
                modelId: 'GPT-4.1',
                modelName: 'GPT-4.1',
            }, {
                modelId: 'GPT-4o',
                modelName: 'GPT-4o',
            }]
        },
        DeepSeek: {
            apiKey: '',
            apiUrl: '',
            models: [{
                modelId: 'DeepSeek-R1',
                modelName: 'DeepSeek-R1',
            }, {
                modelId: 'DeepSeek-V3',
                modelName: 'DeepSeek-V3',
            }]
        },
        AliYun: {
            apiKey: '',
            apiUrl: '',
            models: [{
                modelId: 'DeepSeek-R1',
                modelName: 'DeepSeek-R1-aliyun',
            }, {
                modelId: 'DeepSeek-V3',
                modelName: 'DeepSeek-V3-aliyun',
            }]
        },
        Tencent: {
            apiKey: '',
            apiUrl: '',
            models: [{
                modelId: 'DeepSeek-R1',
                modelName: 'DeepSeek-R1-tencent',
            }, {
                modelId: 'DeepSeek-V3',
                modelName: 'DeepSeek-V3-tencent',
            }]
        },
    },
}

const llmApiProviders = Object.keys(allModels.LlmApis);
const modelOptions = reactive([])
const changedModelCategory = (v) => {
    if (v === 'HuggingFace') {
        modelOptions.splice(0, modelOptions.length, ...allModels.HuggingFace);
    }
    else if (v === 'Ollama') {
        modelOptions.splice(0, modelOptions.length, ...allModels.Ollama.models);
    }
    else if (v === 'LlmApis') {
        modelOptions.splice(0, modelOptions.length, ...allModels.LlmApis[nodeData.llmApiProvider].models);
    }
    // console.log(modelOptions);
}

onMounted(async () => {
    // console.log('llmChatNode')
    const node = getNode();
    const data = node.getData();
    console.log('data', data);
    copyProperties(data, nodeData);

    if (nodeData.newNode) {
        nodeData.nodeName += data.nodeCnt.toString();
        // node.removePorts();
        node.addPort({
            group: 'absolute',
            args: { x: nodeName.value.offsetWidth - 15, y: 104 },
            markup: [
                { tagName: "circle", selector: "bopdy" },
                { tagName: "rect", selector: "bg" }
            ],
            attrs: {
                text: {
                    text: 'Next',
                    fontSize: 12,
                },
                // https://codesandbox.io/s/port-label-viwnos?file=/src/App.tsx
                bg: {
                    ref: "text",
                    refWidth: "100%",
                    refHeight: "110%",
                    refX: "-100%",
                    refX2: -15,
                    refY: -5,
                    fill: "rgb(235,238,245)"
                }
            },
        });
    }
    nodeData.newNode = false;
    validate();
})

const validate = () => {
    const d = nodeData;
    const m = d.invalidMessages;
    m.splice(0, m.length);
    if (!d.prompt)
        m.push('Please fill out "prompt" field.');
    d.valid = m.length == 0;
}

const saveForm = () => {
    const node = getNode();
    const ports = node.getPorts();
    const branch = getDefaultBranch();
    branch.branchName = ports[0].attrs.text.text;
    branch.branchId = ports[0].id;
    branch.branchType = 'GotoAnotherNode';
    nodeData.branches.splice(0, nodeData.branches.length, branch);
    validate()
    delete nodeData.exitCondition;
    nodeData.exitCondition = {};
    const nodeExitType = nodeData.nodeExitType.replace(/exitBy/, '');
    if (nodeExitType == 'Intent')
        nodeData.exitCondition[nodeExitType] = nodeData.exitIntent;
    else if (nodeExitType == 'SpecialInputs')
        nodeData.exitCondition[nodeExitType] = nodeData.exitSpecialInputs;
    else if (nodeExitType == 'LlmResultContains')
        nodeData.exitCondition[nodeExitType] = nodeData.exitLlmResultContains;
    else if (nodeExitType == 'MaxChatTimes')
        nodeData.exitCondition[nodeExitType] = nodeData.maxChatTimes;
    nodeData.prompt = JSON.stringify([{ "role": "user", "content": nodeData.promptText },])
    if (!overrideTimeoutEnabled.value) {
        nodeData.timeoutMillis = null;
        nodeData.readTimeout = null;
    }
    delete nodeData.whenTimeoutThen;
    if (whenTimeoutThen.value == 'ResponseAlternateText') {
        nodeData.whenTimeoutThen = { ResponseAlternateText: responseAlternateText }
    } else {
        nodeData.whenTimeoutThen = whenTimeoutThen;
    }
    console.log('nodeData', nodeData);
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: false });
    updateBrief();
    hideForm();
}

const hideForm = () => {
    nodeSetFormVisible.value = false;
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
    background-color: rgb(145, 113, 227);
    color: white;
    font-weight: 500;
    font-size: 14px;
    padding: 5px;
}

.optionWidth {
    width: 110px;
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
        <div v-html="nodeData.brief"></div>
        <!-- <teleport to="body"> -->
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="70%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :label-width="formLabelWidth" :model="nodeData">
                <el-form-item :label="t('common.nodeName')" :label-width="formLabelWidth" prop="nodeName" :rules="[
                    { required: true, message: 'nodeName is required' },
                ]">
                    <el-input v-model="nodeData.nodeName" />
                </el-form-item>
                <el-form-item label="System prompt" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.systemPrompt" :rows="5" type="textarea" placeholder="" />
                </el-form-item>
                <el-form-item label="User prompt" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.userPrompt" :rows="5" type="textarea" placeholder="" />
                </el-form-item>
                <el-form-item label="LLM model" :label-width="formLabelWidth">
                    <el-radio-group v-model="nodeData.modelCategory" @change="changedModelCategory">
                        <el-radio-button label="LLM APIs" value="LlmApis" />
                        <el-radio-button label="Ollama" value="Ollama" />
                        <el-radio-button label="HuggingFace" value="HuggingFace" />
                    </el-radio-group>
                </el-form-item>
                <el-form-item v-show="nodeData.modelCategory === 'LlmApis'" label="" :label-width="formLabelWidth">
                    <el-select v-model="nodeData.llmApiProvider" @change="(v) => changedModelCategory('LlmApis')">
                        <el-option v-for="item in llmApiProviders" :key="item" :label="item" :value="item" />
                    </el-select>
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-select v-model="nodeData.modelId" placeholder="Select">
                        <el-option v-for="item in modelOptions" :key="item.modelId" :label="item.modelName"
                            :value="item.modelId" />
                    </el-select>
                </el-form-item>
                <el-form-item v-show="nodeData.modelCategory === 'Ollama'" label="API URL"
                    :label-width="formLabelWidth">
                    <el-input v-model="nodeData.ollamaApiUrl" />
                </el-form-item>
                <el-form-item v-show="nodeData.modelCategory === 'LlmApis'" label="API Key"
                    :label-width="formLabelWidth">
                    <el-input v-model="nodeData.apiKey" />
                </el-form-item>
                <el-form-item label="Context length" :label-width="formLabelWidth">
                    <el-input-number v-model="nodeData.contextLength" :min="0" :max="50" :step="5" />
                    How many chat history records will be added.
                </el-form-item>
                <el-form-item label="Sync/Async" :label-width="formLabelWidth">
                    <el-checkbox v-model="nodeData.asyncReq" label="Asynchronous" />
                </el-form-item>
                <el-form-item v-show="nodeData.modelCategory !== 'HuggingFace'" label="Timeout"
                    :label-width="formLabelWidth">
                    <el-input-number v-model="nodeData.timeoutMillis" :min="100" :max="65500" />
                </el-form-item>
                <el-form-item v-show="!nodeData.asyncReq" label="Save response to" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.resultVarName" autocomplete="on" placeholder="Enter a variable name" />
                </el-form-item>
            </el-form>
            <div>
                <el-button type="primary" @click="saveForm()">{{ t('common.save') }}</el-button>
                <el-button @click="hideForm()">{{ t('common.cancel') }}</el-button>
            </div>
        </el-drawer>
        <!-- </teleport> -->
    </div>
</template>