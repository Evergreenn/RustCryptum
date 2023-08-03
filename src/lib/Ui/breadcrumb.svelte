<script lang="ts">
	import { useStateStore } from '$lib/stores/stateStore';

	let breadcrumb: any = '';
	let statestore = useStateStore();

	$: {
		breadcrumb = $statestore.breadcrumb;
	}
</script>

{breadcrumb}
<!-- {#if i === breadcrumbs.length - 1} -->
<!-- 	<span class="text-gray-500">{breadcrumb}</span> -->
<!-- {:else} -->
<!-- 	<a href="#" class="text-gray-500 hover:text-gray-700">{breadcrumb}</a> -->
<!-- {/if} -->
<!-- {#if i !== breadcrumbs.length - 1} -->
<!-- 	<svg -->
<!-- 		xmlns="http://www.w3.org/2000/svg" -->
<!-- 		fill="none" -->
<!-- 		viewBox="0 0 24 24" -->
<!-- 		stroke="currentColor" -->
<!-- 		class="w-4 h-4 inline-block" -->
<!-- 	> -->
<!-- 		<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /> -->
<!-- 	</svg> -->
<!-- {/if} -->
