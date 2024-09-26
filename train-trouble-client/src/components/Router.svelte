<script lang="ts">
  import { hashToChannel, type Channel } from "../lib/channels";
  import GameScreen from "./GameScreen.svelte";
  import MapScreen from "./MapScreen.svelte";

  let hash = window.location.hash;
  let channel: Channel | null;
  $: channel = hashToChannel[hash] ?? null;
</script>

<svelte:window on:hashchange={() => (hash = window.location.hash)} />

{#key channel}
  {#if channel == null}
    <main>Tato lokace neexistuje.</main>
  {:else if channel.type == "map"}
    <MapScreen />
  {:else}
    <GameScreen {channel} />
  {/if}
{/key}

<style>
  main {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
