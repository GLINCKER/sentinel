<!--
  @file Mini Sparkline Chart
  @glinr/sentinel

  Compact sparkline chart for metric cards.
-->

<script lang="ts">
  interface DataPoint {
    timestamp: number;
    value: number;
  }

  interface Props {
    data: DataPoint[];
    color?: string;
    height?: number;
  }

  let { data, color = '#3b82f6', height = 40 }: Props = $props();

  const width = 200;

  let path = $derived(() => {
    if (data.length === 0) return '';

    const maxValue = Math.max(...data.map((d) => d.value), 1);
    const minValue = Math.min(...data.map((d) => d.value), 0);
    const range = maxValue - minValue || 1;

    const points = data.map((d, i) => {
      const x = (i / (data.length - 1 || 1)) * width;
      const y = height - ((d.value - minValue) / range) * height;
      return `${x},${y}`;
    });

    return `M ${points.join(' L ')}`;
  });

  let areaPath = $derived(() => {
    if (data.length === 0) return '';

    const maxValue = Math.max(...data.map((d) => d.value), 1);
    const minValue = Math.min(...data.map((d) => d.value), 0);
    const range = maxValue - minValue || 1;

    const points = data.map((d, i) => {
      const x = (i / (data.length - 1 || 1)) * width;
      const y = height - ((d.value - minValue) / range) * height;
      return [x, y];
    });

    const pathData = points
      .map((p, i) => {
        return i === 0 ? `M ${p[0]},${p[1]}` : `L ${p[0]},${p[1]}`;
      })
      .join(' ');

    return `${pathData} L ${width},${height} L 0,${height} Z`;
  });
</script>

<svg {width} {height} class="sparkline">
  <!-- Area fill with gradient -->
  {#if areaPath()}
    <defs>
      <linearGradient
        id="sparkline-gradient-{color}"
        x1="0%"
        y1="0%"
        x2="0%"
        y2="100%"
      >
        <stop offset="0%" style="stop-color:{color};stop-opacity:0.3" />
        <stop offset="100%" style="stop-color:{color};stop-opacity:0.05" />
      </linearGradient>
    </defs>
    <path d={areaPath()} fill="url(#sparkline-gradient-{color})" />
  {/if}

  <!-- Line -->
  {#if path()}
    <path
      d={path()}
      fill="none"
      stroke={color}
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  {/if}
</svg>

<style>
  .sparkline {
    width: 100%;
    height: auto;
    display: block;
  }
</style>
