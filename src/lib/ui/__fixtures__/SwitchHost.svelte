<script lang="ts">
  import Switch from "../Switch.svelte";

  let { onchange }: { onchange?: (v: boolean) => void } = $props();
  let checked = $state(false);
</script>

<Switch bind:checked {onchange}>
  {#snippet label()}Enable{/snippet}
</Switch>
