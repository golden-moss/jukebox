<!-- TODO play/pause button, progress bar, volume control -->

<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let path: String = '';
	let playState = '';

	async function toggle_playback() {
		// console.log('toggle_playback called');
		playState = await invoke('toggle_playback');
	}
</script>

<div>
	<button on:click={toggle_playback}>{playState}</button>
</div>
