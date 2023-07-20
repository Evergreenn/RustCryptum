<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
</script>

<div class="flex flex-col items-center justify-center h-screen">
	<ProgressRadial
		...
		stroke={20}
		meter="stroke-primary-500"
		track="stroke-primary-500/30"
		value={undefined}
	/>
</div>
