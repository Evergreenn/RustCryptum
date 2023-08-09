<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { goto } from '$app/navigation';
	import { toastStore } from '@skeletonlabs/skeleton';
	import { FileDropzone, type ToastSettings } from '@skeletonlabs/skeleton';

	let files: FileList;
	let databaseName: string = '';
	let databasePassword: string = '';
	let databaseDescription: string = '';
	let isLoading: boolean = false;
	let message: string = 'Select a folder';
	let subMessage: string = 'This is where your database will be stored';
	let selected: string;

	export let toggleInit = () => {};

	const onClick = () => {
		isLoading = true;
		invoke('create_database', {
			name: databaseName,
			description: databaseDescription,
			password: databasePassword,
			path: selected
		})
			.then((res) => {
				isLoading = false;
				toggleInit();
				// goto('/home');
			})
			.catch((err) => {
				isLoading = false;
				const t: ToastSettings = {
					message: err,
					// Provide any utility or variant background style:
					background: 'variant-filled-error'
				};
				toastStore.trigger(t);
				console.log(err);
			});
	};

	const onDropzaonClickHandle = async (event: Event) => {
		event.preventDefault();

		selected = await open({
			multiple: false,
			directory: true
		});
		if (selected === null) {
			// user cancelled the selection
		} else {
			// console.log(selected);
			message = selected;
			// user selected a single file
		}
	};
</script>

<form class="relative pb-16">
	<label class="label mb-6">
		<span>Database name</span>
		<input bind:value={databaseName} class="input" type="text" placeholder="Random Database Name" />
	</label>
	<label class="label mb-6">
		<span>Database Description</span>
		<input
			bind:value={databaseDescription}
			class="input"
			type="text"
			placeholder="Random Database Description (optional)"
		/>
	</label>
	<label class="label mb-6">
		<span>Database Password</span>
		<input
			bind:value={databasePassword}
			class="input"
			type="password"
			placeholder="Random Database Password"
		/>
	</label>
	<div class="flex flex-col items-center justify-center p-4 w-full">
		<FileDropzone
			on:click={onDropzaonClickHandle}
			class="mb-8"
			name="files"
			bind:files
			multiple={false}
		>
			<svelte:fragment slot="lead">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-6 h-6 absolute top-0 left-0 right-0 mx-auto mt-4"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5"
					/>
				</svg>
			</svelte:fragment>
			<svelte:fragment slot="message">{message}</svelte:fragment>
			<svelte:fragment slot="meta">{subMessage}</svelte:fragment>
		</FileDropzone>
	</div>
	<label class="label">
		<button
			on:click={onClick}
			type="submit"
			class="btn variant-ghost-primary absolute bottom-0 l weft-0 right-0"
		>
			{#if isLoading}
				<ProgressRadial width={'w-4'} stroke={40} value={undefined} />
			{:else}
				Create
			{/if}
		</button>
	</label>
</form>
