<script lang="ts">
	import { onMount } from 'svelte';
	import Title from './Title/Title.svelte';
	import { modeCurrent } from '@skeletonlabs/skeleton';

	export let title: string;
	export let body: string = '';
	export let centered: boolean = true;
	export let onClick: any = undefined;
	export let pointer: boolean = false;

	let isLightMode;
	$: {
		isLightMode = $modeCurrent;
		if (pointer) {
			if (isLightMode) {
				classCard = 'card max-w-md border border-secondary-500 cursor-pointer hover:shadow-xl';
			} else {
				classCard = 'card max-w-md border border-secondary-500 cursor-pointer hover:bg-surface-700';
			}
		} else {
			classCard = 'card max-w-md border border-secondary-500';
		}
	}

	let classCard: string = '';
	let element: HTMLElement;

	onMount(() => {
		if (onClick) {
			element.addEventListener('click', onClick);
		}
	});

	if (centered) {
		body = `<div class="flex justify-center items-center">${body}</div>`;
	} else {
		body = `<div class="">${body}</div>`;
	}
</script>

<div class={classCard} bind:this={element}>
	<section class="p-4">
		<div class="flex flex-col gap-2">
			<div class="text-sm text-gray-500">
				<Title {title} weight={'h6'} />
			</div>
			{@html body}
			{#if $$slots.body}
				<slot name="body" />
			{/if}
		</div>
	</section>
</div>
