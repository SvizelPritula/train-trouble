<script lang="ts">
  import type { Submit } from "../lib/actions";
  import type { ZoneView } from "../lib/views";
  import TrackControls from "./TrackControls.svelte";
  import TradeForm from "./TradeForm.svelte";

  export let view: ZoneView;
  export let submit: Submit;
</script>

<svelte:head>
  <title>{view.name} | Train Trouble</title>
</svelte:head>

<div class="control">
  <h1>{view.name}</h1>

  <h2>Kurzy</h2>

  <table>
    <thead>
      <tr>
        <th>Komodita</th>
        <th>Cena za kilogram</th>
      </tr>
    </thead>

    <tbody>
      {#each view.rates as { name, rate }}
        <tr>
          <td>{name}</td>
          <td>{rate} ₸</td>
        </tr>
      {/each}
    </tbody>
  </table>

  <p class="status">
    Rozpočet:
    <b>{view.balance.toLocaleString("cs-CZ")} ₸</b>
  </p>
</div>

{#each view.platforms.entries() as [idx, { trains }]}
  <div class="control">
    <h2>Kolej {idx + 1}</h2>

    {#each trains as train}
      <p class="status">
        Vlak {train.name}
        {#if !train.stopped}přijíždí…{/if}
      </p>

      {#if train.load != null}
        <TradeForm rates={view.rates} {train} {submit} />
      {/if}
    {:else}
      <p class="status"><i>Prázdná</i></p>
    {/each}
  </div>
{/each}

<TrackControls {view} {submit} />

<style>
  .control {
    margin: 2rem 0;
  }

  h2 {
    margin: 0;
    margin-bottom: 1rem;
  }

  .status {
    margin: 1rem 0;
    font-size: 1.25rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  td,
  th {
    text-align: center;
    padding: 0.2rem;
  }

  thead {
    border-bottom: 0.2rem #fff solid;
  }
</style>
