<script lang="ts">
	import type { PageData } from './$types';
	import { invoke } from '@tauri-apps/api/tauri';

	export let data: PageData;

	let globalSettings = '';

	async function get_global_settings() {
		// console.log('toggle_playback called');
		globalSettings = await invoke('get_global_settings');
	}

	onload = () => {
		get_global_settings();
	};
</script>

<section>
	<h1>Settings</h1>
	<div>
		{globalSettings}
	</div>
</section>
