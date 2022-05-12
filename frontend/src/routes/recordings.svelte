<script lang="ts">
	import CamInfo from '$lib/components/CamInfo.svelte';
	import { onMount } from 'svelte';

	let recordedModels: string[] = [];

	onMount(async () => {
		const res = await fetch('/api/models/recordedModels');
		recordedModels = (await res.json()).models;
	});
</script>

<div>
	{#each recordedModels as model}
		<CamInfo {model} />
	{/each}
</div>

<style>
	div {
		display: flex;
		flex-wrap: wrap;
	}
</style>
