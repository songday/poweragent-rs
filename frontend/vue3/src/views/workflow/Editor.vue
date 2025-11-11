<script setup>
import { h, ref, onMounted, onUnmounted, nextTick, provide, readonly } from "vue";
import { useRoute, useRouter } from 'vue-router';
import BiArrowLeft from '~icons/bi/arrow-left'
import BiFloppy from '~icons/bi/floppy'
import BiClipboard2Check from '~icons/bi/clipboard2-check'
import BiSend from '~icons/bi/send'
import CronJobNode from "@/components/workflow/nodes/CronJobNode.vue";
import HttpReqNode from "@/components/workflow/nodes/HttpReqNode.vue";
import LlmNode from "@/components/workflow/nodes/LlmNode.vue";
import { Graph } from '@antv/x6';
// https://x6.antv.vision/zh/docs/tutorial/advanced/react#%E6%B8%B2%E6%9F%93-vue-%E8%8A%82%E7%82%B9
import { register, getTeleport } from "@antv/x6-vue-shape";
const router = useRouter();

register({
    shape: "CronJobNode",
    width: 270,
    height: 110,
    component: CronJobNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "HttpReqNode",
    width: 270,
    height: 120,
    component: HttpReqNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "LlmNode",
    width: 270,
    height: 120,
    component: LlmNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

let graph = null;

onMounted(async () => {
    // console.log("subflow onMounted");
    const canvas = document.getElementById('canvas');
    // offsetLeft = canvas.offsetLeft;
    // console.log('offsetLeft=' + offsetLeft);
    // offsetTop = canvas.offsetTop;
    // console.log('offsetTop=' + offsetTop);
    // console.log('offsetHeight=' + canvas.offsetHeight);
    graph = new Graph({
        container: canvas,
        width: '100%',
        // width: canvas.offsetWidth - 10,
        height: '100%',
        // height: canvas.offsetHeight,
        // height: 500,
        background: {
            color: "#FFF",
        },
        autoResize: false,
        connecting: {
            allowBlank: false,
            allowLoop: false,
            allowNode: true,
            allowMulti: true,
            // http://x6.antv.antgroup.com/tutorial/basic/interacting#createedge
            createEdge() {
                return this.createEdge({
                    shape: 'edge',
                    attrs: {
                        line: {
                            stroke: '#8f8f8f',
                            strokeWidth: 1,
                            targetMarker: {
                                name: 'block',
                                width: 12,
                                height: 8,
                            },
                        },
                    },
                })
            },
            validateConnection(args) {
                return args.targetCell?.store?.data?.connectable === true;
            }
        },
        // https://x6.antv.vision/zh/docs/api/graph/interaction#highlighting
        // 可以通过 graph.options.highlighting.magnetAvailable.attrs.xxx = xxx 动态修改样式。
        highlighting: {
            // 当链接桩可以被链接时，在链接桩外围渲染一个 2px 宽的红色矩形框
            magnetAvailable: {
                name: 'stroke',
                args: {
                    padding: 4,
                    attrs: {
                        'stroke-width': 2,
                        stroke: 'black',
                    }
                },
            },
        },
        panning: true,
    });
    graph.on('node:click', ({ e, x, y, node, view }) => {
        node.setTools([{
            name: 'button-remove',
            args: { x: 0, y: 0 },
        },]);
    });
    graph.on('node:mouseleave', ({ e, x, y, node, view }) => {
        if (node.hasTool("button-remove")) {
            node.removeTool("button-remove");
        }
    });
    graph.on('node:dblclick', ({ e, x, y, node, view }) => {
        node.setData({ currentTime: Date.now() });
        // editedSubFlow = true;
    });
    graph.on("edge:click", ({ e, x, y, edge, view }) => {
        edge.setTools(['button-remove']);
    });
});

onUnmounted(
    () => {
        if (graph != null)
            graph.dispose()
    }
);

const allVarNames = new Set();
provide('allVarNames', allVarNames);

function addHandleNode(x, y, item) {
    // console.log('addHandleNode' + x);
    const node = graph.addNode({
        shape: item.type,
        x: x,
        y: y,
        connectable: item.connectable,
        // tools: ["button-remove"],
    });
    item.cnt++;
    node.setData({ nodeType: item.type, nodeCnt: item.cnt });
}

function handleDragEnd(e, item) {
    const point = graph.pageToLocal(e.pageX, e.pageY);
    addHandleNode(point.x, point.y, item);
}

function dragoverDiv(ev) {
    ev.preventDefault();
}

const goBack = () => {
  router.push({ name: 'home' });
}

const TeleportContainer = getTeleport();

const nodes = [
    { name: 'Cron job node', type: 'CronJobNode', desc: 'Cron job node', cnt: 0, connectable: false },
    { name: 'Http request node', type: 'HttpReqNode', desc: 'Http request node', cnt: 0, connectable: true },
    { name: 'LLM node', type: 'LlmNode', desc: 'LLM node', cnt: 0, connectable: true },
]
</script>

<style scoped>
#canvas {
    width: 100vw;
    height: 100vh;
}

.nodesBox {
    display: flex;
    flex-direction: column;
    position: absolute;
    top: 20px;
    left: 20px;
    z-index: 100;
    width: 170px;
    font-size: 9pt;
}

.node-btn {
    cursor: pointer;
    border: 1px solid #dad9d9;
    padding: 8px;
    margin-top: 6px;
    font-size: 9pt;
    /* width: v-bind(nodesBtnWidth); */
    width: 140px;
    background-color: white;
}

.CronJobNode {
    border-left: 5px solid #EFB7BA;
}

.HttpReqNode {
    border-left: 5px solid rgb(255, 196, 0);
}

.LlmNode {
    border-left: 5px solid rgb(145, 113, 227);
}
</style>

<template>
    <div class="nodesBox">
        <div>
            <el-button circle @click="goBack">
                <el-icon size="large">
                    <BiArrowLeft />
                </el-icon>
            </el-button>
            <el-button circle>
                <el-icon size="large">
                    <BiFloppy />
                </el-icon>
            </el-button>
            <el-button circle>
                <el-icon size="large">
                    <BiClipboard2Check />
                </el-icon>
            </el-button>
            <el-button circle>
                <el-icon size="large">
                    <BiSend />
                </el-icon>
            </el-button>
        </div>
        <div v-for="item in nodes" :key="item.type" class="node-btn" :class="item.type" draggable="true"
            @dragend="handleDragEnd($event, item)">
            <el-tooltip class="box-item" effect="dark" :content="item.desc" placement="right-start">
                <span> {{ item.name }}</span>
            </el-tooltip>
        </div>
    </div>
    <div id="canvas" @dragover="dragoverDiv"></div>
    <TeleportContainer />
</template>