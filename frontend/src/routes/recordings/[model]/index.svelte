<script lang="ts">
	import { page } from '$app/stores';
	import RecordingInfo from '$lib/components/RecordingInfo.svelte';
	import { navbarElement } from '$lib/stores';
	import { onDestroy, onMount } from 'svelte';
	import { User } from 'svelte-hero-icons';

	$navbarElement = {
		icon: User,
		name: $page.params.model,
		path: $page.url.pathname
	};

	onDestroy(() => ($navbarElement = undefined));

	let recordings: { name: string; duration: number }[] = [];

	onMount(async () => {
		const res = await fetch(`/api/model/${$page.params.model}/videos`);
		if (res.status != 200) return;
		recordings = (await res.json())['videos'];
	});
	let selectedVideo: string | undefined = undefined;
	const deleteVideo = async (video: string) => {
		const res = await fetch(`/api/model/${$page.params.model}/videos`, {
			method: 'DELETE',
			body: JSON.stringify({
				video
			})
		});
		if (!res.ok) return;
		if (selectedVideo == video) selectedVideo = undefined;
		recordings = recordings.filter((recording) => recording.name != video);
	};
	const renameVideo = async (video: string, newName: string) => {
		const res = await fetch(`/api/model/${$page.params.model}/videos`, {
			method: 'POST',
			body: JSON.stringify({
				video,
				newName
			})
		});
		if (!res.ok) return;
		if (selectedVideo == video) selectedVideo = newName;
		recordings = recordings.map((recording) => {
			return {
				...recording,
				name: recording.name == video ? newName : recording.name
			};
		});
	};
</script>

<div class="container">
	{#if selectedVideo}
		<video
			width="100%"
			src={`/api/model/${$page.params.model}/video.mp4?video=${selectedVideo}`}
			poster={`/api/model/${$page.params.model}/thumbnail.png?video=${selectedVideo}`}
			controls
			muted
		>
			<track kind="captions" />
		</video>
	{/if}
	<div class="recordings" class:side={selectedVideo}>
		{#each recordings as recording}
			<RecordingInfo
				model={$page.params.model}
				video={recording.name}
				durationInSeconds={recording.duration}
				selected={selectedVideo == recording.name}
				onClick={(video) => {
					if (!selectedVideo)
						setTimeout(
							() => document.getElementById(recording.name)?.scrollIntoView({ behavior: 'smooth' }),
							10
						);
					selectedVideo = selectedVideo == video ? undefined : video;
				}}
				onDelete={deleteVideo}
				onRename={renameVideo}
			/>
		{/each}
	</div>
</div>

<style>
	.container {
		display: flex;
		flex-wrap: wrap;
		flex-direction: column;
		flex: 1;
		overflow: hidden;
	}
	.recordings {
		flex: 5;
		overflow: auto;
	}
	video {
		flex: 1;
	}

	@media (min-width: 640px) {
		.container {
			flex-direction: row;
			flex-wrap: nowrap;
		}
		.recordings {
			display: flex;
			flex-wrap: wrap;
			flex-direction: row;
			align-items: flex-start;
			align-content: flex-start;
		}
		.side {
			display: block;
			min-width: 500px;
			order: 1;
			flex: 0 1 auto;
		}
		video {
			flex: 1;
			order: 2;
			width: 100%;
			height: 100%;
			object-fit: contain;
			overflow: auto;
		}
	}
</style>
