<script lang="ts">
	import BackArrow from '$lib/icons/BackArrow.svelte';

	const onClick = () => {
		window.history.back();
	};
</script>

<button type="button" class="mb-4 btn-icon variant-ringed btn-icon-sm" on:click={onClick}>
	<BackArrow />
</button>
