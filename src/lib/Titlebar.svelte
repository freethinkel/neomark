<div class="wrapper" data-tauri-drag-region>
  <div class="title">
    <slot/>
  </div>
</div>

<style>
  .wrapper {
    height: 32px;
    padding: 0 70px;
    display: flex;
    justify-content: center;
    align-items: center;
    border-bottom: 1px solid var(--color-border);
    cursor: default;
    user-select: none;
    opacity: 0.9;
    font-size: 0.9rem;
    font-weight: 500;
    white-space: nowrap;
  }
  .title {
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: center;
    max-width: 100%;
    min-width: 0;

  }
</style>

