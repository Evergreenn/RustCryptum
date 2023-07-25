<script lang="ts">
	// import Greet from '$lib/Greet.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { emit, listen } from '@tauri-apps/api/event';
	import { writable } from 'svelte/store';

	let keysStore = writable();

	interface Keys {
		id: string;
		name: string;
		password: string;
		created_at: string;
		updated_at: string;
		last_used_at: string;
		last_changed_at: string;
	}

	let keys: Array<Keys> = [];
	// let tmp;

	// $: console.log(tmp);

	invoke('get_keys')
		.then((res) => {
			// tmp = [...res];
			console.log('oui');
			console.log(res);
		})
		.catch((err) => {
			console.log(err);
		});

	listen('key_created', (event) => {
		invoke('get_keys').then((res) => {
			// keys = [...res];
		});
	});
</script>

<!-- <div class="flex flex-col ..."> -->
<div class="grid grid-cols-2 gap-4">
	{#each keys as key}
		<div class="card p-4 m-6 card-hover">
			<header class="card-header">{key.name}</header>
			<section class="p-4">{key.password}</section>
			<footer class="card-footer">(footer)</footer>
			<br />
		</div>
		<!-- <a class="block card card-hover p-4" href="/elements/cards">{key.name}</a> -->
	{/each}
</div>
