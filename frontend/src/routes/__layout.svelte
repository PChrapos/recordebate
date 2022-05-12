<script>
	import NavBar from '$lib/components/NavBar.svelte';

	let primaryColor = 'rgb(65, 65, 65)';
	let secondaryColor = 'white';
	let tertiaryColor = 'rgb(255, 25, 25)';
	let textColor = 'white';
	let backgroundColor = 'grey';
</script>

<div
	class="container"
	style="--p-color: {primaryColor}; --s-color: {secondaryColor}; --t-color: {tertiaryColor}; --txt-color: {textColor}; --bg-color: {backgroundColor}"
>
	<NavBar />
	<slot />
</div>

<style>
	:global(*) {
		--primary-color: var(--p-color);
		--secondary-color: var(--s-color);
		--tertiary-color: var(--t-color);
		--text-color: var(--txt-color);
		--background-color: var(--bg-color);
	}
	:global(body) {
		margin: 0px;
		padding: 0px;
	}
	.container {
		position: absolute;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		overflow: hidden;
		color: var(--text-color);
		background-color: var(--background-color);
		display: flex;
		flex-direction: column;
	}
</style>
