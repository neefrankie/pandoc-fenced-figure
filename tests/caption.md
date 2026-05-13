# Caption container

## Pandoc Table

  Right     Left     Center     Default
-------     ------ ----------   -------
     12     12        12            12
    123     123       123          123
      1     1          1             1

Table:  Demonstration of simple table syntax.

## Pandoc Figure

![The caption.](image.png){alt="description of image"}

## My proposal

Code caption:

:::{.figure}

```python
print("Hello World!")
```

Caption: Marks a paragraph used as figcaption.
:::


With figure:

:::{.figure #my-figure caption="this is caption"}

![](/images/slp3/slp-fig-6-7.png)

Caption: Marks a paragraph used as figcaption.
:::

