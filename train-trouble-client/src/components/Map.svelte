<script lang="ts">
  export let occupied: Record<string, boolean>;
</script>

<div>
  {#each Object.entries(occupied) as [track, status]}
    <p class:occupied={status}>{track}</p>
  {/each}
</div>

<style>
  .occupied {
    color: green;
  }
</style>
