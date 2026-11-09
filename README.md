# pandoc-fenced-figure

pandoc-fenced-figure is a [pandoc](https://pandoc.org/) filter that turns Pandoc Div element with `figure` class into `Figure` element:

```markdown
::: figure

````python
print("Hello World!")
````

The last paragraph in the div is turuned to caption.
:::
```

will be rendered to the following HTML:

```html
<figure class="figure">
    <div class="sourceCode" id="cb1">
        <pre
            class="sourceCode python"><code class="sourceCode python"><span id="cb1-1"><a href="#cb1-1" aria-hidden="true" tabindex="-1"></a><span class="bu">print</span>(<span class="st">&quot;Hello World!&quot;</span>)</span></code></pre>
    </div>
    <figcaption>Caption: Marks a paragraph used as figcaption.</figcaption>
</figure>
```

It should also render properly into `figure` in typst.

## Install

```bash
cargo install --path . --locked
```

## Usage

```bash
pandoc -f markdown-auto_identifiers-implicit_figures -t typst --filter pandoc-fenced-figure -o out.typ markdown.md
```