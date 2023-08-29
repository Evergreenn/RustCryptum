<script lang="ts">
	import Button from './Button.svelte';

	const theme = ['tealLightning', 'crimson'];

	const onClickTheme = (item) => {
		const body = document.body;
		body.dataset.theme = item;
	};
</script>

<div class="flex flex-row gap-4">
	{#each theme as item}
		<button on:click={() => onClickTheme(item)} type="button" class="btn variant-filled-primary"
			>{item}</button
		>
	{/each}
</div>
