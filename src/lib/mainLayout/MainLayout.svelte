<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { emit, listen } from '@tauri-apps/api/event';
	import Card from '$lib/Ui/Card.svelte';

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
	export let database = {};
	export let currentTile = 0;
	let currentGroup: any = [];

	listen('key_created', (event) => {
		invoke('get_keys').then((res) => {
			// keys = [...res];
		});
	});

	const findRecursivlyInArray = (array: [], key) => {
		let result = [];
		for (let i = 0; i < array.length; i++) {
			if (array[i].uuid === key) {
				result.push(array[i]);
			} else {
				if (array[i].groups.length > 0) {
					result = result.concat(findRecursivlyInArray(array[i].groups, key));
				}
			}
		}
		return result;
	};

	$: {
		currentGroup = findRecursivlyInArray(database.groups, currentTile);
		console.log(currentGroup);
	}
</script>

<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 mr-2">
	{#if currentGroup.length > 0}
		{#each currentGroup[0].entries as field}
			<Card>
				<span slot="title">{field.card.title}</span>
				<span slot="body">
					<div class="flex flex-col">
						<input class="input" type="text" value={field.card.username} disabled />
						<input class="input" type="password" value={field.card.password} disabled />
					</div>
				</span>
			</Card>
		{/each}
	{/if}
</div>
