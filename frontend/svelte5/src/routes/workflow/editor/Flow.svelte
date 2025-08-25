<script>
	import { Portal } from 'svelte-teleport';
	import { SvelteFlow, MiniMap, Controls, useSvelteFlow, Background } from '@xyflow/svelte';
	// import '@xyflow/svelte/dist/base.css';
	import '@xyflow/svelte/dist/style.css';
	import TextUpdaterNode from './TextUpdaterNode.svelte';
	import PopDrawer from './PopDrawer.svelte';
	const { screenToFlowPosition } = useSvelteFlow();
	const nodeTypes = { textUpdater: TextUpdaterNode, popDrawer: PopDrawer };

	function onDragOver(event) {
		event.preventDefault();
		if (event.dataTransfer) {
			event.dataTransfer.dropEffect = 'move';
		}
	}

	function handleDragEnd(evt, item) {
		const position = screenToFlowPosition({
			x: evt.clientX,
			y: evt.clientY
		});
		console.log(evt.clientX);
		console.log(position.x);

		const newNode = {
			id: `${Math.random()}`,
			type: item,
			position,
			data: { label: `${item} node` },
			origin: [0.5, 0.0]
		};

		nodes = [...nodes, newNode];
	}

	let nodes = $state.raw([
		{
			id: '1',
			type: 'input',
			position: { x: 0, y: 0 },
			data: { label: 'Hello' }
		},
		{
			id: '2',
			type: 'output',
			position: { x: 100, y: 100 },
			data: { label: 'World' }
		},
		{
			id: 'node-1',
			type: 'popDrawer',
			position: { x: 200, y: 200 },
			data: { text: 'some text' }
		}
	]);

	let edges = $state.raw([
		{ id: 'e1-2', source: '1', target: '2', type: 'smoothstep', label: 'Hello World' }
	]);
</script>

<div style:width="100vw" style:height="100vh">
	<aside class="nodesBox">
		<div
			style="height: 50px;width:90px"
			draggable="true"
			ondragend={(event) => handleDragEnd(event, 'textUpdater')}
			role="button"
			tabindex="0"
		>
			drag me
		</div>
	</aside>
	<SvelteFlow bind:nodes bind:edges {nodeTypes} fitView zoomOnDoubleClick={false} ondragover={onDragOver}>
		<MiniMap />
		<Controls />
		<Background />
	</SvelteFlow>
</div>

<style>
	.nodesBox {
		display: flex;
		flex-direction: column;
		position: absolute;
		top: 20px;
		left: 20px;
		z-index: 100;
		width: 100px;
	}
</style>
