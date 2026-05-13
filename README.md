# pandoc-caption

pandoc-caption is a [pandoc](https://pandoc.org/) filter that adds a caption to any block by making use of the Pandoc markdown's Div syntax:

```markdown
:::{.figure}

```python
print("Hello World!")
```

Caption: Marks a paragraph used as figcaption.
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

it should also render properly into `figure` in typst.

## Install

```bash
cargo install --path . --locked --release
```

## Usage

```bash
pandoc -f markdown -t html --filter pandoc-caption -o myfile.html markdown.md
```