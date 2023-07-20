<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let greetMsg = '';

	async function getKeys() {
		greetMsg = await invoke('get_keys');
	}
	console.log(getKeys());
</script>

<!-- <div> -->
<!-- 	<input id="greet-input" placeholder="Enter a name..." bind:value={name} /> -->
<!-- 	<button on:click={greet}>Greet</button> -->
<!-- 	<p>{greetMsg}</p> -->
<!-- </div> -->

<style lang="postcss">
	button {
		@apply font-bold py-2 px-6 rounded;
		@apply bg-green-500 text-white;
	}

	button:hover {
		@apply bg-blue-700;
	}
</style>
