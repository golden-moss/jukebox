<!-- TODO play/pause button, progress bar, volume control -->

<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { playState } from '$lib/playstate';
	import { onMount } from 'svelte';

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
		toggle_playback();
	});
</script>

<div>
	<button on:click={toggle_playback}>{localState}</button>
</div>
