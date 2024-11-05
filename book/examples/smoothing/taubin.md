# Taubin Smoothing

The Cubit and Python code used to generate the figures is included [below](#source).

iso | iso midplane | `xz` midplane
:---: | :---: | :---:
![sphere_10k.png](sphere_10k.png) | ![sphere_10k_iso_midplane.png](sphere_10k_iso_midplane.png) | ![sphere_10k_xz_midplane.png](sphere_10k_xz_midplane.png)
![sphere_10k_noised.png](sphere_10k_noised.png) | ![sphere_10k_iso_midplane_noised.png](sphere_10k_iso_midplane_noised.png) | ![sphere_10k_xz_midplane_noised.png](sphere_10k_xz_midplane_noised.png)

## automesh

```sh
alias automesh='/Users/chovey/autotwin/automesh/target/release/automesh'
```

```sh
cd ~/autotwin/automesh/book/examples/smoothing/
```

```sh
automesh smooth -i sphere_res_1cm_noised.inp -o s10.exo -n 10
```

## Taubin paper example

![sphere_surface_w_noise.png](sphere_surface_w_noise.png)

## Source

### `sphere.jou`

```sh
<!-- cmdrun cat sphere.jou -->
```

### `noise_augmentation.py`

```python
<!-- cmdrun cat noise_augmentation.py -->
```
