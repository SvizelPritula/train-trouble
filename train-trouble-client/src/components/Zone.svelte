<script lang="ts">
  import type { Submit } from "../lib/actions";
  import type { ZoneView } from "../lib/views";

  export let view: ZoneView;
  export let submit: Submit;
</script>

{#each view.platforms.entries() as [idx, { trains }]}
  <div class="control">
    <h2>Kolej {idx + 1}</h2>

    {#each trains as train}
      <p class="status">
        Vlak {train.id}
        {#if !train.stopped}přijíždí…{/if}
      </p>
    {:else}
      <p class="status"><i>Prázdná</i></p>
    {/each}
  </div>
{/each}

{#each view.signals as { id, clear }}
  <div class="control">
    <h2>Návěstidlo {id}</h2>

    <p class="status">
      Návěst:
      <b>
        {#if clear == true}
          volno
        {:else if clear == false}
          stůj
        {:else}
          přehazuji…
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

{#each view.switches as { id, direction }}
  <div class="control">
    <h2>Výhybka {id}</h2>

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
  }

  .status {
    margin: 0.5rem 0;
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
</style>
