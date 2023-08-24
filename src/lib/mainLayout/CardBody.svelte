<script lang="ts">
	export let entry;
	export let editmod = true;
</script>

<div class="flex flex-col">
	{#each entry.card.fields as field}
		<label class="label mb-4">
			<span>{field.key}</span>
			<input
				disabled={editmod}
				type="text"
				class="input"
				placeholder={field.key}
				value={field.value}
			/>
		</label>
	{/each}
</div>
