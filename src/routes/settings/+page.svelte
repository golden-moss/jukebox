<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { invoke } from '@tauri-apps/api/tauri';

	export let data: PageData;

	interface GlobalSettings {
		folder_to_scan: string;
		volume: number;
	}

	let globalSettings: GlobalSettings;

	async function get_global_settings() {
		// console.log('toggle_playback called');
		globalSettings = await invoke('get_global_settings').then((res) => res);
	}

	async function update_global_settings() {
		await invoke('update_global_settings', { globalSettings });
	}

	onMount(() => {
		get_global_settings();
	});
</script>

<section>
	<h1>Settings</h1>
	<button on:click={get_global_settings}>get</button>
	<div>
		{#if globalSettings}
			<div>Folder to scan: {globalSettings.folder_to_scan}</div>
			<div>Volume: {globalSettings.volume}</div>
		{/if}
		<!-- {#each globalSettings as setting} -->
		<!-- <div>{setting}</div> -->
		<!-- {/each} -->
	</div>
	<button on:click={update_global_settings}>Update</button>
</section>
