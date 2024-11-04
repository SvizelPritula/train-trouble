<script lang="ts">
  export let occupied: Record<string, boolean>;
</script>

<svg viewBox="0 0 860 440" version="1.1">
  <g class="railway">
    <path d="M 530,110 H 290" class:occupied={occupied["to-nová-ves-left"]} />
    <path
      d="m 300,110 10,-10 v 20 z"
      class:occupied={occupied["to-nová-ves-left"]}
      class="signal"
    />
    <path d="m 530,80 -240,0" class:occupied={occupied["to-nová-ves-right"]} />
    <path
      d="m 300,80 10,-10 v 20 z"
      class:occupied={occupied["to-nová-ves-right"]}
      class="signal"
    />
    <path
      d="m 290,110 h -140"
      class:occupied={occupied["nová-ves-left-stop"]}
    />
    <path
      d="m 160,110 10,-10 v 20 z"
      class:occupied={occupied["nová-ves-left-stop"]}
      class="signal"
    />
    <path
      d="m 290,80 h -140"
      class:occupied={occupied["nová-ves-right-stop"]}
    />
    <path
      d="m 160,80 10,-10 v 20 z"
      class:occupied={occupied["nová-ves-right-stop"]}
      class="signal"
    />
    <path
      d="M 140,80 110,110"
      class:occupied={occupied["nová-ves-right-switch"]}
    />
    <path
      d="M 140,110 110,80"
      class:occupied={occupied["nová-ves-left-switch"]}
    />
    <path
      d="M 150,80 H 110"
      class:occupied={occupied["nová-ves-right-switch"]}
    />
    <path
      d="m 150,110 -40,0"
      class:occupied={occupied["nová-ves-left-switch"]}
    />
    <path
      d="m 110,110 -20,0 -30,30 V 230 l 30,30 H 190"
      class:occupied={occupied["to-horní-mechoklaty"]}
    />
    <path
      d="M 180,260 170,270 v -20 z"
      class:occupied={occupied["to-horní-mechoklaty"]}
      class="signal"
    />
    <path
      d="M 190,260 H 330"
      class:occupied={occupied["horní-mechoklaty-stop"]}
    />
    <path
      d="m 320,260 -10,10 v -20 z"
      class:occupied={occupied["horní-mechoklaty-stop"]}
      class="signal"
    />
    <path
      d="M 110,80 H 90 l -60,60 v 210 l 60,60 H 190"
      class:occupied={occupied["to-dolní-mechoklaty"]}
    />
    <path
      d="M 180,410 170,420 v -20 z"
      class:occupied={occupied["to-dolní-mechoklaty"]}
      class="signal"
    />
    <path
      d="M 190,410 H 330"
      class:occupied={occupied["dolní-mechoklaty-stop"]}
    />
    <path
      d="M 320,410 310,420 v -20 z"
      class:occupied={occupied["dolní-mechoklaty-stop"]}
      class="signal"
    />
    <path
      d="m 390,380 -30,30"
      class:occupied={occupied["dolní-mechoklaty-switch"]}
    />
    <path
      d="m 330,410 h 60"
      class:occupied={occupied["dolní-mechoklaty-switch"]}
    />
    <path
      d="M 330,260 H 480"
      class:occupied={occupied["to-předvořany-switch-left"]}
    />
    <path
      d="m 390,380 90,-90"
      class:occupied={occupied["to-předvořany-switch-right"]}
    />
    <path
      d="M 570,260 480,260"
      class:occupied={occupied["předvořany-switch"]}
    />
    <path d="m 510,260 -30,30" class:occupied={occupied["předvořany-switch"]} />
    <path
      d="M 560,260 550,270 v -20 z"
      class:occupied={occupied["předvořany-switch"]}
      class="signal"
    />
    <path d="M 570,260 H 710" class:occupied={occupied["předvořany-stop"]} />
    <path
      d="m 700,260 -10,10 v -20 z"
      class:occupied={occupied["předvořany-stop"]}
      class="signal"
    />
    <path
      d="m 710,260 h 30 l 60,-60 L 800,140 770,110 h -30"
      class:occupied={occupied["to-kolnov-left"]}
    />
    <path
      d="m 750,110 10,-10 v 20 z"
      class:occupied={occupied["to-kolnov-left"]}
      class="signal"
    />
    <path
      d="m 390,410 h 380 l 60,-60 0,-210 -60,-60 h -30"
      class:occupied={occupied["to-kolnov-right"]}
    />
    <path
      d="m 750,80 10,-10 v 20 z"
      class:occupied={occupied["to-kolnov-right"]}
      class="signal"
    />
    <path d="m 710,110 h 30" class:occupied={occupied["kolnov-entry-switch"]} />
    <path
      d="M 710,80 740,110"
      class:occupied={occupied["kolnov-entry-switch"]}
    />
    <path
      d="m 710,80 h 30"
      class:occupied={occupied["kolnov-entry-connection"]}
    />
    <path d="m 710,110 -150,0" class:occupied={occupied["kolnov-left-stop"]} />
    <path
      d="m 580,110 10,-10 v 20 z"
      class:occupied={occupied["kolnov-left-stop"]}
      class="signal"
    />
    <path d="M 710,80 H 560" class:occupied={occupied["kolnov-right-stop"]} />
    <path
      d="m 580,80 10,-10 v 20 z"
      class:occupied={occupied["kolnov-right-stop"]}
      class="signal"
    />
    <path d="M 560,80 H 530" class:occupied={occupied["kolnov-exit-switch"]} />
    <path
      d="M 560,80 530,110"
      class:occupied={occupied["kolnov-exit-switch"]}
    />
    <path
      d="M 560,110 H 530"
      class:occupied={occupied["kolnov-exit-connection"]}
    />
  </g>

  <g class="text">
    <g class="title">
      <text x="235" y="50">Nová Ves nad Spálou</text>
      <text x="625" y="230">Předvořany</text>
      <text x="245" y="230">Horní Mechoklaty</text>
      <text x="245" y="380">Dolní Mechoklaty</text>
      <text x="670" y="50">Kolnov</text>
    </g>

    <text x="190" y="100">1</text>
    <text x="190" y="70">2</text>
    <text x="290" y="100">N1</text>
    <text x="290" y="70">N2</text>
    <text x="150" y="100">N3</text>
    <text x="150" y="70">N4</text>
    <text x="130" y="122">Na</text>
    <text x="130" y="76">Nb</text>
    <text x="610" y="100">1</text>
    <text x="610" y="70">2</text>
    <text x="740" y="100">K1</text>
    <text x="740" y="70">K2</text>
    <text x="570" y="100">K3</text>
    <text x="570" y="70">K4</text>
    <text x="730" y="122">Ka</text>
    <text x="550" y="76">Kb</text>
    <text x="180" y="250">H1</text>
    <text x="320" y="250">H2</text>
    <text x="180" y="400">D1</text>
    <text x="320" y="400">D2</text>
    <text x="360" y="423">Da</text>
    <text x="560" y="250">P1</text>
    <text x="700" y="250">P2</text>
  </g>
</svg>

<style>
  svg {
    width: 100%;
    height: 100%;
  }

  .railway {
    stroke: #fff;
    stroke-width: 3;
    stroke-linecap: butt;
    stroke-linejoin: round;
    fill: none;
  }

  .signal {
    fill: #000;
  }

  .occupied {
    stroke: #0a0;
  }

  .text {
    fill: #fff;
    font-size: 12px;
    line-height: 1.25;
    font-family: "Noto Mono", monospace;
  }

  .title {
    font-size: 16px;
    text-align: center;
    text-anchor: middle;
  }
</style>
