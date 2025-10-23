<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import uPlot from 'uplot';
  import 'uplot/dist/uPlot.min.css';
  import type { NetworkSnapshot } from '$lib/types/network';

  interface Props {
    data?: NetworkSnapshot[];
    height?: number;
    theme?: 'dark' | 'light';
  }

  let { data = [], height = 300, theme = 'dark' }: Props = $props();

  let chartContainer: HTMLDivElement;
  let chart: uPlot | null = null;
  let resizeObserver: ResizeObserver;

  // Convert NetworkSnapshot[] to uPlot data format
  // Calculate rate of change (bytes per second) instead of cumulative totals
  function prepareChartData(snapshots: NetworkSnapshot[]): uPlot.AlignedData {
    if (snapshots.length === 0) {
      return [[], [], []];
    }

    const timestamps: number[] = [];
    const sendRates: number[] = [];
    const receiveRates: number[] = [];

    for (let i = 0; i < snapshots.length; i++) {
      const current = snapshots[i];
      const timestamp = new Date(current.timestamp).getTime() / 1000;
      timestamps.push(timestamp);

      if (i === 0) {
        // First data point: no rate calculation possible, use 0
        sendRates.push(0);
        receiveRates.push(0);
      } else {
        const previous = snapshots[i - 1];
        const timeDelta =
          timestamp - new Date(previous.timestamp).getTime() / 1000;

        if (timeDelta > 0) {
          // Calculate bytes per second, then convert to KB/s
          const sentDelta = current.totalBytesSent - previous.totalBytesSent;
          const receivedDelta =
            current.totalBytesReceived - previous.totalBytesReceived;

          const sendRate = sentDelta / timeDelta / 1024; // KB/s
          const receiveRate = receivedDelta / timeDelta / 1024; // KB/s

          sendRates.push(Math.max(0, sendRate));
          receiveRates.push(Math.max(0, receiveRate));
        } else {
          sendRates.push(0);
          receiveRates.push(0);
        }
      }
    }

    return [timestamps, sendRates, receiveRates];
  }

  // uPlot configuration
  function getChartOptions(width: number): uPlot.Options {
    return {
      width,
      height,
      series: [
        {},
        {
          label: 'Upload',
          stroke: '#3b82f6',
          fill: 'rgba(59, 130, 246, 0.15)',
          width: 2.5,
          points: {
            show: false
          }
        },
        {
          label: 'Download',
          stroke: '#10b981',
          fill: 'rgba(16, 185, 129, 0.15)',
          width: 2.5,
          points: {
            show: false
          }
        }
      ],
      axes: [
        {
          stroke: '#4b5563',
          grid: {
            stroke: '#1f2937',
            width: 1
          },
          ticks: {
            stroke: '#374151',
            width: 1
          }
        },
        {
          stroke: '#4b5563',
          grid: {
            stroke: '#1f2937',
            width: 1
          },
          ticks: {
            stroke: '#374151',
            width: 1
          },
          values: (u, vals) =>
            vals.map((v) => {
              if (v === 0) return '0';
              if (v >= 1000) {
                return (v / 1024).toFixed(1) + ' MB/s';
              }
              if (v < 1) {
                return (v * 1024).toFixed(0) + ' B/s';
              }
              return v.toFixed(0) + ' KB/s';
            })
        }
      ],
      scales: {
        x: {
          time: true
        },
        y: {
          range: (u, min, max) => {
            // Ensure minimum scale of 10 KB/s for better visibility
            const minScale = 10;
            const maxScale = Math.max(max, minScale);
            return [0, maxScale];
          }
        }
      },
      cursor: {
        drag: {
          x: false,
          y: false
        },
        points: {
          size: 8,
          width: 2
        },
        sync: {
          key: 'network-graph'
        }
      },
      legend: {
        show: false // Hide legend since we have custom one
      },
      padding: [16, 16, 0, 0]
    };
  }

  function initChart() {
    if (!chartContainer) return;

    const containerWidth = chartContainer.offsetWidth;
    const chartData = prepareChartData(data);
    const options = getChartOptions(containerWidth);

    chart = new uPlot(options, chartData, chartContainer);
  }

  function updateChart() {
    if (!chart) return;

    const chartData = prepareChartData(data);
    chart.setData(chartData);
  }

  onMount(() => {
    initChart();

    // Handle resize
    resizeObserver = new ResizeObserver(() => {
      if (chart && chartContainer) {
        chart.setSize({
          width: chartContainer.offsetWidth,
          height: height
        });
      }
    });
    resizeObserver.observe(chartContainer);
  });

  onDestroy(() => {
    chart?.destroy();
    resizeObserver?.disconnect();
  });

  // Update chart when data changes
  $effect(() => {
    if (data && chart) {
      updateChart();
    }
  });

  // Update theme when it changes
  $effect(() => {
    if (theme && chart && chartContainer) {
      chart.destroy();
      initChart();
    }
  });
</script>

<div class="network-graph">
  <div bind:this={chartContainer} class="chart-container"></div>
</div>

<style>
  .network-graph {
    width: 100%;
    background: transparent;
  }

  .chart-container {
    width: 100%;
    min-height: 300px;
  }

  /* Override uPlot default styles for dark mode */
  .chart-container :global(.u-over) {
    background: transparent !important;
  }

  .chart-container :global(.u-legend) {
    display: none;
  }

  .chart-container :global(.u-cursor-x),
  .chart-container :global(.u-cursor-y) {
    border-color: #4b5563 !important;
    border-width: 1px !important;
  }

  .chart-container :global(.u-cursor-pt) {
    background: var(--accent-primary) !important;
    border: 2px solid white !important;
    border-radius: 50%;
  }

  .chart-container :global(.u-select) {
    background: rgba(59, 130, 246, 0.1) !important;
  }

  .chart-container :global(.u-axis) {
    color: #9ca3af;
    font-size: 12px;
    font-weight: 500;
  }

  .chart-container :global(.u-value) {
    color: #d1d5db;
    font-weight: 600;
  }
</style>
