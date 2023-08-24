<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	enum ButtonType {
		button = 'button',
		submit = 'submit',
		reset = 'reset'
	}
	export let type: ButtonType;
	export let variant: string = 'variant-filled-primary';
	export let onClick: () => void;
	export let disabled: boolean;
	export let isLoading: boolean;
	export let size: string;

	let isMobile: boolean = false;
	let isMdScreen: boolean = false;
	const customClassMobile = `btn-icon ${variant} ${size}`;
	const customClass = `w-full btn ${variant} ${size}`;

	let windowWidth: number = 0;

	$: {
		isMobile = windowWidth < 768;
		isMdScreen = windowWidth < 1024;
	}
</script>

<svelte:window bind:innerWidth={windowWidth} />

{#if isMobile || isMdScreen}
	<button {type} class={customClassMobile} {disabled} on:click={onClick}>
		{#if isLoading}
			<ProgressRadial width={'w-4'} stroke={40} value={undefined} />
		{:else}
			<slot name="icon" />
		{/if}
	</button>
{:else}
	<button {type} class={customClass} {disabled} on:click={onClick}>
		{#if isLoading}
			<ProgressRadial width={'w-4'} stroke={40} value={undefined} />
		{:else}
			<slot name="icon" />
			<slot name="text" />
		{/if}
	</button>
{/if}
