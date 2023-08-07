<script lang="ts">
	import { useStateStore } from '$lib/stores/stateStore';

	let breadcrumb: any = '';
	let statestore = useStateStore();

	$: {
		breadcrumb = $statestore.breadcrumb;
	}
</script>

{breadcrumb}
