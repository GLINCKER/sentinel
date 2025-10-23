import { invoke } from '@tauri-apps/api/core';
import type { NetworkSnapshot, NetworkInterfaceStats } from '$lib/types/network';

export async function getNetworkStats(): Promise<NetworkSnapshot> {
	return await invoke('get_network_stats');
}

export async function getNetworkHistory(durationSeconds: number): Promise<NetworkSnapshot[]> {
	return await invoke('get_network_history', { durationSeconds });
}

export async function clearNetworkHistory(): Promise<void> {
	await invoke('clear_network_history');
}

export async function getNetworkInterfaces(): Promise<NetworkInterfaceStats[]> {
	return await invoke('get_network_interfaces');
}
