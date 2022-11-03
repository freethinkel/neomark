<script lang="ts">
  import { unified } from "unified";
  import remarkParse from "remark-parse";
  import remarkRehype from "remark-rehype";
  import rehypeStringify from "rehype-stringify";
  import rehypeHighlight from "rehype-highlight";
  import remarkGfm from "remark-gfm";

  import highlight from "highlight.js/lib/common";

  import "./styles.pcss";

  export let content = "";
  let rendered = "";

  $: {
    process(content).then((content) => {
      console.log(content);
      rendered = content;
    });
  }

  const process = async (content: string) => {
    const file = await unified()
      .use(remarkParse)
      .use(remarkGfm)
      .use(remarkRehype)
      .use(rehypeHighlight, {
        ignoreMissing: true,
        languages: highlight.listLanguages().reduce((acc, curr) => {
          acc[curr] = (_) => highlight.getLanguage(curr);
          return acc;
        }, {}),
      })
      .use(rehypeStringify)
      .process(content);

    return String(file);
  };
</script>

<div class="preview markdown-body">
  {@html rendered}
</div>

<style lang="postcss">
  .preview {
    height: 100%;
    min-height: inherit;
    padding: 10px;
  }
</style>
