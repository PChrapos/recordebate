<script>
	import { navbarElement } from '$lib/stores';
	import { Bookmark, Collection, Home } from 'svelte-hero-icons';
	import NavBarElement from './NavBarElement.svelte';

	let elements = [
		{
			icon: Home,
			path: '/',
			name: 'Home'
		},
		{
			icon: Collection,
			path: '/recordings',
			name: 'Recordings'
		},
		{
			icon: Bookmark,
			path: '/following',
			name: 'Following'
		}
	];
</script>

<nav>
	{#each elements as element}
		<NavBarElement IconType={element.icon} name={element.name} path={element.path} />
	{/each}
	{#if $navbarElement}
		<NavBarElement
			IconType={$navbarElement.icon}
			name={$navbarElement.name}
			path={$navbarElement.path}
		/>
	{/if}
</nav>

<style>
	nav {
		background-color: var(--secondary-color);
		color: var(--primary-color);
		display: block;
		display: flex;
		align-items: center;
		box-shadow: 3px 3px 7px var(--secondary-color2);
	}
</style>
