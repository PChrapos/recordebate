<script>
	import NavBar from '$lib/components/NavBar.svelte';

	let primaryColor = '#202124';
	let primaryColor2 = '#1e1e1e';
	let secondaryColor = '#171717';
	let secondaryColor2 = 'black';
	let tertiaryColor = '#7A0BC0';
	let tertiaryColor2 = '#FA58B6';
</script>

<div
	class="container"
	style="--p-color: {primaryColor}; --s-color: {secondaryColor}; --t-color: {tertiaryColor}; --p-color2: {primaryColor2}; --s-color2: {secondaryColor2}; --t-color2: {tertiaryColor2}"
>
	<NavBar />
	<slot />
</div>

<style>
	:global(*) {
		--primary-color: var(--p-color);
		--primary-color2: var(--p-color2);
		--secondary-color: var(--s-color);
		--secondary-color2: var(--s-color2);
		--tertiary-color: var(--t-color);
		--tertiary-color2: var(--t-color2);
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
		color: var(--secondary-color);
		background-color: var(--primary-color);
		display: flex;
		flex-direction: column;
		font-weight: 600;
	}
</style>
