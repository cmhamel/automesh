# Metrics

```sh
automesh metrics --help
<!-- cmdrun automesh metrics --help -->
```

## Input/Output File Types

```sh
inp -> csv
```

### Maximum Aspect Ratio

### Minimum Scaled Jacobian

### Maximum Skew

## Unit Tests

Inspired by Livesu *et al.*[^Livesu_2021], Figure 2, reproduced here below:

![](img/Livesu_Fig_2.png)

valence | singleton | MAR | MSJ | skew
:---: | :---: | :---: | :---: | :---:
3 | ![](img/single_valence_03.png) | 1.0 (9.999997e-1) | 0.866 (1.0) | 0.5 (0.0)
3n1 | ![](img/single_valence_03_noise1.png) | 1.29 (7.738382e-1) | 0.192 (4.088211e-1) | 0.680 (6.294436e-2)
4 | ![](img/single_valence_04.png) | 1.0 (1.0) | 1.0 (1.0) | 0.0 (0.0)
4n2 | ![](img/single_valence_04_noise2.png) | 1.17 (8.562497e-1) | 0.374 (3.785657e-1) | 0.486 (3.992235e-2)
5 | ![](img/single_valence_05.png) | 1.0 (9.999995e-1) | 0.951 (1.0) | 0.309 (0.0)
6 | ![](img/single_valence_06.png) | 1.0 (9.999997e-1) | 0.866 (1.0) | 0.5 (0.0)
... | ... | ... | ... | ...
10 | ![](img/single_valence_10.png) | 1.0 (9.999999e-1) | 0.588 (1.0) | 0.809 (0.0)

Figure: Maximum aspect ratio (MAR), minimum scaled Jacobian (MSJ), and maximum skew.  Values in (xx) are as repoted in AM as of 2025-01-14-1621-EST.  Values preceeding the (xx) values are from Cubit.

## References

[^Livesu_2021]: Livesu M, Pitzalis L, Cherchi G. Optimal dual schemes for adaptive grid based hexmeshing. ACM Transactions on Graphics (TOG). 2021 Dec 6;41(2):1-4. [link](https://dl.acm.org/doi/pdf/10.1145/3494456)
