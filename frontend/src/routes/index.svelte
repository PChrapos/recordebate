<script lang="ts">
	import AddCam from '$lib/components/AddCam.svelte';
	import CamInfo from '$lib/components/CamInfo.svelte';
	import { onMount } from 'svelte';

	let trackedModels: string[] = [];
	onMount(async () => {
		const res = await fetch('/api/models/trackedModels');
		trackedModels = (await res.json()).trackedModels;
	});
	const deleteModel = async (name: string) => {
		const res = await fetch('/api/models/trackedModels', {
			method: 'DELETE',
			body: JSON.stringify({
				name
			})
		});
		if (res.ok) {
			trackedModels = trackedModels.filter((model) => model != name);
		}
	};
	const addModel = async (name: string) => {
		console.log(name);
		if (!name) return;
		const res = await fetch('/api/models/trackedModels', {
			method: 'PUT',
			body: JSON.stringify({
				name
			})
		});
		if (res.ok) {
			trackedModels = trackedModels.filter((model) => model != name);
			trackedModels.push(name);
			trackedModels = trackedModels;
		}
	};
</script>

<div class="container">
	{#each trackedModels as model}
		<CamInfo {model} onDelete={deleteModel} />
	{/each}
	<AddCam onAdd={addModel} />
</div>

<style>
	.container {
		overflow: auto;
		flex-wrap: wrap;
		display: flex;
		align-items: flex-start;
	}
</style>
