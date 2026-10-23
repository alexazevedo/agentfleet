<script lang="ts">
  import Tabs from "../Tabs.svelte";

  const tabs = [
    { id: "a", label: "Alpha" },
    { id: "b", label: "Beta" },
    { id: "c", label: "Gamma" },
  ];
  let active = $state("a");
</script>

<Tabs {tabs} bind:active>
  {#snippet children(current)}
    <p>content-{current}</p>
  {/snippet}
</Tabs>
