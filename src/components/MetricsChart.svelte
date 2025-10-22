<!--
  @file Metrics Chart Component
  @glinr/sentinel

  Lightweight SVG chart for displaying metrics history.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  interface DataPoint {
    timestamp: number;
    value: number;
  }

  interface Props {
    data: DataPoint[];
    label: string;
    color?: string;
    height?: number;
    max?: number;
  }

  let { data, label, color = '#3b82f6', height = 120, max }: Props = $props();

  const width = 600;
  const padding = { top: 10, right: 10, bottom: 20, left: 40 };
  const chartWidth = width - padding.left - padding.right;
  const chartHeight = height - padding.top - padding.bottom;

  let points = $derived(() => {
    if (data.length === 0) return '';

    const maxValue = max ?? Math.max(...data.map((d) => d.value), 1);
    const minValue = 0;
    const range = maxValue - minValue;

    return data
      .map((d, i) => {
        const x = padding.left + (i / (data.length - 1 || 1)) * chartWidth;
        const y =
          padding.top +
          chartHeight -
          ((d.value - minValue) / range) * chartHeight;
        return `${x},${y}`;
      })
      .join(' ');
  });

  let areaPoints = $derived(() => {
    if (!points()) return '';
    const bottomLeft = `${padding.left},${padding.top + chartHeight}`;
    const bottomRight = `${padding.left + chartWidth},${padding.top + chartHeight}`;
    return `${bottomLeft} ${points()} ${bottomRight}`;
  });

  let currentValue = $derived(
    data.length > 0 ? data[data.length - 1].value.toFixed(1) : '0.0'
  );

  function getYAxisLabels(maxVal: number) {
    const steps = 4;
    return Array.from({ length: steps + 1 }, (_, i) => {
      const value = (maxVal / steps) * (steps - i);
      return {
        y: padding.top + (chartHeight / steps) * i,
        label: value.toFixed(0)
      };
    });
  }

  let yAxisLabels = $derived(
    getYAxisLabels(max ?? Math.max(...data.map((d) => d.value), 100))
  );
</script>

<div class="metrics-chart">
  <div class="chart-header">
    <h3 class="chart-title">{label}</h3>
    <span class="chart-value">{currentValue}%</span>
  </div>

  <svg {width} {height} class="chart-svg">
    <!-- Grid lines -->
    {#each yAxisLabels as { y } (y)}
      <line
        x1={padding.left}
        y1={y}
        x2={padding.left + chartWidth}
        y2={y}
        class="grid-line"
      />
    {/each}

    <!-- Area fill -->
    {#if areaPoints()}
      <polygon
        points={areaPoints()}
        class="area-fill"
        style="fill: {color}; fill-opacity: 0.1;"
      />
    {/if}

    <!-- Line -->
    {#if points()}
      <polyline points={points()} class="chart-line" style="stroke: {color};" />
    {/if}

    <!-- Y-axis labels -->
    {#each yAxisLabels as { y, label } (y)}
      <text x={padding.left - 8} y={y + 4} class="axis-label" text-anchor="end">
        {label}
      </text>
    {/each}

    <!-- Current value indicator -->
    {#if data.length > 0}
      {@const lastPoint = data[data.length - 1]}
      {@const maxValue = max ?? Math.max(...data.map((d) => d.value), 1)}
      {@const x = padding.left + chartWidth}
      {@const y =
        padding.top + chartHeight - (lastPoint.value / maxValue) * chartHeight}
      <circle cx={x} cy={y} r="4" fill={color} class="current-indicator" />
    {/if}
  </svg>
</div>

<style>
  .metrics-chart {
    background: var(--bg-secondary);
    border-radius: var(--radius-lg);
    padding: var(--space-md);
    border: 1px solid var(--border-color);
  }

  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-sm);
  }

  .chart-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-secondary);
    margin: 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .chart-value {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    color: var(--text-primary);
  }

  .chart-svg {
    width: 100%;
    height: auto;
    display: block;
  }

  .grid-line {
    stroke: var(--border-light);
    stroke-width: 1;
    stroke-dasharray: 2, 2;
  }

  .chart-line {
    fill: none;
    stroke-width: 2;
    stroke-linejoin: round;
    stroke-linecap: round;
  }

  .axis-label {
    font-size: 10px;
    fill: var(--text-tertiary);
    font-family: var(--font-mono);
  }

  .current-indicator {
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
  }
</style>
