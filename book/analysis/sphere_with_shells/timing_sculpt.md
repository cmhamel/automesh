# Timing - Sculpt

Set up.

```sh
alias sculpt='/Applications/Cubit-16.14/Cubit.app/Contents/MacOS/sculpt'
cd ~/autotwin/automesh/book/analysis/sphere_with_shells/
```

Use `automesh` to create `.spn` files from the `.npy` files.

```sh
automesh convert -i spheres_resolution_1.npy -o spheres_resolution_1.spn
automesh convert -i spheres_resolution_2.npy -o spheres_resolution_2.spn
automesh convert -i spheres_resolution_3.npy -o spheres_resolution_3.spn
automesh convert -i spheres_resolution_4.npy -o spheres_resolution_4.spn
```

```sh
    automesh 0.2.8
     Reading spheres_resolution_1.npy
        Done 143.541µs
     Writing spheres_resolution_1.spn
        Done 1.035459ms
       Total 1.325833ms
    automesh 0.2.8
     Reading spheres_resolution_2.npy
        Done 177.334µs
     Writing spheres_resolution_2.spn
        Done 4.078375ms
       Total 4.409791ms
    automesh 0.2.8
     Reading spheres_resolution_3.npy
        Done 259.833µs
     Writing spheres_resolution_3.spn
        Done 16.901542ms
       Total 17.292542ms
    automesh 0.2.8
     Reading spheres_resolution_4.npy
        Done 2.417333ms
     Writing spheres_resolution_4.spn
        Done 230.103333ms
       Total 232.658709ms
```

Run Sculpt.

```sh
sculpt --num_procs 1 --input_spn "spheres_resolution_1.spn" \
-x 24 -y 24 -z 24 \
--xtranslate -24 --ytranslate -24 --ztranslate -24 \
--spn_xyz_order 0 \
--exodus_file "spheres_resolution_1" \
--stair 1
```

```sh
Total Time on 1 Procs   1.084441 sec. (0.018074 min.)
```

```sh
sculpt --num_procs 1 --input_spn "spheres_resolution_2.spn" \
-x 48 -y 48 -z 48 \
--xscale 0.5 --yscale 0.5 --zscale 0.5 \
--xtranslate -12 --ytranslate -12 --ztranslate -12 \
--spn_xyz_order 0 \
--exodus_file "spheres_resolution_2" \
--stair 1
```

```sh
Total Time on 1 Procs   3.221756 sec. (0.053696 min.)
```

```sh
sculpt --num_procs 1 --input_spn "spheres_resolution_3.spn" \
-x 96 -y 96 -z 96 \
--xscale 0.25 --yscale 0.25 --zscale 0.25 \
--xtranslate -12 --ytranslate -12 --ztranslate -12 \
--spn_xyz_order 0 \
--exodus_file "spheres_resolution_3" \
--stair 1
```

```sh
Total Time on 1 Procs   24.635255 sec. (0.410588 min.)
```

```sh
sculpt --num_procs 1 --input_spn "spheres_resolution_4.spn" \
-x 240 -y 240 -z 240 \
--xscale 0.1 --yscale 0.1 --zscale 0.1 \
--xtranslate -12 --ytranslate -12 --ztranslate -12 \
--spn_xyz_order 0 \
--exodus_file "spheres_resolution_4" \
--stair 1
```

```sh
Total Time on 1 Procs   411.352895 sec. (6.855882 min.)
```
