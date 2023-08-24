<script lang="ts">
	import Title from './Title/Title.svelte';

	export let title: string;
	export let body: string;
</script>

<div class="card max-w-xs border border-secondary-500">
	<section class="p-4">
		<div class="flex flex-col gap-2">
			<div class="text-sm text-gray-500">
				<Title {title} weight={'h6'} />
			</div>
			<div class="flex justify-center items-center">
				{body}
			</div>
		</div>
	</section>
</div>
