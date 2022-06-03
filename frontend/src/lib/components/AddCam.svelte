<script lang="ts">
	import { Icon, PlusCircle } from 'svelte-hero-icons';
	let value: string;
	export let onAdd: ((name: string) => void) | undefined = undefined;
</script>

<form
	class="container"
	on:submit|preventDefault={() => {
		onAdd && onAdd(value);
		value = '';
	}}
>
	<div class="label">Add:</div>
	<input bind:value />
	<button class="add">
		<Icon src={PlusCircle} />
	</button>
</form>

<style>
	.container {
		margin: 10px;
		background-color: var(--secondary-color);
		border-radius: 20px;
		padding: 5px;
		width: 100%;
		font-size: large;
		display: flex;
		align-items: center;
		color: var(--tertiary-color);
		box-shadow: 0 0 10px var(--secondary-color2);
	}
	.label {
		margin-left: 5px;
		margin-right: 5px;
	}
	input {
		width: 100%;
		border-radius: 10px;
		height: 20px;
		padding-left: 5px;
		font-weight: 600;
		background-color: var(--primary-color2);
		color: var(--tertiary-color2);
		border: 0;
		outline-color: var(--tertiary-color2);
		box-shadow: 0 0 3px var(--tertiary-color);
	}
	input:focus {
		outline: none;
		box-shadow: 0 0 5px var(--tertiary-color2);
	}
	.add {
		width: 30px;
		margin-left: 3px;
		color: inherit;
		background: none;
		border: none;
		padding: 0;
		font: inherit;
		cursor: pointer;
		outline: inherit;
	}
	.add :global(svg) {
		display: block;
	}
	.add:hover {
		color: var(--tertiary-color2);
	}
	@media (min-width: 640px) {
		.container {
			width: 500px;
		}
	}
</style>
