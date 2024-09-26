<script lang="ts">
  import type { Submit } from "../lib/actions";
  import type { ZoneView } from "../lib/views";

  export let view: ZoneView;
  export let submit: Submit;
</script>

{#each view.signals as { id, clear }}
  <h2>Návěstidlo {id}</h2>

  <p>
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
{/each}

{#each view.switches as { id, direction }}
  <h2>Výhybka {id}</h2>

  <p>
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
{/each}
