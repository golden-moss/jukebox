<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let path: String = '';
	let tagMsg = '';
	let playbackState = '';

	async function read_tags() {
		console.log('read_tags called');
		tagMsg = await invoke('read_tags', { path });
		console.log('invoke called');
	}

	async function toggle_playback() {
		playbackState = await invoke('toggle_playback');
	}
</script>

<div>
	<input id="read-tags-input" placeholder="Enter a file..." bind:value={path} />
	<button on:click={read_tags}>Read File Tags</button>
	<p>{tagMsg}</p>
</div>

<button on:click={toggle_playback}>Toggle Playback</button>
<p>playbackState: {playbackState}</p>
