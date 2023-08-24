<script lang="ts">
	import { page } from '$app/stores';
	import BackButton from '$lib/Ui/Button/BackButton.svelte';
	import Button from '$lib/Ui/Button/Button.svelte';
	import Card from '$lib/Ui/Card.svelte';
	import Loader from '$lib/Ui/Loader.svelte';
	import SmallCard from '$lib/Ui/SmallCard.svelte';
	import Edit from '$lib/icons/Edit.svelte';
	import CardBody from '$lib/mainLayout/CardBody.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	const id = $page.params.slug;

	let promise = invoke('get_one_key', { id }).then((res) => {
		return res;
	});

	let editmod = true;
</script>

<div class="flex flex-row justify-between">
	<BackButton />
</div>

{#await promise}
	<Loader />
{:then entry}
	<div class="mb-4">
		<Card>
			<span slot="title">{entry.card.title}</span>
			<span slot="actions">
				<Button onClick={() => (editmod = !editmod)}>
					<span slot="text">Edit</span>
					<span slot="icon"><Edit /></span>
				</Button>
			</span>
			<span slot="body">
				<CardBody bind:editmod {entry} />
			</span>
			<span slot="footer" />
		</Card>
	</div>
	<div class="grid lg:grid-cols-4 md:grid-cols-3 sm:grid-cols-2 gap-4">
		{#each Object.entries(entry.times) as [title, body]}
			<SmallCard {title} {body} />
		{/each}
	</div>
{:catch error}
	<p>{error.message}</p>
{/await}
