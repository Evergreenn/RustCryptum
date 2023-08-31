<script lang="ts">
	import { formatTitle } from '$lib/utils';

	// enum TitleWeight {
	// 	h1 = 'h1',
	// 	h2 = 'h2',
	// 	h3 = 'h3',
	// 	h4 = 'h4',
	// 	h5 = 'h5',
	// 	h6 = 'h6'
	// }
	export let title: string;
	export let weight: string;
	export let divider: boolean = false;

	$: title = formatTitle(title);
</script>

{#if weight === 'h1'}
	<h1 class="h1">{title}</h1>
{:else if weight === 'h2'}
	<h2 class="h2">{title}</h2>
{:else if weight === 'h3'}
	<h3 class="h3">{title}</h3>
{:else if weight === 'h4'}
	<h4 class="h4">{title}</h4>
{:else if weight === 'h5'}
	<h5 class="h5">{title}</h5>
{:else if weight === 'h6'}
	<h6 class="h6">{title}</h6>
{:else}
	<h6 class="h6">{title}</h6>
{/if}
{#if divider}
	<hr class="!border-t-4 mt-4 mb-8" />
{/if}
