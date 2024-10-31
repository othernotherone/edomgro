<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let content = "";
  let parsedContent = "";

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
        bind:value={content}
        placeholder="Enter org-mode content here..."
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
  :global(body) {
    margin: 0;
    padding: 0;
    background-color: #1e1e1e;
    color: #ffffff;
  }

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
    color: #ffffff;
  }

  h2 {
    margin-top: 0;
    margin-bottom: 10px;
    color: #ffffff;
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
    border: 1px solid #333;
    border-radius: 4px;
    background-color: #2d2d2d;
    color: #ffffff;
  }

  textarea:focus {
    outline: none;
    border-color: #0078d4;
  }

  pre {
    flex: 1;
    margin: 0;
    padding: 10px;
    background: #2d2d2d;
    border: 1px solid #333;
    border-radius: 4px;
    overflow: auto;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    font-size: 14px;
    color: #ffffff;
  }
</style> 