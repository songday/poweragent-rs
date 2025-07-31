<script>
	import { Position, useSvelteFlow, Handle } from '@xyflow/svelte';
	let { isConnectable, id, data } = $props();
	let { updateNodeData } = useSvelteFlow();
	// let isConnectable = $derived(connections.current.length === 0);
</script>

<Handle type="target" position={Position.Left} {isConnectable} />
<div
	class="text-updater-node"
	style="height: 200px;"
	ondblclick={() => console.log("You've double clicked me!!!!")}
	role="presentation"
>
	<div>
		<label for="text">Text:</label>
		<input
			id="text"
			name="text"
			value={data.text}
			oninput={(evt) => {
				updateNodeData(id, { text: evt.target.value });
			}}
			class="nodrag"
		/>
	</div>
</div>
<Handle
	id="mys-handle-1"
	type="source"
	position={Position.Right}
	{isConnectable}
	style="top: 0; background: #555"
>
	<div class="handletext">Mytext</div>
</Handle>
<Handle
	id="mys-handle-2"
	type="source"
	position={Position.Right}
	{isConnectable}
	style="top: 20%; background: #555"
/>
<Handle
	id="mys-handle-3"
	type="source"
	position={Position.Right}
	{isConnectable}
	style="top: 40%; background: #555"
/>
<Handle
	id="mys-handle-4"
	type="source"
	position={Position.Right}
	{isConnectable}
	style="top: 60%; background: #555"
/>
<Handle
	id="mys-handle-5"
	type="source"
	position={Position.Right}
	{isConnectable}
	style="top: 80%; background: #555"
/>

<style>
	.handletext {
		position: relative;
		left: -100px;
		width: 200px;
		transform: translate(10px, -40%);
		font-size: 12px;
		pointer-events: none;
	}
</style>
