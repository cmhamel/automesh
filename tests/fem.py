from automesh import Voxels

remove = [0]
scale = [1, 1, 1]
translate = [0, 0, 0]
voxels = Voxels.from_npy('tests/input/letter_f_3d.npy')


def test_smooth_laplace():
    fem = voxels.as_finite_elements(remove, scale, translate)
    fem.smooth(method='Laplace')


def test_smooth_taubin():
    fem = voxels.as_finite_elements(remove, scale, translate)
    fem.smooth(method='Taubin')


def test_write_inp():
    fem = voxels.as_finite_elements(remove, scale, translate)
    inp = 'target/letter_f_3d.inp'
    fem.write_inp(inp)
    with open('tests/input/letter_f_3d.inp') as gold, open(inp) as file:
        for _ in range(2):
            assert gold.readline() == file.readline()
        gold.readline()
        assert file.readline()[:8] == "version "
        # not checking version number
        gold.readline()
        assert file.readline()[:17] == "autogenerated on "
        line = file.readline()
        while line != '':
            assert gold.readline() == line
            line = file.readline()


def test_write_inp_sparse():
    voxels = Voxels.from_spn('tests/input/sparse.spn', [5, 5, 5])
    fem = voxels.as_finite_elements(remove, scale, translate)
    inp = 'target/sparse.inp'
    fem.write_inp(inp)
    with open('tests/input/sparse.inp') as gold, open(inp) as file:
        for _ in range(2):
            assert gold.readline() == file.readline()
        gold.readline()
        assert file.readline()[:8] == "version "
        # not checking version number
        gold.readline()
        assert file.readline()[:17] == "autogenerated on "
        line = file.readline()
        while line != '':
            assert gold.readline() == line
            line = file.readline()
