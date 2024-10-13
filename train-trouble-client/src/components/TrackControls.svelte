<script lang="ts">
  import type { Submit } from "../lib/actions";
  import type { ZoneView } from "../lib/views";

  export let view: ZoneView;
  export let submit: Submit;
</script>

{#if view.signals.length > 0}
  <div class="control">
    <h2>Návěstidla</h2>

    {#each view.signals as { id, name, clear }}
      <div class="split">
        <div class="status">
          {name}:
          <b>
            {#if clear == true}
              volno
            {:else if clear == false}
              stůj
            {:else}
              přepínám…
            {/if}
          </b>
        </div>

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
  </div>
{/if}

{#if view.switches.length > 0}
  <div class="control">
    <h2>Výhybky</h2>

    {#each view.switches as { id, name, direction }}
      <div class="split">
        <div class="status">
          {name}:
          <b>
            {#if direction == "left"}
              doleva
            {:else if direction == "right"}
              doprava
            {:else}
              přehazuji…
            {/if}
          </b>
        </div>

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
  </div>
{/if}

<style>
  .control {
    margin: 2rem 0;
  }

  h2 {
    margin: 0;
    margin-bottom: 1rem;
  }

  .split {
    margin: 0.5rem 0;

    display: flex;
    flex-direction: row;

    align-items: center;
  }

  .status {
    font-size: 1.25rem;
    flex-grow: 1;
  }

  button {
    min-width: 6rem;
    padding: 0.25rem 0.5rem;

    background-color: #333;
    border: 0.15rem solid #666;
    border-radius: 0.25rem;

    color: inherit;
    font-size: 1rem;
  }

  button:hover:enabled {
    background-color: #282828;
  }

  button:disabled {
    color: #bbb;
    background-color: #444;
  }
</style>
