<script setup>
import { h, ref, onMounted, onUnmounted, nextTick, provide, readonly } from "vue";
import CronJobNode from "@/components/workflow/nodes/CronJobNode.vue";
import HttpReqNode from "@/components/workflow/nodes/HttpReqNode.vue";
import { Graph } from '@antv/x6';
// https://x6.antv.vision/zh/docs/tutorial/advanced/react#%E6%B8%B2%E6%9F%93-vue-%E8%8A%82%E7%82%B9
import { register, getTeleport } from "@antv/x6-vue-shape";

register({
    shape: "CronJobNode",
    width: 270,
    height: 120,
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

function addHandleNode(x, y, item) {
    // console.log('addHandleNode' + x);
    const node = graph.addNode({
        shape: item.type,
        x: x,
        y: y,
        connectable: false,
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

const TeleportContainer = getTeleport();

const nodes = [
    { name: 'CronJobNode', type: 'CronJobNode', desc: 'Cron job node', cnt: 1 },
    { name: 'HttpReqNode', type: 'HttpReqNode', desc: 'Http request node', cnt: 1 },
]
</script>

<style scoped>
#canvas {
    width: 100vw;
    height: calc(100vh - 43px);
}

.nodesBox {
    display: flex;
    flex-direction: column;
    position: absolute;
    top: 20px;
    left: 20px;
    z-index: 100;
    width: 100px;
    font-size: 9pt;
}

.node-btn {
    cursor: pointer;
    border: 1px solid #dad9d9;
    padding: 10px;
    margin-bottom: 6px;
    font-size: 9pt;
    /* width: v-bind(nodesBtnWidth); */
    width: 100px;
    background-color: white;
}

.HttpReqNode {
    border-left: 5px solid rgb(255, 196, 0);
}

.CronJobNode {
    border-left: 5px solid #EFB7BA;
}
</style>

<template>
    <div class="nodesBox">
        <div v-for="item in nodes" :key="item.type" class="node-btn" :class="item.type" draggable="true"
            @dragend="handleDragEnd($event, item)">
            <el-tooltip class="box-item" effect="dark" :content="item.desc" placement="right-start">
                <span> {{ item.name }}</span>
            </el-tooltip>
        </div>
    </div>
    <div id="canvas" @dragover="dragoverDiv" style="border: 1px #000 solid;"></div>
    <TeleportContainer />
</template>