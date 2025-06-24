<script lang="ts">
  export let progress: number | null;
  export let color: string;
</script>

{#if progress != null}
  <div class="container" style:--color={color}>
    <div class="headline">Detekována srážka</div>
    <div class="progress">Stav úklidu: <b>{progress} %</b></div>
  </div>
{/if}

<style>
  .container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;

    font-family: "Noto Mono", monospace;
    color: color-mix(in hsl, var(--color) 80%, white);
    text-align: center;

    background-color: #00000044;
  }

  .headline {
    font-size: 10vmin;
  }

  .progress {
    font-size: 5vmin;
  }
</style>
