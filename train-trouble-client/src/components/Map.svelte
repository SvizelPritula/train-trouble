<script lang="ts">
  export let occupied: Record<string, boolean>;
</script>

<svg viewBox="0 0 860 440" version="1.1">
  <g class="railway">
    <path
      d="M 530,110 H 290.00002"
      class:occupied={occupied["to-nová-ves-left"]}
    />
    <path
      d="m 300.00002,110.00001 9.99998,-10 v 20 z"
      class:occupied={occupied["to-nová-ves-left"]}
      class="signal"
    />
    <path
      d="m 530,80.000001 -239.99998,1e-5"
      class:occupied={occupied["to-nová-ves-right"]}
    />
    <path
      d="m 300.00002,80.000011 9.99998,-10 v 20 z"
      class:occupied={occupied["to-nová-ves-right"]}
      class="signal"
    />
    <path
      d="m 290.00002,110.00001 h -140"
      class:occupied={occupied["nová-ves-left-stop"]}
    />
    <path
      d="m 160.00002,110.00001 9.99998,-10 v 20 z"
      class:occupied={occupied["nová-ves-left-stop"]}
      class="signal"
    />
    <path
      d="m 290.00002,80.000011 h -140"
      class:occupied={occupied["nová-ves-right-stop"]}
    />
    <path
      d="m 160.00002,80.000011 9.99998,-10 v 20 z"
      class:occupied={occupied["nová-ves-right-stop"]}
      class="signal"
    />
    <path
      d="M 140,80.000001 110,109.99999"
      class:occupied={occupied["nová-ves-right-switch"]}
    />
    <path
      d="M 140,109.99999 110,80.000001"
      class:occupied={occupied["nová-ves-left-switch"]}
    />
    <path
      d="M 150,80.000001 H 110"
      class:occupied={occupied["nová-ves-right-switch"]}
    />
    <path
      d="m 150,110 -40,-1e-5"
      class:occupied={occupied["nová-ves-left-switch"]}
    />
    <path
      d="m 110,110 -19.999999,-1e-5 -30,30.00002 V 230 l 30,30 H 190"
      class:occupied={occupied["to-horní-mechoklaty"]}
    />
    <path
      d="M 179.99998,260 170,270 v -20 z"
      class:occupied={occupied["to-horní-mechoklaty"]}
      class="signal"
    />
    <path
      d="M 190,260 H 330"
      class:occupied={occupied["horní-mechoklaty-stop"]}
    />
    <path
      d="m 320,260 -9.99998,10 v -20 z"
      class:occupied={occupied["horní-mechoklaty-stop"]}
      class="signal"
    />
    <path
      d="M 110,80.000001 H 90.000001 l -60,59.999999 v 210 l 60,60 H 190"
      class:occupied={occupied["to-dolní-mechoklaty"]}
    />
    <path
      d="M 179.99998,410 170,420 v -20 z"
      class:occupied={occupied["to-dolní-mechoklaty"]}
      class="signal"
    />
    <path
      d="M 190,410 H 330"
      class:occupied={occupied["dolní-mechoklaty-stop"]}
    />
    <path
      d="M 319.99998,410 310,420 v -20 z"
      class:occupied={occupied["dolní-mechoklaty-stop"]}
      class="signal"
    />
    <path
      d="m 390,379.99999 -30.00002,30"
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
      d="m 390,379.99999 90,-90"
      class:occupied={occupied["to-předvořany-switch-right"]}
    />
    <path
      d="M 570.00002,259.99999 480,260"
      class:occupied={occupied["předvořany-switch"]}
    />
    <path
      d="m 510,260 -30,29.99999"
      class:occupied={occupied["předvořany-switch"]}
    />
    <path
      d="M 559.99998,260 550,270 v -20 z"
      class:occupied={occupied["předvořany-switch"]}
      class="signal"
    />
    <path d="M 570,260 H 710" class:occupied={occupied["předvořany-stop"]} />
    <path
      d="m 700,260 -9.99998,10 v -20 z"
      class:occupied={occupied["předvořany-stop"]}
      class="signal"
    />
    <path
      d="m 710.00001,260 h 30 l 60,-60 L 800,140.00001 769.99998,109.99999 h -29.99999"
      class:occupied={occupied["to-kolnov-left"]}
    />
    <path
      d="m 750.00003,110.00001 9.99998,-10 v 20 z"
      class:occupied={occupied["to-kolnov-left"]}
      class="signal"
    />
    <path
      d="m 390,410 h 380.00001 l 60,-60 -2e-5,-209.99999 -60.00001,-60.000009 h -29.99997"
      class:occupied={occupied["to-kolnov-right"]}
    />
    <path
      d="m 750.00003,80.000011 9.99998,-10 v 20 z"
      class:occupied={occupied["to-kolnov-right"]}
      class="signal"
    />
    <path
      d="m 710.00002,110 h 29.99999"
      class:occupied={occupied["kolnov-entry-switch"]}
    />
    <path
      d="M 710.00002,80.000011 740.00001,110"
      class:occupied={occupied["kolnov-entry-switch"]}
    />
    <path
      d="m 710.00002,80.000011 h 29.99999"
      class:occupied={occupied["kolnov-entry-connection"]}
    />
    <path
      d="m 710.00002,110 -150.00003,-1e-5"
      class:occupied={occupied["kolnov-left-stop"]}
    />
    <path
      d="m 580.00002,110.00001 9.99998,-10 v 20 z"
      class:occupied={occupied["kolnov-left-stop"]}
      class="signal"
    />
    <path
      d="M 710.00002,80.000011 H 560"
      class:occupied={occupied["kolnov-right-stop"]}
    />
    <path
      d="m 580.00002,80.000011 9.99998,-10 v 20 z"
      class:occupied={occupied["kolnov-right-stop"]}
      class="signal"
    />
    <path
      d="M 559.99999,80.000001 H 530"
      class:occupied={occupied["kolnov-exit-switch"]}
    />
    <path
      d="M 559.99999,80.000001 530,109.99999"
      class:occupied={occupied["kolnov-exit-switch"]}
    />
    <path
      d="M 559.99999,109.99999 H 530"
      class:occupied={occupied["kolnov-exit-connection"]}
    />
  </g>

  <g class="text" font-family="'Noto Mono'">
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
  }

  .title {
    font-size: 16px;
    text-align: center;
    text-anchor: middle;
  }
</style>
