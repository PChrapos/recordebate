<script lang="ts">
	import { onMount } from 'svelte';
	import { Icon, StatusOnline, XCircle } from 'svelte-hero-icons';

	export let model: string;
	export let onDelete: ((name: string) => void) | undefined = undefined;

	const baseModelImage = `https://roomimg.stream.highwebmedia.com/stream?room=${model}`;
	let modelImage = baseModelImage;
	let isOnline = false;

	const refresh = async () => {
		const timestamp = Date.now();
		modelImage = `${baseModelImage}&${timestamp}`;
		try {
			const res = await fetch(modelImage);
			console.log(res.status);
			isOnline = res.status == 200;
		} catch {
			isOnline = false;
		}
		if (!isOnline) {
			modelImage = `/api/model/${model}/thumbnail.png`;
			setTimeout(refresh, 300000);
		} else {
			setTimeout(refresh, 60000);
		}
	};
	onMount(refresh);
</script>

<div class="container">
	<div>
		<div class="onlineStatus" class:online={isOnline}>
			<Icon src={StatusOnline} />
		</div>
		<div class="modelName">{model}</div>
		<button
			class="modelDelete"
			on:click={() => onDelete && onDelete(model)}
			hidden={onDelete === undefined}
		>
			<Icon src={XCircle} />
		</button>
	</div>
	<a href={`/recordings/${model}`}>
		<img src={modelImage} alt={model} />
	</a>
</div>

<style>
	.container {
		margin: 10px;
		background-color: var(--primary-color);
		border-radius: 20px;
		padding: 5px;
		width: 100%;
	}
	img {
		width: 100%;
		border-radius: 20px;
		display: block;
	}
	div > div {
		display: flex;
		align-items: center;
	}
	.onlineStatus {
		flex: 1;
		margin-left: 2px;
		color: grey;
		width: 7px;
	}
	.online {
		color: green;
	}
	.modelName {
		flex: 15;
		margin-left: 7px;
		margin-top: 2px;
		margin-bottom: 5px;
		font-size: large;
	}
	.modelDelete {
		flex: 1;
		margin-right: 2px;
		color: var(--secondary-color);
		width: 9px;
		background: none;
		border: none;
		padding: 0;
		font: inherit;
		cursor: pointer;
		outline: inherit;
	}
	.modelDelete:hover {
		color: var(--tertiary-color);
	}
	@media (min-width: 640px) {
		.container {
			width: auto;
		}
		img {
			width: 500px;
		}
	}
</style>
