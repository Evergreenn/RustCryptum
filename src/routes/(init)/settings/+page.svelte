<script lang="ts">
	import { goto } from '$app/navigation';

	const onClick = () => {
		goto('/home');
	};
</script>

settings
<button
	on:click={onClick}
	class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
>
	Click me
</button>
