<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { page } from '$app/stores';
	import Addkey from '$lib/sidebar/Addkey.svelte';

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

	invoke('get_keys').then((res) => {
		keys = [...res];
	});

	$: classesActive = (href: string) => (href === $page.url.pathname ? '!bg-primary-500' : '');
</script>

<nav class="list-nav">
	<ul>
		<li>
			<a href="/" class={classesActive('/')}>
				<span class="badge bg-primary-500">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="w-6 h-6"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
						/>
					</svg>
				</span>
				<span class="flex-auto">Home</span>
			</a>
		</li>
	</ul>
</nav>

<Addkey />
