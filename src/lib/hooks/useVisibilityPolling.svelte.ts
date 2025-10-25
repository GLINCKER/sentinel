/**
 * Visibility-aware polling hook
 * Only polls when component is mounted AND page is visible
 * Pauses polling when user switches tabs/windows or navigates away
 */

import { onMount, onDestroy } from 'svelte';

interface PollingOptions {
	/** Polling interval in milliseconds */
	interval: number;
	/** Callback function to execute on each poll */
	callback: () => void | Promise<void>;
	/** Execute immediately on mount (default: true) */
	immediate?: boolean;
	/** Poll even when page is hidden (default: false) */
	pollWhenHidden?: boolean;
}

export function useVisibilityPolling(options: PollingOptions) {
	const { interval, callback, immediate = true, pollWhenHidden = false } = options;

	let intervalId: ReturnType<typeof setInterval> | null = null;
	let isPolling = $state(false);

	function startPolling() {
		if (isPolling) return;

		// Execute immediately if requested
		if (immediate) {
			callback();
		}

		intervalId = setInterval(() => {
			// Only poll if page is visible (unless pollWhenHidden is true)
			if (pollWhenHidden || !document.hidden) {
				callback();
			}
		}, interval);

		isPolling = true;
	}

	function stopPolling() {
		if (intervalId) {
			clearInterval(intervalId);
			intervalId = null;
		}
		isPolling = false;
	}

	function handleVisibilityChange() {
		if (document.hidden) {
			// Page hidden - stop polling to save resources
			stopPolling();
		} else {
			// Page visible - resume polling
			startPolling();
		}
	}

	onMount(() => {
		// Start polling immediately
		startPolling();

		// Listen for visibility changes (tab switching, window minimize, etc.)
		if (!pollWhenHidden) {
			document.addEventListener('visibilitychange', handleVisibilityChange);
		}
	});

	onDestroy(() => {
		stopPolling();
		if (!pollWhenHidden) {
			document.removeEventListener('visibilitychange', handleVisibilityChange);
		}
	});

	return {
		get isPolling() {
			return isPolling;
		},
		startPolling,
		stopPolling,
		restart: () => {
			stopPolling();
			startPolling();
		}
	};
}
