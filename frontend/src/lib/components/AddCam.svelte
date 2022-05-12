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
		background-color: var(--primary-color);
		border-radius: 20px;
		padding: 5px;
		width: 100%;
		font-size: large;
		display: flex;
		align-items: center;
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
		color: rgb(27, 158, 27);
	}
	@media (min-width: 640px) {
		.container {
			width: 500px;
		}
	}
</style>
