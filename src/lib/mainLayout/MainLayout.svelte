<script>
	// import Greet from '$lib/Greet.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { emit, listen } from '@tauri-apps/api/event';
	import { writable } from 'svelte/store';

	import { AppRail, AppRailTile, AppRailAnchor } from '@skeletonlabs/skeleton';

	let keysStore = writable();

	// interface Field {
	// 	key: string;
	// 	value: string;
	// }

	// interface Entries {
	// 	fields: Array<Field>;
	// 	uuid: string;
	// 	override_url: string;
	// 	times: {
	// 		creation_time: number;
	// 		last_modification_time: number;
	// 		last_access_time: number;
	// 		expiry_time: number;
	// 		expires: boolean;
	// 		location_changed: number;
	// 		usage_count: number;
	// 	};
	// }

	// interface Database {
	// 	entries: Array<Entries>;
	// 	meta: {
	// 		database_name: string;
	// 		custom_data: Array<Field>;
	// 		database_description: string;
	// 	};
	// }
	let currentTile = 0;
	export let entries = [];

	listen('key_created', (event) => {
		invoke('get_keys').then((res) => {
			// keys = [...res];
		});
	});

	const findKeyByIndex = (index) => {
		return entries[index].fields || [];
	};
</script>

<div class="flex flex-row gap-4">
	<!-- <div class="grid grid-cols-2 gap-4"> -->
	<AppRail class="basis-1/6 h-screen" aspectRatio="1:1">
		{#each entries as key, i}
			{#each key.fields as field}
				{#if field.key === 'Title'}
					<AppRailTile
						class="m-4"
						bind:group={currentTile}
						name={i.toString()}
						value={i + 1}
						title={field.value}
					>
						<div class="grid grid-cols-2 p-4">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								class="w-6 h-6"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"
								/>
							</svg>
							{field.value}
						</div>
					</AppRailTile>
				{/if}
			{/each}
		{/each}
	</AppRail>
	{#if currentTile > 0}
		<div class="card basis-3/4 mr-6 mt-6">
			{#each findKeyByIndex(currentTile - 1) as field}
				{field.key} : {field.value} <br />
			{/each}
		</div>

		<!-- coucou -->
	{/if}
</div>
