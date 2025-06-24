<script lang="ts">
  import "@fontsource/noto-mono";

  import { onMount } from "svelte";
  import { connect } from "train-trouble-engine-client";
  import { teams, type Channel } from "../lib/channels";
  import type { MapView } from "../lib/views";
  import Map from "./Map.svelte";
  import CrashStatus from "./CrashStatus.svelte";

  let view: MapView | null = null;
  let connected: boolean = false;

  onMount(() => {
    let connection = connect<Channel, MapView, void>(
      { type: "map" },
      (v) => (view = v),
      (c) => (connected = c)
    );

    return connection.stop;
  });
</script>

<main>
  {#if view != null}
    {#each teams as { id, name } (id)}
      <div class="map">
        <Map occupied={view.teams[id].occupied} />
        <CrashStatus progress={view.teams[id].crash_cleanup_progress} />
      </div>
    {/each}
  {/if}
</main>

<style>
  main {
    width: 100%;
    height: 100%;

    display: grid;
    grid-template-columns: 100%;
    grid-auto-rows: minmax(0, 1fr);

    background-color: #000;
    filter: url("../styles/filters.svg#glow");
  }

  .map {
    position: relative;
    display: flex;
  }
</style>
