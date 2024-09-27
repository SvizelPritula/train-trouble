<script lang="ts">
  import type { Submit } from "../lib/actions";
  import type { ZoneView } from "../lib/views";
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
        <th>Cena za jednotku</th>
      </tr>
    </thead>

    <tbody>
      {#each view.rates as { id, rate }}
        <tr>
          <td>{id}</td>
          <td>{rate}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  <p class="status">
    Rozpočet:
    <b>{view.balance.toLocaleString("cs-CZ")}</b>
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

{#each view.signals as { id, name, clear }}
  <div class="control">
    <h2>Návěstidlo {name}</h2>

    <p class="status">
      Návěst:
      <b>
        {#if clear == true}
          volno
        {:else if clear == false}
          stůj
        {:else}
          přepínám…
        {/if}
      </b>
    </p>

    <button
      disabled={clear == null}
      on:click={() =>
        submit({
          type: "signal",
          id,
          clear: !clear,
        })}
    >
      Přepnout
    </button>
  </div>
{/each}

{#each view.switches as { id, name, direction }}
  <div class="control">
    <h2>Výhybka {name}</h2>

    <p class="status">
      Směr:
      <b>
        {#if direction == "left"}
          doleva
        {:else if direction == "right"}
          doprava
        {:else}
          přehazuji…
        {/if}
      </b>
    </p>

    <button
      disabled={direction == null}
      on:click={() =>
        submit({
          type: "switch",
          id,
          direction: direction == "left" ? "right" : "left",
        })}
    >
      Přehodit
    </button>
  </div>
{/each}

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

  button {
    width: 100%;
    padding: 0.5rem 1rem;

    background-color: #333;
    border: 0.15rem solid #666;
    border-radius: 0.25rem;

    color: inherit;
    font-size: 1.25rem;
  }

  button:hover:enabled {
    background-color: #282828;
  }

  button:disabled {
    color: #bbb;
    background-color: #444;
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
