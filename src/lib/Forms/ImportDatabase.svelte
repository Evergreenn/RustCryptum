<script lang="ts">
	import { FileDropzone, type ToastSettings } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { Stepper, Step } from '@skeletonlabs/skeleton';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import Card from '$lib/Ui/Card.svelte';

	const toastStore = getToastStore();
	let files: FileList;
	let message: string = 'Upload or drop your .kdbx file here';
	let subMessage: string = 'Only .kdbx file are allowed';
	let selected: string;
	let password: string;

	export let toggleInit = () => {};

	const onCompleteHandler = () => {
		invoke('upload_kdbx_database', { path: selected, password })
			.then((res) => {
				toggleInit();
				console.log(res);
			})
			.catch((err) => {
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
			directory: false,
			filters: [
				{
					name: 'KeePass Database',
					extensions: ['kdbx']
				}
			]
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

<Card>
	<span slot="title">Import Database</span>
	<span slot="actions" />
	<span slot="body">
		<Stepper on:complete={onCompleteHandler}>
			<Step>
				<svelte:fragment slot="header">Import File</svelte:fragment>
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
			</Step>
			<Step>
				<svelte:fragment slot="header">Password</svelte:fragment>
				<label class="label">
					<span>Enter your keepass password</span>
					<input
						class="input"
						type="password"
						placeholder="Database password"
						bind:value={password}
					/>
				</label>
			</Step>
			<!-- ... -->
		</Stepper>
	</span>
</Card>
