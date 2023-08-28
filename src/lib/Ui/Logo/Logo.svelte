<script lang="ts">
	import logo from '$lib/assets/logo-no-background.svg';
	import shortLogo from '$lib/assets/logo-short-no-background.png';

	let windowWidth: number = 0;
	let isMobile: boolean = false;
	let isMdScreen: boolean = false;
	let isLargeScreen: boolean = false;

	$: {
		isMobile = windowWidth < 768;
		isMdScreen = windowWidth < 1024;
		isLargeScreen = windowWidth > 1200;
		console.log(isMobile, isMdScreen, isLargeScreen);
	}
</script>

<svelte:window bind:innerWidth={windowWidth} />

{#if isMobile || isMdScreen}
	<img alt="logo" src={shortLogo} class="w-20 mt-4 mb-6" />
{:else if isLargeScreen}
	<img alt="logo" src={logo} class="w-96 mt-4 mb-6" />
{:else}
	<img alt="logo" src={logo} class=" mt-7 mb-6" />
{/if}
