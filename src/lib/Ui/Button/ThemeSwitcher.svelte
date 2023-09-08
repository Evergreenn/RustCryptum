<script lang="ts">
	import SmallCard from '../SmallCard.svelte';
	import Placeholder from '../Placeholder.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { firstUpper } from '$lib/utils';

	const theme = ['tealLightning', 'crimson', 'modern', 'wintry', 'vintage'];

	const onClickTheme = (item: string) => {
		const body = document.body;
		body.dataset.theme = item;
		item = firstUpper(item);
		invoke('set_theme', { theme: item });
	};
</script>

<div class="grid grid-cols-1 gap-4 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
	{#each theme as item}
		<SmallCard title={item} pointer={true} onClick={() => onClickTheme(item)}>
			<span slot="body">
				<Placeholder color={item} />
			</span>
		</SmallCard>
	{/each}
</div>
