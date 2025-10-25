<script lang="ts">
  import { X, ChevronDown } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { toast } from 'svelte-sonner';
  import {
    PROCESS_TEMPLATES,
    type ProcessTemplate
  } from '$lib/data/processTemplates';

  interface Props {
    show: boolean;
    onClose: () => void;
    onProcessStarted?: () => void;
  }

  let { show, onClose, onProcessStarted }: Props = $props();

  let processName = $state('');
  let command = $state('');
  let args = $state('');
  let workingDir = $state('');
  let autoRestart = $state(false);
  let saveToConfig = $state(true);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let showTemplates = $state(false);
  let selectedTemplate = $state<ProcessTemplate | null>(null);

  function applyTemplate(template: ProcessTemplate) {
    selectedTemplate = template;
    processName = template.name;
    command = template.command;
    args = template.args.join(' ');
    autoRestart = template.autoRestart;
    showTemplates = false;
  }

  async function handleStart() {
    if (!processName.trim() || !command.trim()) {
      toast.error('Process name and command are required');
      return;
    }

    try {
      loading = true;
      error = null;

      const argsArray = args
        .trim()
        .split(/\s+/)
        .filter((a) => a.length > 0);

      const processConfig = {
        name: processName.trim(),
        command: command.trim(),
        args: argsArray,
        cwd: workingDir.trim() || null,
        env: selectedTemplate?.envVars || {},
        auto_restart: autoRestart,
        restart_limit: autoRestart ? 5 : 0,
        restart_delay: 1000,
        depends_on: [],
        health_check: null
      };

      // Start the process using PTY
      const pid = await invoke('spawn_pty_process', {
        processId: processName.trim(),
        command: command.trim(),
        args: argsArray,
        cwd: workingDir.trim() || null,
        env: selectedTemplate?.envVars || null
      });

      // Save to config if requested
      if (saveToConfig) {
        try {
          await invoke('save_process_to_config', {
            processConfig: processConfig,
            path: null
          });
        } catch (e) {
          console.warn('Failed to save to config:', e);
          // Don't fail the whole operation if save fails
        }
      }

      // Show success toast
      const saveMsg = saveToConfig ? ' and saved to config' : '';
      toast.success(
        `Process "${processName.trim()}" started successfully${saveMsg}`,
        {
          description: `PID: ${pid} | Running: ${command.trim()}${argsArray.length ? ' ' + argsArray.join(' ') : ''}`
        }
      );

      // Reset form
      processName = '';
      command = '';
      args = '';
      workingDir = '';
      autoRestart = false;
      saveToConfig = true;
      selectedTemplate = null;

      if (onProcessStarted) {
        onProcessStarted();
      }

      onClose();
    } catch (err) {
      const errorMessage = String(err);
      toast.error('Failed to start process', {
        description: errorMessage
      });
      error = errorMessage;
    } finally {
      loading = false;
    }
  }
</script>

