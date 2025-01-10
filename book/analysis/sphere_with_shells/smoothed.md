# Smoothed Mesh

```sh
alias automesh='~/autotwin/automesh/target/release/automesh'
```

```sh
cd ~/autotwin/automesh/book/analysis/sphere_with_shells
```

Smooth with various number of iterations:

```sh
automesh mesh \
--remove 0 \
--xscale 0.5 --yscale 0.5 --zscale 0.5 \
--xtranslate -12 --ytranslate -12 --ztranslate -12 \
--input spheres_resolution_2.npy \
--output sr2s10.exo \
smooth \
--hierarchical \
--iterations 10
```

```sh
automesh mesh \
--remove 0 \
--xscale 0.5 --yscale 0.5 --zscale 0.5 \
--xtranslate -12 --ytranslate -12 --ztranslate -12 \
--input spheres_resolution_2.npy \
--output sr2s50.exo \
smooth \
--hierarchical \
--iterations 50
```

```sh
automesh mesh \
--remove 0 \
--xscale 0.5 --yscale 0.5 --zscale 0.5 \
--xtranslate -12 --ytranslate -12 --ztranslate -12 \
--input spheres_resolution_2.npy \
--output sr2s200.exo \
smooth \
--hierarchical \
--iterations 200
```

Assess element quality to avoid oversmoothing:

```sh
automesh mesh \
--remove 0 \
--xscale 0.5 --yscale 0.5 --zscale 0.5 \
--xtranslate -12 --ytranslate -12 --ztranslate -12 \
--input spheres_resolution_2.npy \
--output sr2s10.inp \
smooth \
--hierarchical \
--iterations 10

automesh metrics \
--input sr2s10.inp \
--output sr2s10.csv \
```
