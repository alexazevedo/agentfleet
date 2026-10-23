<script lang="ts">
  import Popover from "../Popover.svelte";

  let anchor: HTMLElement | null = $state(null);
  let open = $state(false);
</script>

<button bind:this={anchor} onclick={() => (open = !open)}>Trigger</button>
<Popover {anchor} bind:open>
  <input aria-label="Name" />
</Popover>
