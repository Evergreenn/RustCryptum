<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	const id = $page.params.slug;

	console.log('card id: ', id);

	const onClick = () => {
		goto('/home');
	};
</script>

<button
	on:click={onClick}
	class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
>
	Click me
</button>
{id}
