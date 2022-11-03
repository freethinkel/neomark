<script>
  import { appWindow } from "@tauri-apps/api/window";
  import { createEventDispatcher, onMount } from "svelte";

  const dispatch = createEventDispatcher();

  const listen = async () => {
    await appWindow.onFileDropEvent((event) => {
      if (event.payload.type === "drop") {
        const [filePath] = event.payload.paths;
        if (filePath) {
          dispatch("select", filePath);
        }
      }
    });
  };

  onMount(() => {
    listen();
  });
</script>

<div class="wrapper">
  <div class="inner">
    <h1>Drop a markdown file</h1>
  </div>
</div>

<style>
  .wrapper {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: inherit;
    padding: 12px;
  }

  .inner {
    border: 1px dashed var(--color-border);
    padding: 24px;
    border-radius: 10px;
  }
</style>
