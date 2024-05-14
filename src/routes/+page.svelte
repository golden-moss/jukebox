<script lang="ts">
	import type { PageData } from './$types';
	import Player from '$lib/Player.svelte';

	export let data: PageData;
</script>

<section>
	<h1>Player</h1>
	<Player iconSize={128} />
</section>
