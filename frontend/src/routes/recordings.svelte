<script lang="ts">
	import CamInfo from '$lib/components/CamInfo.svelte';
	import { onMount } from 'svelte';

	let recordedModels: string[] = [];

	onMount(async () => {
		const res = await fetch('/api/models/recordedModels');
		recordedModels = (await res.json()).models;
	});
</script>

<div class="container">
	{#each recordedModels as model}
		<CamInfo {model} />
	{/each}
</div>

<style>
	.container {
		display: flex;
		flex-wrap: wrap;
		overflow: auto;
	}
</style>
