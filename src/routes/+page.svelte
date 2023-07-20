<script>
	// import Greet from '../lib/Greet.svelte';
	import { AppBar } from '@skeletonlabs/skeleton';
	import { AppShell } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';
	import SidebarLeft from '$lib/sidebar/SidebarLeft.svelte';

	import Greet from '$lib/Greet.svelte';
</script>

<AppShell>
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">(icon)</svelte:fragment>
			<svelte:fragment slot="trail">
				<LightSwitch />
			</svelte:fragment>
			<svelte:fragment slot="headline">(headline)</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<SidebarLeft />
	</svelte:fragment>

	<!-- (sidebarRight) -->
	<!-- (pageHeader) -->
	<!-- Router Slot -->
	<slot />
	<Greet />
	<!-- ---- / ---- -->
	<svelte:fragment slot="pageFooter">Page Footer</svelte:fragment>
	<!-- (footer) -->
</AppShell>

<!-- <h1>Welcome to SvelteKit</h1> -->
<!-- <LightSwitch /> -->
<!-- <AppBar> -->
<!-- 	<svelte:fragment slot="lead">(icon)</svelte:fragment> -->
<!-- 	<svelte:fragment slot="trail">(actions)</svelte:fragment> -->
<!-- 	<svelte:fragment slot="headline">(headline)</svelte:fragment> -->
<!-- </AppBar> -->
<!-- <Greet /> -->

<!-- <style lang="postcss"> -->
<!-- 	h1 { -->
<!-- 		@apply text-4xl font-bold; -->
<!-- 	} -->
<!-- </style> -->
