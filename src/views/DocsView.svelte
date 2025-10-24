<script lang="ts">
  import { Search, ExternalLink, ChevronRight } from 'lucide-svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import { docs, type DocSection, type DocArticle } from '$lib/data/docs';

  let searchQuery = $state('');
  let selectedSection = $state<DocSection | null>(docs[0]);
  let selectedArticle = $state<DocArticle | null>(docs[0].content[0]);
  let showApiDocs = $state(false);
  let apiDocsPath = $state('');

  // Load Rust API docs
  async function loadApiDocs() {
    // For now, point to a relative path - in production this would be bundled
    // The Rust docs are generated at build time
    apiDocsPath = '/doc/sentinel/index.html';
  }

  // Handle section click
  function selectSection(section: DocSection) {
    selectedSection = section;
    selectedArticle = section.content[0];
    showApiDocs = false;
  }

  // Handle article click
  function selectArticle(article: DocArticle) {
    selectedArticle = article;
  }

  // Toggle API docs view
  function toggleApiDocs() {
    showApiDocs = !showApiDocs;
    if (showApiDocs && !apiDocsPath) {
      loadApiDocs();
    }
  }

  // Filter articles based on search
  let filteredSections = $derived.by(() => {
    if (!searchQuery.trim()) return docs;

    const query = searchQuery.toLowerCase();
    return docs
      .map((section) => ({
        ...section,
        content: section.content.filter(
          (article) =>
            article.title.toLowerCase().includes(query) ||
            article.content.toLowerCase().includes(query) ||
            article.tags?.some((tag) => tag.toLowerCase().includes(query))
        )
      }))
      .filter((section) => section.content.length > 0);
  });

  // Render markdown with improved formatting
  function renderMarkdown(content: string): string {
    const lines = content.split('\n');
    let html = '';
    let i = 0;

    while (i < lines.length) {
      const line = lines[i].trim();

      // Skip empty lines
      if (!line) {
        i++;
        continue;
      }

      // Code blocks
      if (line.startsWith('```')) {
        let code = '';
        i++;
        while (i < lines.length && !lines[i].trim().startsWith('```')) {
          code += lines[i] + '\n';
          i++;
        }
        html += `<pre><code>${code.trim()}</code></pre>`;
        i++;
        continue;
      }

      // Headers
      if (line.startsWith('### ')) {
        html += `<h3>${line.slice(4)}</h3>`;
        i++;
        continue;
      }
      if (line.startsWith('## ')) {
        html += `<h2>${line.slice(3)}</h2>`;
        i++;
        continue;
      }
      if (line.startsWith('# ')) {
        html += `<h1>${line.slice(2)}</h1>`;
        i++;
        continue;
      }

      // Lists
      if (line.match(/^[-•]\s/) || line.match(/^\d+\.\s/)) {
        let items = '';
        while (i < lines.length) {
          const listLine = lines[i].trim();
          if (!listLine) {
            i++;
            break;
          }
          const match =
            listLine.match(/^[-•]\s+(.+)/) || listLine.match(/^\d+\.\s+(.+)/);
          if (match) {
            let text = match[1];
            text = text.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
            text = text.replace(/\*(.+?)\*/g, '<em>$1</em>');
            text = text.replace(/`(.+?)`/g, '<code>$1</code>');
            items += `<li>${text}</li>`;
            i++;
          } else {
            break;
          }
        }
        html += `<ul>${items}</ul>`;
        continue;
      }

      // Paragraphs
      let paragraph = line;
      i++;
      while (
        i < lines.length &&
        lines[i].trim() &&
        !lines[i].trim().match(/^(#{1,3}\s|[-•]\s|\d+\.\s|```)/)
      ) {
        paragraph += ' ' + lines[i].trim();
        i++;
      }
      paragraph = paragraph.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
      paragraph = paragraph.replace(/\*(.+?)\*/g, '<em>$1</em>');
      paragraph = paragraph.replace(/`(.+?)`/g, '<code>$1</code>');
      html += `<p>${paragraph}</p>`;
    }

    return html;
  }
</script>

<div class="docs-view">
  <PageHeader title="Documentation" subtitle="Help, guides, and API reference">
    <svelte:fragment slot="actions">
      <button
        class="api-docs-toggle"
        class:active={showApiDocs}
        onclick={toggleApiDocs}
      >
        <ExternalLink size={16} />
        {showApiDocs ? 'User Docs' : 'API Docs'}
      </button>
    </svelte:fragment>
  </PageHeader>

  <div class="docs-container">
    {#if showApiDocs}
      <!-- API Documentation iframe -->
      <div class="api-docs-container">
        {#if apiDocsPath}
          <iframe
            src={apiDocsPath}
            title="Sentinel API Documentation"
            class="api-docs-frame"
          ></iframe>
        {:else}
          <div class="loading-state">
            <p>Loading API documentation...</p>
          </div>
        {/if}
      </div>
    {:else}
      <!-- User Documentation -->
      <div class="docs-layout">
        <!-- Left Sidebar -->
        <aside class="docs-sidebar">
          <!-- Search -->
          <div class="search-box">
            <Search size={18} class="search-icon" />
            <input
              type="text"
              placeholder="Search documentation..."
              bind:value={searchQuery}
              class="search-input"
            />
          </div>

          <!-- Sections List -->
          <nav class="sections-nav">
            {#each filteredSections as section (section.id)}
              {@const Icon = section.icon}
              <div class="section-group">
                <button
                  class="section-button"
                  class:active={selectedSection?.id === section.id}
                  onclick={() => selectSection(section)}
                >
                  <Icon size={18} />
                  <span>{section.title}</span>
                  <ChevronRight size={14} />
                </button>

                {#if selectedSection?.id === section.id}
                  <div class="articles-list">
                    {#each section.content as article (article.id)}
                      <button
                        class="article-button"
                        class:active={selectedArticle?.id === article.id}
                        onclick={() => selectArticle(article)}
                      >
                        {article.title}
                      </button>
                    {/each}
                  </div>
                {/if}
              </div>
            {/each}
          </nav>
        </aside>

        <!-- Main Content -->
        <main class="docs-content">
          {#if selectedArticle}
            <article class="article">
              <!-- Breadcrumb -->
              {#if selectedSection}
                {@const Icon = selectedSection.icon}
                <div class="breadcrumb">
                  <Icon size={14} />
                  <span>{selectedSection.title}</span>
                  <ChevronRight size={12} />
                  <span class="current">{selectedArticle.title}</span>
                </div>
              {/if}

              <h1 class="article-title">{selectedArticle.title}</h1>

              {#if selectedArticle.tags}
                <div class="tags">
                  {#each selectedArticle.tags as tag (tag)}
                    <span class="tag">{tag}</span>
                  {/each}
                </div>
              {/if}

              <div class="article-body">
                {@html renderMarkdown(selectedArticle.content)}
              </div>
            </article>
          {:else}
            <div class="empty-state">
              <Search size={48} />
              <p>No documentation found</p>
              <span>Try adjusting your search query</span>
            </div>
          {/if}
        </main>
      </div>
    {/if}
  </div>
</div>

<style>
  .docs-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--background);
  }

  .docs-container {
    flex: 1;
    overflow: hidden;
    padding: var(--space-xs);
  }

  .api-docs-toggle {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    padding: var(--space-xs) var(--space-md);
    background: var(--muted);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--foreground);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .api-docs-toggle:hover {
    background: var(--accent);
    color: var(--accent-foreground);
  }

  .api-docs-toggle.active {
    background: var(--primary);
    color: var(--primary-foreground);
  }

  /* API Docs */
  .api-docs-container {
    height: 100%;
    background: var(--card);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .api-docs-frame {
    width: 100%;
    height: 100%;
    border: none;
  }

  .loading-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--muted-foreground);
  }

  /* Docs Layout */
  .docs-layout {
    display: grid;
    grid-template-columns: 240px 1fr;
    gap: var(--space-xs);
    height: 100%;
  }

  /* Sidebar */
  .docs-sidebar {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    height: 100%;
    overflow: hidden;
  }

  .search-box {
    position: relative;
  }

  .search-box :global(.search-icon) {
    position: absolute;
    left: var(--space-sm);
    top: 50%;
    transform: translateY(-50%);
    color: var(--muted-foreground);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding: var(--space-xs) var(--space-sm) var(--space-xs) 2.5rem;
    background: var(--muted);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    font-size: 0.875rem;
    color: var(--foreground);
    transition: all var(--transition-fast);
  }

  .search-input:focus {
    outline: none;
    background: var(--background);
    border-color: var(--ring);
  }

  .sections-nav {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }

  .section-group {
    margin-bottom: 2px;
  }

  .section-button {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    width: 100%;
    padding: 5px var(--space-xs);
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--foreground);
    text-align: left;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .section-button:hover {
    background: var(--muted);
  }

  .section-button.active {
    background: var(--accent);
    color: var(--accent-foreground);
  }

  .section-button :global(svg:last-child) {
    margin-left: auto;
    transition: transform var(--transition-fast);
  }

  .section-button.active :global(svg:last-child) {
    transform: rotate(90deg);
  }

  .articles-list {
    display: flex;
    flex-direction: column;
    gap: 1px;
    padding-left: var(--space-sm);
    margin-top: 1px;
    margin-bottom: 2px;
  }

  .article-button {
    padding: 3px var(--space-xs);
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    font-size: 0.688rem;
    color: var(--muted-foreground);
    text-align: left;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .article-button:hover {
    color: var(--foreground);
    background: var(--muted);
  }

  .article-button.active {
    color: var(--primary);
    background: var(--primary-foreground);
    font-weight: 600;
  }

  /* Content */
  .docs-content {
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    background: var(--card);
    border-radius: var(--radius-lg);
    padding: var(--space-md) var(--space-lg);
  }

  .article {
    max-width: 850px;
  }

  .breadcrumb {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 0.688rem;
    color: var(--muted-foreground);
    margin-bottom: var(--space-xs);
    padding: 2px 0;
  }

  .breadcrumb .current {
    color: var(--foreground);
    font-weight: 500;
  }

  .article-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--foreground);
    margin-bottom: var(--space-xs);
    line-height: 1.2;
  }

  .tags {
    display: flex;
    gap: 4px;
    margin-bottom: var(--space-sm);
    flex-wrap: wrap;
  }

  .tag {
    padding: 2px var(--space-xs);
    background: var(--muted);
    border-radius: var(--radius-sm);
    font-size: 0.688rem;
    font-weight: 600;
    color: var(--muted-foreground);
  }

  .article-body {
    line-height: 1.5;
    color: var(--foreground);
    font-size: 0.875rem;
  }

  .article-body :global(h1),
  .article-body :global(h2),
  .article-body :global(h3) {
    margin: 0;
    margin-top: 12px;
    margin-bottom: 6px;
    font-weight: 600;
    color: var(--foreground);
    line-height: 1.3;
  }

  .article-body :global(h1) {
    font-size: 1.375rem;
    margin-top: 16px;
  }

  .article-body :global(h1:first-child),
  .article-body :global(h2:first-child),
  .article-body :global(h3:first-child) {
    margin-top: 0;
  }

  .article-body :global(h2) {
    font-size: 1.125rem;
  }

  .article-body :global(h3) {
    font-size: 1rem;
  }

  .article-body :global(p) {
    margin: 0;
    margin-bottom: 8px;
  }

  .article-body :global(p:last-child) {
    margin-bottom: 0;
  }

  .article-body :global(ul),
  .article-body :global(ol) {
    margin: 0;
    margin-bottom: 8px;
    padding-left: 20px;
  }

  .article-body :global(ul:last-child),
  .article-body :global(ol:last-child) {
    margin-bottom: 0;
  }

  .article-body :global(li) {
    margin: 0;
    padding: 0;
    line-height: 1.6;
  }

  .article-body :global(code) {
    padding: 2px 6px;
    background: var(--muted);
    border-radius: var(--radius-sm);
    font-family: 'SF Mono', 'Consolas', monospace;
    font-size: 0.875em;
    color: var(--primary);
  }

  .article-body :global(pre) {
    padding: var(--space-xs) var(--space-sm);
    background: var(--muted);
    border-radius: var(--radius-md);
    overflow-x: auto;
    margin: var(--space-xs) 0;
    font-size: 0.75rem;
  }

  .article-body :global(pre code) {
    padding: 0;
    background: transparent;
    color: var(--foreground);
  }

  .article-body :global(strong) {
    font-weight: 600;
    color: var(--foreground);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--muted-foreground);
    gap: var(--space-sm);
  }

  .empty-state p {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--foreground);
    margin: 0;
  }

  .empty-state span {
    font-size: 0.875rem;
  }
</style>
