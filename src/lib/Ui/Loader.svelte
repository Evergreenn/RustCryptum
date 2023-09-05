<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	export let title: string = 'Loading...';
</script>

<div class="flex items-center justify-center h-screen p-4">
	<div class="flex flex-col items-center justify-center">
		<div class="mb-4">
			<ProgressRadial size="large" value={undefined} />
		</div>
		<span class="text-2xl">{title}</span>
	</div>
</div>
