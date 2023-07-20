<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let greetMsg = '';

	async function greet() {
		greetMsg = await invoke('greet', { name });
	}
</script>

<div>
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<button on:click={greet}>Greet</button>
	<p>{greetMsg}</p>
</div>

<style lang="postcss">
	button {
		@apply font-bold py-2 px-6 rounded;
		@apply bg-blue-500 text-white;
	}

	button:hover {
		@apply bg-blue-700;
	}
</style>
