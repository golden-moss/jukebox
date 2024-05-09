<script lang="ts">
	import type { PageData } from './$types';
	import Test from '$lib/Test.svelte';
	import Player from '$lib/Player.svelte';

	export let data: PageData;
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<p>test</p>

<Test />

<section>
	<h1>Player</h1>
	<Player />
</section>