{#if show}
  <div class="modal-overlay" onclick={onClose} role="presentation">
    <div
      class="modal-content"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-label="Start Process"
    >
      <!-- Header -->
      <div class="header">
        <h2 class="title">Start Managed Process</h2>
        <button class="close-button" onclick={onClose} aria-label="Close">
          <X size={20} />
        </button>
      </div>

      <!-- Templates -->
      <div class="templates-section">
        <button
          class="templates-toggle"
          onclick={() => (showTemplates = !showTemplates)}
        >
          <span>Framework Templates</span>
          <ChevronDown size={16} class={showTemplates ? 'rotate-180' : ''} />
        </button>

        {#if showTemplates}
          <div class="templates-grid">
            {#each PROCESS_TEMPLATES.slice(0, 12) as template (template.id)}
              <button
                class="template-card"
                class:selected={selectedTemplate?.id === template.id}
                onclick={() => applyTemplate(template)}
              >
                <div class="template-name">{template.name}</div>
                <div class="template-desc">{template.description}</div>
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Form -->
      <div class="form">
        {#if error}
          <div class="error-banner" role="alert">
            {error}
          </div>
        {/if}

        <div class="form-group">
          <label for="processName">Process Name *</label>
          <input
            id="processName"
            type="text"
            bind:value={processName}
            placeholder="e.g., my-app"
            required
          />
        </div>

        <div class="form-group">
          <label for="command">Command *</label>
          <input
            id="command"
            type="text"
            bind:value={command}
            placeholder="e.g., node, python3, /bin/bash"
            required
          />
        </div>

        <div class="form-group">
          <label for="args">Arguments</label>
          <input
            id="args"
            type="text"
            bind:value={args}
            placeholder="e.g., server.js --port 3000"
          />
        </div>

        <div class="form-group">
          <label for="workingDir">Working Directory</label>
          <input
            id="workingDir"
            type="text"
            bind:value={workingDir}
            placeholder="Leave empty for current directory"
          />
        </div>

        <div class="form-group">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={autoRestart} />
            <span>Auto-restart on crash (max 5 attempts)</span>
          </label>
        </div>

        <div class="form-group">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={saveToConfig} />
            <span>Save to config file (persist across app restarts)</span>
          </label>
        </div>
      </div>

      <!-- Footer -->
      <div class="footer">
        <button class="btn btn-cancel" onclick={onClose} disabled={loading}>
          Cancel
        </button>
        <button
          class="btn btn-primary"
          onclick={handleStart}
          disabled={loading}
        >
          {loading ? 'Starting...' : 'Start Process'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 2rem;
  }

  .modal-content {
    background: var(--card);
    backdrop-filter: blur(20px);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-xl);
    width: 90%;
    max-width: 600px;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-xl);
    border-bottom: 1px solid var(--border);
  }

  .title {
    margin: 0;
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-button {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: var(--space-sm);
    border-radius: var(--radius-sm);
    transition: var(--transition-fast);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-button:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .presets {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-lg) var(--space-xl);
    border-bottom: 1px solid var(--border);
    flex-wrap: wrap;
  }

  .presets-label {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    font-weight: 600;
  }

  .preset-button {
    padding: var(--space-sm) var(--space-lg);
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: var(--font-size-xs);
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .preset-button:hover {
    background: var(--bg-hover);
    border-color: var(--border-light);
  }

  .form {
    padding: var(--space-xl);
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
  }

  .error-banner {
    padding: var(--space-md) var(--space-lg);
    background: var(--error-bg);
    border: 1px solid var(--error);
    border-radius: var(--radius-sm);
    color: var(--error);
    font-size: var(--font-size-sm);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .form-group label {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .form-group input {
    padding: var(--space-md);
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    transition: var(--transition-fast);
  }

  .form-group input:focus {
    outline: none;
    border-color: var(--primary);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .form-group input::placeholder {
    color: var(--text-tertiary);
  }

  .footer {
    display: flex;
    gap: var(--space-md);
    padding: var(--space-xl);
    border-top: 1px solid var(--border);
  }

  .btn {
    flex: 1;
    padding: var(--space-md) var(--space-xl);
    border: 1px solid;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-cancel {
    background: transparent;
    border-color: var(--border);
    color: var(--text-secondary);
  }

  .btn-cancel:hover:not(:disabled) {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn-primary {
    background: var(--primary);
    border-color: var(--primary);
    color: var(--primary-foreground);
  }

  .btn-primary:hover:not(:disabled) {
    opacity: 0.9;
  }

  .templates-section {
    padding: var(--space-lg) var(--space-xl);
    border-bottom: 1px solid var(--border);
  }

  .templates-toggle {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-md);
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition-fast);
    border-radius: var(--radius-sm);
  }

  .templates-toggle:hover {
    background: var(--bg-hover);
  }

  .templates-toggle :global(.rotate-180) {
    transform: rotate(180deg);
    transition: transform 0.2s ease;
  }

  .templates-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: var(--space-sm);
    margin-top: var(--space-md);
    max-height: 300px;
    overflow-y: auto;
    padding: var(--space-xs);
  }

  .template-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-xs);
    padding: var(--space-md);
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: var(--font-size-xs);
    text-align: left;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .template-card:hover {
    background: var(--bg-hover);
    border-color: var(--border-light);
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .template-card.selected {
    background: var(--primary);
    border-color: var(--primary);
    color: var(--primary-foreground);
  }

  .template-name {
    font-weight: 600;
    font-size: var(--font-size-sm);
  }

  .template-desc {
    font-size: var(--font-size-xs);
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .template-card.selected .template-desc {
    color: var(--primary-foreground);
    opacity: 0.9;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    cursor: pointer;
    user-select: none;
  }

  .checkbox-label input[type='checkbox'] {
    width: 18px;
    height: 18px;
    cursor: pointer;
    accent-color: var(--primary);
  }

  .checkbox-label span {
    font-size: var(--font-size-sm);
    color: var(--text-primary);
    font-weight: 500;
  }
</style>
