<!-- TODO progress / scrubbing bar, volume control -->

<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { playState } from '$lib/playstate';
	import { onMount } from 'svelte';
	import {
		CirclePause,
		CirclePlay,
		Repeat,
		Repeat1,
		Shuffle,
		SkipBack,
		SkipForward
	} from 'lucide-svelte';

	export let iconSize = 24;

	let localState: string;

	async function toggle_playback() {
		if ($playState) {
			playState.update(() => false);
			localState = await invoke('toggle_playback');
		} else {
			playState.update(() => true);
			localState = await invoke('toggle_playback');
		}
	}

	onMount(() => {
		// toggle_playback(); // while this does solve the (backend) problem, it is really annoying during development and potentially a bug
	});
</script>

<section>
	<div id="info">
		<hgroup>
			<h2>album</h2>
			<p>album artist</p>
		</hgroup>
		<hgroup>
			<h1>song title</h1>
			<p>song artist</p>
		</hgroup>
	</div>
	<div id="mainPlaybackControls">
		<button>
			<SkipBack />
		</button>
		<button id="play" on:click={toggle_playback}>
			{#if localState === 'Play'}
				<CirclePause size={iconSize} color="var(--orange-9)" />
			{:else}
				<CirclePlay size={iconSize} color="var(--orange-9)" />
			{/if}
		</button>
		<button>
			<SkipForward />
		</button>
	</div>
	<div id="metaPlaybackControls">
		<button>
			<Shuffle />
		</button>
		<button>
			<Repeat />
			<Repeat1 />
		</button>
	</div>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		border: solid 8px var(--orange-9);
		border-radius: 55px;
		padding: 2lvh;
	}

	#info {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		border: 1px solid var(--gray-a9);
	}

	#mainPlaybackControls {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
	}

	#metaPlaybackControls {
	}

	button#play {
		min-width: fit-content;
		height: auto;
		background: none;
		border: none;
		cursor: pointer;
		clip-path: circle(50%);
	}
</style>
