<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let content = $state("");
  let parsedContent = $state("");

  async function testParser() {
    try {
      parsedContent = await invoke("test_parser");
    } catch (e) {
      console.error(e);
      parsedContent = "Error parsing content";
    }
  }

  async function parseContent() {
    try {
      parsedContent = await invoke("parse_org_content", { content });
    } catch (e) {
      console.error(e);
      parsedContent = "Error parsing content";
    }
  }

  onMount(() => {
    testParser();
  });
</script>

<main class="container">
  <header>
    <h1>Org Mode Editor</h1>
  </header>

  <div class="editor-container">
    <div class="editor-section">
      <h2>Editor</h2>
      <textarea
        placeholder="Enter org-mode content here..."
        bind:value={content}
        on:input={parseContent}
      />
    </div>

    <div class="parsed-section">
      <h2>Parsed Structure</h2>
      <pre>{parsedContent}</pre>
    </div>
  </div>
</main>

<style>
  .container {
    margin: 0;
    padding: 20px;
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  header {
    padding-bottom: 20px;
  }

  h1 {
    text-align: center;
    margin: 0;
    color: var(--text-color);
  }

  h2 {
    margin-top: 0;
    margin-bottom: 10px;
    color: var(--text-color);
  }

  .editor-container {
    display: flex;
    gap: 20px;
    flex: 1;
  }

  .editor-section, .parsed-section {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  textarea {
    flex: 1;
    padding: 10px;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    font-size: 14px;
    resize: none;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    background-color: var(--editor-bg);
    color: var(--text-color);
  }

  textarea:focus {
    outline: none;
    border-color: var(--accent-color);
  }

  pre {
    flex: 1;
    margin: 0;
    padding: 10px;
    background: var(--editor-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    overflow: auto;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    font-size: 14px;
    color: var(--text-color);
  }

  :global(:root) {
    --text-color: #ffffff;
    --bg-color: #1e1e1e;
    --editor-bg: #2d2d2d;
    --border-color: #333;
    --accent-color: #0078d4;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--bg-color);
    color: var(--text-color);
  }

  :global(*) {
    box-sizing: border-box;
  }

  :global(::-webkit-scrollbar) {
    width: 10px;
    height: 10px;
  }

  :global(::-webkit-scrollbar-track) {
    background: var(--bg-color);
  }

  :global(::-webkit-scrollbar-thumb) {
    background: #444;
    border-radius: 5px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: #555;
  }
</style>
