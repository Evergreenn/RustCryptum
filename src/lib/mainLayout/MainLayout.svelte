<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { emit, listen } from '@tauri-apps/api/event';
	import Card from '$lib/Ui/Card.svelte';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { getToastStore } from '@skeletonlabs/skeleton';

	const toastStore = getToastStore();
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
	export let database: any = {};
	export let currentTile: number = 0;

	let currentGroup: any = [];

	const onHandleCopyClick = async (uuid: string, value: string) => {
		const item = findRecursivlyInArrayEntries(database.groups, uuid);
		if (item.length === 0) {
			return;
		}

		writeText(item[0].card[value]).then(() => {
			const formattedText = value.charAt(0).toUpperCase() + value.slice(1);
			const t = {
				message: `${formattedText} copied to clipboard !`,
				// Provide any utility or variant background style:
				background: 'variant-filled-success'
			};
			toastStore.trigger(t);
		});
	};

	listen('key_created', (event) => {
		invoke('get_keys').then((res) => {
			// keys = [...res];
		});
	});
	const findRecursivlyInArrayEntries = (array: [], key: string) => {
		let result: any = [];
		for (let i = 0; i < array.length; i++) {
			array.forEach((el: any) => {
				el.entries.forEach((e: any) => {
					if (e.uuid === key) {
						result.push(e);
					}
				});
				if (result.length === 0) {
					if (el.groups.length > 0) {
						// console.log('array[i]', array[i].groups);
						result = result.concat(findRecursivlyInArrayEntries(el.groups, key));
					}
				}
			});
		}
		return result;
	};

	const findRecursivlyInArray = (array: [], key: string) => {
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
	}
</script>

<!-- <div -->
<!-- 	class="overflow-auto pb-10 grid grid-cols-1 gap-4 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 mr-2" -->
<!-- > -->
{#if currentGroup.length > 0}
	{#each currentGroup[0].entries as field}
		<Card>
			<span slot="title">{field.card.title}</span>
			<span slot="body">
				<div class="flex flex-col gap-4">
					<div class="w-full text-token flex items-center gap-4">
						<input
							class="input"
							type="text"
							value={field.card.username}
							disabled
							data-clipboard="usenameInput"
						/>
						<button on:click={onHandleCopyClick(field.uuid, 'username')} class="btn variant-filled"
							>Copy</button
						>
					</div>
					<div class="w-full text-token flex items-center gap-4">
						<input
							class="input"
							type="password"
							value={field.card.password}
							disabled
							data-clipboard="passwordInput"
						/>
						<button on:click={onHandleCopyClick(field.uuid, 'password')} class="btn variant-filled"
							>Copy</button
						>
					</div>
				</div>
			</span>
			<span slot="footer">
				<div class="flex flex-row justify-end sm:max-lg:flex-col gap-4">
					<!-- <button type="button" class="btn variant-filled-primary">Copy Password</button> -->
					<a href={'/card/' + field.uuid} class="btn variant-filled-primary">More</a>
				</div>
			</span>
		</Card>
	{/each}
{/if}
<!-- </div> -->
