<script lang="ts">
	import { useStateStore } from '$lib/stores/stateStore';

	let breadcrumb: any = 'Root';
	let statestore = useStateStore();

	$: {
		breadcrumb = $statestore.breadcrumb || 'Root';
	}
</script>

{breadcrumb}
