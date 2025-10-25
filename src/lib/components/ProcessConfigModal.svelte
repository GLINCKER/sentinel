<script lang="ts">
  import {
    X,
    Folder,
    Plus,
    Trash2,
    GitBranch,
    Sparkles,
    Search,
    Package,
    Zap,
    Coffee,
    Code2
  } from 'lucide-svelte';
  import { toast } from 'svelte-sonner';
  import { processConfigStore } from '../../stores/processConfig.svelte';
  import type {
    ProcessConfig,
    ProcessTemplate,
    DetectedProject
  } from '../../stores/processConfig.svelte';
  import { open } from '@tauri-apps/plugin-dialog';

  interface Props {
    show: boolean;
    editingConfig?: ProcessConfig | null;
    onClose: () => void;
    onSaved?: () => void;
  }

  let { show, editingConfig = null, onClose, onSaved }: Props = $props();

  // Form state
  let name = $state('');
  let command = $state('');
  let args = $state('');
  let workingDir = $state('');
  let port = $state<string>('');
  let healthCheckUrl = $state('');
  let autoStart = $state(false);
  let envVars = $state<Array<{ key: string; value: string }>>([]);
  let loading = $state(false);

  // Project discovery state
  let isScanning = $state(false);
  let detectedProjects = $state<DetectedProject[]>([]);
  let showProjectPicker = $state(false);
  let selectedProject = $state<DetectedProject | null>(null);

  // Template state (kept for manual mode)
  let selectedTemplate = $state<ProcessTemplate | null>(null);

  // Mode toggle
  let mode = $state<'auto' | 'manual'>('auto');

  // Load config or reset on mount
  $effect(() => {
    if (show) {
      if (editingConfig) {
        loadConfigData(editingConfig);
      } else {
        resetForm();
      }
    }
  });

  function loadConfigData(config: ProcessConfig) {
    name = config.name;
    command = config.command;
    args = config.args.join(' ');
    workingDir = config.workingDir;
    port = config.port ? String(config.port) : '';
    healthCheckUrl = config.healthCheckUrl || '';
    autoStart = config.autoStart;
    envVars = Object.entries(config.envVars).map(([key, value]) => ({
      key,
      value
    }));
  }

  function resetForm() {
    name = '';
    command = '';
    args = '';
    workingDir = '';
    port = '';
    healthCheckUrl = '';
    autoStart = false;
    envVars = [];
    selectedTemplate = null;
  }

  async function handleBrowseFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Working Directory'
      });
      if (selected) {
        workingDir = selected;
      }
    } catch (err) {
      console.error('Failed to open folder dialog:', err);
    }
  }

  async function handleScanDirectory() {
    if (!workingDir.trim()) {
      toast.error('Please select a directory first');
      return;
    }

    try {
      isScanning = true;
      const projects = await processConfigStore.scanDirectory(
        workingDir.trim()
      );

      if (projects.length === 0) {
        toast.info('No projects detected in this directory');
        detectedProjects = [];
        showProjectPicker = false;
      } else if (projects.length === 1) {
        // Auto-select single project
        applyDetectedProject(projects[0]);
        toast.success(`Detected ${projects[0].frameworkType} project`);
      } else {
        // Show picker for multiple projects (monorepo)
        detectedProjects = projects;
        showProjectPicker = true;
        toast.info(`Found ${projects.length} projects - select one`);
      }
    } catch (err) {
      toast.error(`Project scan failed: ${err}`);
    } finally {
      isScanning = false;
    }
  }

  function applyDetectedProject(project: DetectedProject) {
    selectedProject = project;
    name = project.name;
    workingDir = project.path;
    command = project.suggestedCommand;
    args = project.suggestedArgs.join(' ');
    if (project.suggestedPort) {
      port = String(project.suggestedPort);
    }
    // Auto-load environment variables from .env file
    if (project.envVars && Object.keys(project.envVars).length > 0) {
      envVars = Object.entries(project.envVars).map(([key, value]) => ({
        key,
        value
      }));
    }
    showProjectPicker = false;
  }

  function addEnvVar() {
    envVars = [...envVars, { key: '', value: '' }];
  }

  function removeEnvVar(index: number) {
    envVars = envVars.filter((_, i) => i !== index);
  }

  async function handleSave() {
    if (!name.trim() || !command.trim()) {
      toast.error('Name and command are required');
      return;
    }

    try {
      loading = true;

      const argsArray = args
        .trim()
        .split(/\s+/)
        .filter((a) => a.length > 0);

      const envVarsObj = envVars.reduce(
        (acc, { key, value }) => {
          if (key.trim()) {
            acc[key.trim()] = value;
          }
          return acc;
        },
        {} as Record<string, string>
      );

      if (editingConfig) {
        // Update existing config
        const config: ProcessConfig = {
          ...editingConfig,
          name: name.trim(),
          command: command.trim(),
          args: argsArray,
          workingDir: workingDir.trim() || '.',
          envVars: envVarsObj,
          frameworkType:
            selectedTemplate?.frameworkType ||
            selectedProject?.frameworkType ||
            editingConfig?.frameworkType,
          port: port ? parseInt(port) : undefined,
          autoStart,
          healthCheckUrl: healthCheckUrl.trim() || undefined
        };
        await processConfigStore.updateConfig(config);
        toast.success('Process configuration updated');
      } else {
        // Create new config (omit id and timestamps)
        const config = {
          name: name.trim(),
          command: command.trim(),
          args: argsArray,
          workingDir: workingDir.trim() || '.',
          envVars: envVarsObj,
          frameworkType:
            selectedTemplate?.frameworkType || selectedProject?.frameworkType,
          port: port ? parseInt(port) : undefined,
          autoStart,
          healthCheckUrl: healthCheckUrl.trim() || undefined
        };
        await processConfigStore.createConfig(config);
        toast.success('Process configuration created');
      }

      if (onSaved) {
        onSaved();
      }

      onClose();
      resetForm();
    } catch (err) {
      toast.error(`Failed to save configuration: ${err}`);
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
    >
      <!-- Header -->
      <div class="header">
        <h2 class="title">
          {editingConfig
            ? 'Edit Process Configuration'
            : 'New Process Configuration'}
        </h2>
        <button class="close-button" onclick={onClose} title="Close">
          <X size={20} />
        </button>
      </div>

      <!-- Body -->
      <div class="body">
        <!-- Mode Toggle -->
        {#if !editingConfig}
          <div class="mode-toggle">
            <button
              class="mode-btn"
              class:active={mode === 'auto'}
              onclick={() => (mode = 'auto')}
            >
              <Sparkles size={16} />
              Auto-Detect Project
            </button>
            <button
              class="mode-btn"
              class:active={mode === 'manual'}
              onclick={() => (mode = 'manual')}
            >
              <Code2 size={16} />
              Manual Configuration
            </button>
          </div>
        {/if}

        <!-- AUTO MODE: Project Discovery -->
        {#if mode === 'auto' && !editingConfig}
          <div class="auto-mode-section">
            <div class="info-banner">
              <GitBranch size={18} />
              <div class="info-text">
                <strong>Automatic Project Discovery</strong>
                <p>
                  Browse for a Git repository or project folder. Sentinel will
                  automatically detect framework type, start command, and
                  configuration.
                </p>
              </div>
            </div>

            <!-- Directory Selector -->
            <div class="form-group">
              <label class="label" for="workingDirAuto">Project Directory</label
              >
              <div class="input-group">
                <input
                  id="workingDirAuto"
                  type="text"
                  class="input"
                  placeholder="Select a directory..."
                  bind:value={workingDir}
                  readonly
                />
                <button
                  class="input-btn"
                  onclick={handleBrowseFolder}
                  title="Browse"
                >
                  <Folder size={16} />
                  Browse
                </button>
              </div>
            </div>

            <!-- Scan Button -->
            {#if workingDir}
              <button
                class="scan-btn"
                onclick={handleScanDirectory}
                disabled={isScanning}
              >
                {#if isScanning}
                  <div class="spinner"></div>
                  Scanning directory...
                {:else}
                  <Search size={18} />
                  Scan for Projects
                {/if}
              </button>
            {/if}

            <!-- Detected Projects Picker (for monorepos) -->
            {#if showProjectPicker && detectedProjects.length > 0}
              <div class="form-group">
                <label class="label"
                  >Detected Projects ({detectedProjects.length})</label
                >
                <div class="project-list">
                  {#each detectedProjects as project (project.path)}
                    <button
                      class="project-item"
                      onclick={() => applyDetectedProject(project)}
                    >
                      <div class="project-icon">
                        {#if project.frameworkType === 'NextJs'}
                          <Package size={20} />
                        {:else if project.frameworkType === 'Vite'}
                          <Zap size={20} />
                        {:else if project.frameworkType === 'FastAPI' || project.frameworkType === 'Flask' || project.frameworkType === 'Django'}
                          <Code2 size={20} />
                        {:else if project.frameworkType === 'SpringBoot'}
                          <Coffee size={20} />
                        {:else}
                          <Package size={20} />
                        {/if}
                      </div>
                      <div class="project-info">
                        <div class="project-name">{project.name}</div>
                        <div class="project-details">
                          {project.frameworkType} • {Math.round(
                            project.confidence * 100
                          )}% confidence
                        </div>
                        <div class="project-path">{project.path}</div>
                      </div>
                    </button>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Auto-filled Configuration Preview -->
            {#if selectedProject}
              <div class="detected-config">
                <div class="detected-header">
                  <Sparkles size={16} />
                  <span>Auto-detected Configuration</span>
                </div>
                <div class="detected-grid">
                  <div class="detected-item">
                    <span class="detected-label">Framework:</span>
                    <span class="detected-value"
                      >{selectedProject.frameworkType}</span
                    >
                  </div>
                  <div class="detected-item">
                    <span class="detected-label">Command:</span>
                    <span class="detected-value"
                      >{selectedProject.suggestedCommand}</span
                    >
                  </div>
                  <div class="detected-item">
                    <span class="detected-label">Arguments:</span>
                    <span class="detected-value"
                      >{selectedProject.suggestedArgs.join(' ')}</span
                    >
                  </div>
                  {#if selectedProject.suggestedPort}
                    <div class="detected-item">
                      <span class="detected-label">Port:</span>
                      <span class="detected-value"
                        >{selectedProject.suggestedPort}</span
                      >
                    </div>
                  {/if}
                </div>
              </div>
            {/if}
          </div>
        {/if}

        <!-- MANUAL MODE: Traditional Form -->
        {#if mode === 'manual' || editingConfig}
          <!-- Name -->
          <div class="form-group">
            <label class="label" for="name">Process Name *</label>
            <input
              id="name"
              type="text"
              class="input"
              placeholder="my-app"
              bind:value={name}
              required
            />
          </div>

          <!-- Working Directory -->
          <div class="form-group">
            <label class="label" for="workingDir">Working Directory</label>
            <div class="input-group">
              <input
                id="workingDir"
                type="text"
                class="input"
                placeholder="/path/to/project"
                bind:value={workingDir}
              />
              <button
                class="input-btn"
                onclick={handleBrowseFolder}
                title="Browse"
              >
                <Folder size={16} />
                Browse
              </button>
            </div>
          </div>

          <!-- Command -->
          <div class="form-group">
            <label class="label" for="command">Command *</label>
            <input
              id="command"
              type="text"
              class="input"
              placeholder="npm, python, java, etc."
              bind:value={command}
              required
            />
          </div>

          <!-- Arguments -->
          <div class="form-group">
            <label class="label" for="args">Arguments</label>
            <input
              id="args"
              type="text"
              class="input"
              placeholder="run dev (space-separated)"
              bind:value={args}
            />
            <div class="hint">Space-separated arguments</div>
          </div>

          <!-- Port & Health Check -->
          <div class="form-row">
            <div class="form-group">
              <label class="label" for="port">Port (Optional)</label>
              <input
                id="port"
                type="number"
                class="input"
                placeholder="3000"
                bind:value={port}
              />
            </div>

            <div class="form-group">
              <label class="label" for="healthCheckUrl"
                >Health Check URL (Optional)</label
              >
              <input
                id="healthCheckUrl"
                type="text"
                class="input"
                placeholder="http://localhost:3000"
                bind:value={healthCheckUrl}
              />
            </div>
          </div>

          <!-- Environment Variables -->
          <div class="form-group">
            <div class="label-with-action">
              <label class="label">Environment Variables</label>
              <button class="add-btn" onclick={addEnvVar}>
                <Plus size={14} />
                Add
              </button>
            </div>
            {#if envVars.length > 0}
              <div class="env-vars">
                {#each envVars as envVar, index (index)}
                  <div class="env-var-row">
                    <input
                      type="text"
                      class="input env-key"
                      placeholder="KEY"
                      bind:value={envVar.key}
                    />
                    <input
                      type="text"
                      class="input env-value"
                      placeholder="value"
                      bind:value={envVar.value}
                    />
                    <button
                      class="remove-btn"
                      onclick={() => removeEnvVar(index)}
                    >
                      <Trash2 size={14} />
                    </button>
                  </div>
                {/each}
              </div>
            {:else}
              <div class="empty-hint">
                No environment variables. Click "Add" to create one.
              </div>
            {/if}
          </div>

          <!-- Auto-start -->
          <div class="form-group">
            <label class="checkbox-label">
              <input
                type="checkbox"
                class="checkbox"
                bind:checked={autoStart}
              />
              <span>Auto-start when Sentinel launches</span>
            </label>
          </div>
        {/if}

        <!-- Common fields for both modes -->
        {#if (mode === 'auto' && selectedProject) || mode === 'manual' || editingConfig}
          <div class="form-group">
            <label class="label" for="nameField">Process Name *</label>
            <input
              id="nameField"
              type="text"
              class="input"
              placeholder="my-app"
              bind:value={name}
              required
            />
          </div>

          <!-- Auto-start -->
          <div class="form-group">
            <label class="checkbox-label">
              <input
                type="checkbox"
                class="checkbox"
                bind:checked={autoStart}
              />
              <span>Auto-start when Sentinel launches</span>
            </label>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="footer">
        <button class="btn btn-cancel" onclick={onClose} disabled={loading}
          >Cancel</button
        >
        <button
          class="btn btn-primary"
          onclick={handleSave}
          disabled={loading ||
            (mode === 'auto' && !selectedProject && !editingConfig)}
        >
          {loading ? 'Saving...' : editingConfig ? 'Update' : 'Create Process'}
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
    max-width: 700px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-lg) var(--space-xl);
    border-bottom: 1px solid var(--border);
  }

  .title {
    margin: 0;
    font-size: 16px;
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

  .body {
    padding: var(--space-lg) var(--space-xl);
    overflow-y: auto;
    flex: 1;
  }

  .form-group {
    margin-bottom: var(--space-md);
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: var(--space-md);
    margin-bottom: var(--space-lg);
  }

  .label {
    display: block;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 6px;
  }

  .label-with-action {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-sm);
  }

  .input {
    width: 100%;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: 12px;
    transition: var(--transition-fast);
  }

  .input:focus {
    outline: none;
    border-color: var(--accent-primary);
    background: var(--bg-tertiary);
  }

  .input::placeholder {
    color: var(--text-tertiary);
  }

  .input-group {
    display: flex;
    gap: var(--space-sm);
  }

  .input-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition-fast);
    white-space: nowrap;
  }

  .input-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .input-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .hint {
    margin-top: 4px;
    font-size: 11px;
    color: var(--text-tertiary);
  }

  .template-selector {
    width: 100%;
    padding: var(--space-md);
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    text-align: left;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .template-selector:hover {
    border-color: var(--accent-primary);
  }

  .template-list {
    margin-top: var(--space-sm);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    background: var(--bg-secondary);
    max-height: 300px;
    overflow-y: auto;
  }

  .template-item {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    width: 100%;
    padding: var(--space-md);
    background: transparent;
    border: none;
    border-bottom: 1px solid var(--border);
    text-align: left;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .template-item:last-child {
    border-bottom: none;
  }

  .template-item:hover {
    background: var(--bg-hover);
  }

  .template-icon {
    font-size: 24px;
    flex-shrink: 0;
  }

  .template-info {
    flex: 1;
    min-width: 0;
  }

  .template-name {
    font-weight: 600;
    color: var(--text-primary);
    font-size: var(--font-size-sm);
  }

  .template-desc {
    font-size: var(--font-size-xs);
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .env-vars {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .env-var-row {
    display: grid;
    grid-template-columns: 1fr 2fr auto;
    gap: var(--space-sm);
    align-items: center;
  }

  .env-key,
  .env-value {
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
  }

  .add-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 10px;
    background: var(--accent-primary);
    border: none;
    border-radius: var(--radius-sm);
    color: white;
    font-size: var(--font-size-xs);
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .add-btn:hover {
    background: var(--accent-hover);
  }

  .remove-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    background: transparent;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .remove-btn:hover {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgb(239, 68, 68);
    color: rgb(239, 68, 68);
  }

  .empty-hint {
    padding: var(--space-md);
    background: var(--bg-secondary);
    border: 1px dashed var(--border);
    border-radius: var(--radius-md);
    text-align: center;
    color: var(--text-tertiary);
    font-size: var(--font-size-sm);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    cursor: pointer;
    color: var(--text-primary);
    font-size: var(--font-size-sm);
  }

  .checkbox {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .footer {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    padding: var(--space-lg) var(--space-xl);
    border-top: 1px solid var(--border);
    background: var(--bg-secondary);
  }

  .btn {
    padding: 8px 16px;
    border-radius: var(--radius-md);
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition-fast);
    border: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-cancel {
    background: var(--bg-secondary);
    color: var(--text-secondary);
    border: 1px solid var(--border);
  }

  .btn-cancel:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--text-primary);
  }

  .btn-primary {
    background: var(--accent-primary);
    color: white;
    border: 1px solid var(--accent-primary);
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--accent-hover);
    border-color: var(--accent-hover);
  }

  /* Mode Toggle */
  .mode-toggle {
    display: flex;
    gap: var(--space-sm);
    margin-bottom: var(--space-xl);
    padding: 4px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
  }

  .mode-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    padding: var(--space-md);
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .mode-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .mode-btn.active {
    background: var(--accent-primary);
    color: white;
  }

  /* Auto Mode Section */
  .auto-mode-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .info-banner {
    display: flex;
    gap: 10px;
    padding: 10px 12px;
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: var(--radius-md);
    color: var(--text-primary);
  }

  .info-text strong {
    display: block;
    font-size: 12px;
    font-weight: 600;
    margin-bottom: 3px;
  }

  .info-text p {
    margin: 0;
    font-size: 11px;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .scan-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px 16px;
    background: var(--accent-primary);
    color: white;
    border: 1px solid var(--accent-primary);
    border-radius: var(--radius-md);
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .scan-btn:hover:not(:disabled) {
    background: var(--accent-hover);
    border-color: var(--accent-hover);
  }

  .scan-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Project List */
  .project-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
    max-height: 300px;
    overflow-y: auto;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    background: var(--bg-secondary);
  }

  .project-item {
    display: flex;
    align-items: flex-start;
    gap: var(--space-md);
    padding: var(--space-md);
    background: transparent;
    border: none;
    border-bottom: 1px solid var(--border);
    text-align: left;
    cursor: pointer;
    transition: var(--transition-fast);
  }

  .project-item:last-child {
    border-bottom: none;
  }

  .project-item:hover {
    background: var(--bg-hover);
  }

  .project-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    color: var(--accent-primary);
    flex-shrink: 0;
  }

  .project-info {
    flex: 1;
    min-width: 0;
  }

  .project-name {
    font-weight: 600;
    font-size: var(--font-size-sm);
    color: var(--text-primary);
    margin-bottom: 4px;
  }

  .project-details {
    font-size: var(--font-size-xs);
    color: var(--text-secondary);
    margin-bottom: 4px;
  }

  .project-path {
    font-size: 11px;
    color: var(--text-tertiary);
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* Detected Config Preview */
  .detected-config {
    padding: 10px 12px;
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    border-radius: var(--radius-md);
  }

  .detected-header {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    font-weight: 600;
    color: rgb(16, 185, 129);
    margin-bottom: 8px;
  }

  .detected-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 8px;
  }

  .detected-item {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .detected-label {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .detected-value {
    font-size: 12px;
    color: var(--text-primary);
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
  }
</style>
