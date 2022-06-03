<script lang="ts">
	import { Check, Icon, Pencil, X, XCircle } from 'svelte-hero-icons';

	export let model: string;
	export let video: string;
	export let selected = false;
	export let durationInSeconds = 0;
	export let onClick: ((video: string) => void) | undefined = undefined;
	export let onDelete: ((video: string) => void) | undefined = undefined;
	export let onRename: ((video: string, newName: string) => void) | undefined = undefined;
	let videoDisplayName = video.endsWith('.mp4') ? video.substring(0, video.length - 4) : video;
	$: videoDisplayName = video.endsWith('.mp4') ? video.substring(0, video.length - 4) : video;
	let edit = false;
	let newValue = videoDisplayName;
	let hovered = false;
</script>

<div class="container" class:selected id={video}>
	<div
		class="titleBar"
		on:mouseenter={() => (hovered = true)}
		on:mouseleave={() => (hovered = false)}
	>
		{#if !edit}
			<button class="title" on:click={() => onClick && onClick(video)}>
				{videoDisplayName}
			</button>
		{:else}
			<form
				on:submit={() => {
					edit = false;
					onRename && onRename(video, newValue.endsWith('.mp4') ? newValue : newValue + '.mp4');
				}}
			>
				<input class="title" bind:value={newValue} />
				<button class="icon">
					<Icon src={Check} />
				</button>
			</form>
			<button
				class="icon"
				on:click={() => {
					edit = false;
					newValue = videoDisplayName;
				}}
			>
				<Icon src={X} />
			</button>
		{/if}
		{#if (selected || hovered) && !edit}
			<button class="icon" on:click={() => (edit = !edit)}>
				<Icon src={Pencil} />
			</button>
		{/if}
		<button class="icon" on:click={() => onDelete && onDelete(video)}>
			<Icon src={XCircle} />
		</button>
	</div>
	<button on:click={() => onClick && onClick(video)}>
		<div class="duration">{new Date(durationInSeconds * 1000).toISOString().substring(11, 19)}</div>
		<img src={`/api/model/${model}/thumbnail.png?video=${video}`} alt="" />
	</button>
</div>

<style>
	.container {
		margin: 4px;
		background-color: var(--secondary-color);
		border-radius: 20px;
		padding: 5px;
		color: var(--tertiary-color);
		box-shadow: 0 0 10px var(--secondary-color2);
	}
	img {
		width: auto;
		height: auto;
		border-radius: 20px;
		display: block;
		width: 100%;
	}
	.duration {
		position: absolute;
		margin-left: 10px;
		margin-top: 3px;
		color: var(--tertiary-color);
	}
	.titleBar {
		display: flex;
		align-items: center;
		font-size: 17px;
	}
	form {
		display: flex;
		flex: 1;
	}
	.title {
		flex: 1;
		margin-left: 7px;
		margin-top: 2px;
		margin-bottom: 5px;
		color: inherit;
		text-align: start;
	}
	.icon {
		color: var(--tertiary-color);
		width: 27px;
		margin-top: 3px;
		margin-right: 2px;
	}
	.icon:hover {
		color: var(--tertiary-color2);
	}
	button {
		display: block;
		background: none;
		border: none;
		padding: 0;
		font: inherit;
		cursor: pointer;
		outline: inherit;
		position: relative;
	}
	@media (min-width: 640px) {
		.container {
			max-width: 485px;
		}
	}
	.selected {
		color: var(--tertiary-color2);
	}
	input {
		background: transparent;
		border: none;
		font-size: inherit;
		font-style: inherit;
		font-family: inherit;
		font-weight: inherit;
		padding-left: 0px;
	}
</style>
