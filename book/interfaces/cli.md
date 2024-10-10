# Command Line Interface

```sh
automesh --help
<!-- cmdrun automesh --help -->
```

## Example

Convert a Numpy segmentation file to an Abaqus input file:

```sh
automesh mesh --input single.npy --output single.inp
```

The terminal output:

<!-- cmdrun wget https://github.com/autotwin/automesh/raw/main/tests/input/single.npy -O ../../target/single_for_cli.npy -->
<!-- cmdrun automesh mesh --input ../../target/single_for_cli.npy --output ../../target/single_from_cli.inp -q -->
<!-- cmdrun grep version ../../target/single_from_cli.inp | cut -d ' ' -f 2 > ../../target/version.txt -->

<pre><code class="language-sh hljs bash"><font color="#EC00FF">    <b>automesh {{#include ../../target/version.txt}}</b></font>
     <font color="#58C7E2"><b>Reading</b></font> single.npy
        <font color="#54E484"><b>Done</b></font> 141.917µs
     <font color="#58C7E2"><b>Meshing</b></font> single.inp
        <font color="#54E484"><b>Done</b></font> 155.628µs
     <font color="#58C7E2"><b>Writing</b></font> single.inp
        <font color="#54E484"><b>Done</b></font> 150.88µs
</code></pre>

The resulting Abaqus input file:

```sh
{{#include ../../target/single_from_cli.inp}}
```
