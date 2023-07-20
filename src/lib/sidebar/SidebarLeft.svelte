<script lang="ts">
	import { TreeView, TreeViewItem } from '@skeletonlabs/skeleton';
	// your script goes here
</script>

<TreeView>
	<TreeViewItem>
		<p>(item 1)</p>
		<svelte:fragment slot="children">
			<TreeViewItem>
				(Child 1)
				<svelte:fragment slot="children">
					<TreeViewItem>(Child of Child 1)</TreeViewItem>
					<TreeViewItem>(Child of Child 2)</TreeViewItem>
				</svelte:fragment>
			</TreeViewItem>
			<TreeViewItem>(Child 2)</TreeViewItem>
		</svelte:fragment>
	</TreeViewItem>
	<TreeViewItem>(item 2)</TreeViewItem>
</TreeView>
