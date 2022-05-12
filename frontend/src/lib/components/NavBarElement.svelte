<script lang="ts">
	import { page } from '$app/stores';

	import { Icon } from 'svelte-hero-icons';
	export let IconType: any;
	export let name: string = '';
	export let path: string = '/';
</script>

<a href={path}>
	<div class:selected={$page.url.pathname == path}>
		<div>
			<Icon src={IconType} />
		</div>
		<span>{name}</span>
	</div>
</a>

<style>
	a {
		text-decoration: none;
		color: var(--text-color);
	}
	div {
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 18px;
		line-height: 28px;
	}
	a > div:hover:not(.selected) {
		filter: brightness(65%);
	}
	.selected {
		color: var(--tertiary-color);
	}
	div div {
		width: 30px;
		margin: 6px;
	}
	div span {
		display: none;
		/* display: inline; */
	}
	@media (min-width: 640px) {
		div span {
			display: inline;
		}
	}
</style>
